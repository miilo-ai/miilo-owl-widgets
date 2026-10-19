//! Manifest-driven release policy. Missing credentials skip publication; errors fail closed.
use crate::{Result, output, root};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};
fn setting(name: &str) -> String {
    env::var(name).unwrap_or_default()
}
fn append(name: &str, text: &str) -> Result<()> {
    if let Some(path) = env::var_os(name) {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
            .write_all(text.as_bytes())?;
    }
    Ok(())
}
fn version(root: &Path) -> Result<semver::Version> {
    let path = if root.join("helper/Cargo.toml").exists() {
        root.join("helper/Cargo.toml")
    } else {
        root.join("Cargo.toml")
    };
    let text = fs::read_to_string(path)?;
    let value = text
        .lines()
        .find_map(|l| {
            let (k, v) = l.split_once('=')?;
            if k.trim() == "version" {
                Some(v.trim().trim_matches('"'))
            } else {
                None
            }
        })
        .ok_or("Package version missing")?;
    Ok(semver::Version::parse(value)?)
}
fn candidate(root: &Path, tags: &[String], reference: &str) -> Result<Value> {
    let version = version(root)?;
    let tag = format!("v{version}");
    if !reference.is_empty() && reference.trim_start_matches("refs/tags/") != tag {
        return Err("Release tag does not match package version".into());
    }
    if reference.is_empty() {
        for old in tags
            .iter()
            .filter_map(|t| t.strip_prefix('v'))
            .filter_map(|t| semver::Version::parse(t).ok())
        {
            if version.cmp_precedence(&old).is_lt() {
                return Err("Package version is older than an existing version tag".into());
            }
        }
    }
    Ok(
        json!({"version":version.to_string(),"tag":tag,"new_version":(!tags.contains(&tag)).to_string(),"prerelease":(!version.pre.is_empty()).to_string()}),
    )
}
fn permitted(values: &BTreeMap<String, String>) -> bool {
    let get = |k: &str| values.get(k).map(String::as_str).unwrap_or("");
    (get("MIILO_RELEASE_ENABLED") == "true" || get("MIILO_PUBLISH") == "true")
        && get("GITHUB_EVENT_NAME") != "pull_request"
        && !get("RELEASE_TOKEN").trim().is_empty()
        && values.get("DEPENDENCIES_READY").is_none_or(|v| v == "true")
}
fn http() -> Result<reqwest::blocking::Client> {
    Ok(reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(30))
        .user_agent("miilo-release-check")
        .build()?)
}
fn exists_status(status: u16) -> Result<bool> {
    match status {
        200 => Ok(true),
        404 => Ok(false),
        _ => Err("Release lookup failed; publication is not authorized".into()),
    }
}
fn option(args: &[String], key: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| default.into())
}
fn render_aur(args: &[String]) -> Result<()> {
    let tag = option(args, "--tag", "");
    let repository = option(args, "--repo", "");
    let kind = option(args, "--kind", "");
    let parsed = semver::Version::parse(tag.strip_prefix('v').ok_or("Canonical tag required")?)?;
    if !parsed.pre.is_empty() || !parsed.build.is_empty() {
        return Err("AUR requires a stable version".into());
    }
    let segments: Vec<_> = repository.split('/').collect();
    if segments.len() != 2
        || segments.iter().any(|s| {
            s.is_empty()
                || !s
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "_.-".contains(c))
        })
    {
        return Err("Invalid repository".into());
    }
    let binary = match kind.as_str() {
        "cli" => "miilo",
        "ce" => "miilo-ce",
        "owl" => "miilo-owl",
        _ => return Err("Unknown AUR kind".into()),
    };
    let assets = PathBuf::from(option(args, "--assets", "release-assets"));
    let destination = PathBuf::from(option(args, "--output", "aur"));
    let files: Vec<_> = fs::read_dir(&assets)?
        .collect::<std::io::Result<Vec<_>>>()?
        .into_iter()
        .map(|e| e.path())
        .collect();
    let mut archives = vec![];
    for (arch, target) in [
        ("x86_64", "x86_64-unknown-linux-gnu"),
        ("aarch64", "aarch64-unknown-linux-gnu"),
    ] {
        if kind == "owl" && arch != "aarch64" {
            continue;
        }
        let matches: Vec<_> = files
            .iter()
            .filter(|p| {
                let name = p.file_name().unwrap().to_string_lossy();
                if kind == "owl" {
                    name == format!("miilo-owl-widgets-{parsed}-linux-aarch64.tar.gz")
                } else {
                    name.ends_with(&format!("{target}.tar.xz"))
                }
            })
            .collect();
        if matches.len() != 1 {
            return Err(format!("Expected one archive for {arch}").into());
        }
        let path = matches[0];
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let digest = format!("{:x}", Sha256::digest(fs::read(path)?));
        let mut expected = vec![];
        for manifest in &files {
            let filename = manifest.file_name().unwrap().to_string_lossy();
            if filename.ends_with(".sha256") || filename.contains("SHA256SUMS") {
                for line in fs::read_to_string(manifest)?.lines() {
                    let fields: Vec<_> = line.split_whitespace().collect();
                    if fields.len() == 2 && fields[1].trim_start_matches(['*', '.', '/']) == name {
                        expected.push(fields[0].to_ascii_lowercase());
                    }
                }
            }
        }
        if expected.is_empty() || expected.iter().any(|e| e != &digest) {
            return Err(format!("Published checksum missing or inconsistent for {name}").into());
        }
        archives.push((arch, name, digest));
    }
    let package = format!("{binary}-bin");
    let license = if kind == "owl" { "MIT" } else { "Apache-2.0" };
    let architectures = archives
        .iter()
        .map(|(a, _, _)| format!("'{a}'"))
        .collect::<Vec<_>>()
        .join(" ");
    let mut text = format!(
        "pkgname={package}\npkgver={parsed}\npkgrel=1\npkgdesc='Miilo private-by-default observation tooling'\narch=({architectures})\nurl='https://github.com/{repository}'\nlicense=('{license}')\nprovides=('{binary}')\nconflicts=('{binary}')\n"
    );
    for (arch, name, digest) in archives {
        text.push_str(&format!("source_{arch}=(\"{name}::${{url}}/releases/download/v${{pkgver}}/{name}\")\nsha256sums_{arch}=('{digest}')\n"));
    }
    let source = if kind == "owl" {
        "bin/linux-aarch64/miilo-owl".into()
    } else {
        format!(
            "${{srcdir}}/{}-${{CARCH}}-unknown-linux-gnu/{binary}",
            if kind == "cli" {
                "miilo-cli"
            } else {
                "miilo-ce"
            }
        )
    };
    let licensefile = if kind == "owl" {
        "LICENSE".into()
    } else {
        format!(
            "${{srcdir}}/{}-${{CARCH}}-unknown-linux-gnu/LICENSE",
            if kind == "cli" {
                "miilo-cli"
            } else {
                "miilo-ce"
            }
        )
    };
    text.push_str(&format!("\npackage() {{\n  install -Dm755 \"{source}\" \"$pkgdir/usr/bin/{binary}\"\n  install -Dm644 \"{licensefile}\" \"$pkgdir/usr/share/licenses/$pkgname/LICENSE\"\n}}\n"));
    if kind == "cli" {
        text=text.replace("  install -Dm644", "  install -Dm755 \"${srcdir}/miilo-cli-${CARCH}-unknown-linux-gnu/miilo-local\" \"$pkgdir/usr/bin/miilo-local\"\n  install -Dm644");
    }
    fs::create_dir_all(&destination)?;
    fs::write(destination.join("PKGBUILD"), text)?;
    println!("Rendered {package} {tag}");
    Ok(())
}
fn lint() -> Result<()> {
    let (name, digest) = match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => (
            "linux_amd64",
            "8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8",
        ),
        ("macos", "aarch64") => (
            "darwin_arm64",
            "aba9ced2dee8d27fecca3dc7feb1a7f9a52caefa1eb46f3271ea66b6e0e6953f",
        ),
        _ => return Err("Workflow lint supports Linux x86_64 or Apple Silicon".into()),
    };
    let folder = tempfile::tempdir()?;
    let archive = folder.path().join("actionlint.tar.gz");
    let url = format!(
        "https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_{name}.tar.gz"
    );
    // curl follows GitHub release-asset redirects; no credential is attached.
    output(
        Command::new("curl")
            .args([
                "--fail",
                "--silent",
                "--show-error",
                "--location",
                "--max-time",
                "30",
                &url,
                "-o",
            ])
            .arg(&archive),
    )?;
    if format!("{:x}", Sha256::digest(fs::read(&archive)?)) != digest {
        return Err("Actionlint digest mismatch".into());
    }
    output(
        Command::new("tar")
            .args(["-xzf"])
            .arg(&archive)
            .arg("-C")
            .arg(folder.path())
            .arg("actionlint"),
    )?;
    let mut command = Command::new(folder.path().join("actionlint"));
    command.arg("-shellcheck=").arg("-pyflakes=");
    for entry in fs::read_dir(root().join(".github/workflows"))? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "yml") {
            command.arg(path);
        }
    }
    print!("{}", output(&mut command)?);
    Ok(())
}
pub fn run(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("version") => {
            let directory = PathBuf::from(option(args, "--root", &root().to_string_lossy()));
            let tags = output(
                Command::new("git")
                    .args(["tag", "--list"])
                    .current_dir(&directory),
            )?
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>();
            let value = candidate(&directory, &tags, &option(args, "--tag", ""))?;
            println!("{value}");
            for (k, v) in value.as_object().unwrap() {
                append("GITHUB_OUTPUT", &format!("{k}={}\n", v.as_str().unwrap()))?;
            }
        }
        Some("publication") => {
            let allowed = permitted(&env::vars().collect());
            println!(
                "{}",
                if allowed {
                    "Publication authorized for this workflow"
                } else {
                    "No release has been made"
                }
            );
            append("GITHUB_OUTPUT", &format!("publish={allowed}\n"))?;
            if !allowed {
                append("GITHUB_STEP_SUMMARY", "No release has been made\n")?;
            }
        }
        Some("release-exists") => {
            let tag = setting("RELEASE_TAG");
            let token = setting("RELEASE_TOKEN");
            let exists = if token.is_empty() || tag.is_empty() {
                false
            } else {
                let mut url = url_for_release(&setting("GITHUB_REPOSITORY"), &tag)?;
                url.set_query(None);
                let response = http()?
                    .get(url)
                    .bearer_auth(token)
                    .header("Accept", "application/vnd.github+json")
                    .send()
                    .map_err(|_| "Release lookup failed; publication is not authorized")?;
                exists_status(response.status().as_u16())?
            };
            append("GITHUB_OUTPUT", &format!("exists={exists}\n"))?;
        }
        Some("render-aur") => render_aur(args)?,
        Some("lint-workflows") => lint()?,
        _ => return Err("Unknown release tool".into()),
    }
    Ok(())
}
fn url_for_release(repository: &str, tag: &str) -> Result<reqwest::Url> {
    let mut url = reqwest::Url::parse("https://api.github.com/repos/")?;
    {
        let mut segments = url.path_segments_mut().map_err(|_| "Invalid GitHub URL")?;
        segments.pop_if_empty();
        for part in repository.split('/') {
            segments.push(part);
        }
        segments.extend(["releases", "tags", tag]);
    }
    Ok(url)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semver_precedence_and_invalid_input() {
        let parse = |v: &str| semver::Version::parse(v).unwrap();
        assert!(
            parse("1.0.0-rc.2")
                .cmp_precedence(&parse("1.0.0-rc.10"))
                .is_lt()
        );
        assert!(parse("1.0.0-rc.10").cmp_precedence(&parse("1.0.0")).is_lt());
        assert!(
            parse("1.0.0+one")
                .cmp_precedence(&parse("1.0.0+two"))
                .is_eq()
        );
        for value in ["1.2", "01.2.3", "1.2.3-01", "v1.2.3", "1.2.3\nwrite=bad"] {
            assert!(semver::Version::parse(value).is_err());
        }
    }
    #[test]
    fn version_advance_existing_tag_downgrade_and_mismatched_ref() {
        let directory = tempfile::tempdir().unwrap();
        fs::write(
            directory.path().join("Cargo.toml"),
            "[workspace.package]\nversion=\"1.2.3\"\n",
        )
        .unwrap();
        assert_eq!(
            candidate(directory.path(), &["v1.2.2".into()], "").unwrap()["new_version"],
            "true"
        );
        assert_eq!(
            candidate(directory.path(), &["v1.2.3".into()], "").unwrap()["new_version"],
            "false"
        );
        assert!(candidate(directory.path(), &["v2.0.0".into()], "").is_err());
        assert!(candidate(directory.path(), &[], "v9.0.0").is_err());
    }
    #[test]
    fn publication_requires_intent_token_and_dependencies() {
        let valid = BTreeMap::from([
            ("MIILO_PUBLISH".into(), "true".into()),
            ("RELEASE_TOKEN".into(), "private-test-token".into()),
        ]);
        assert!(permitted(&valid));
        for (k, v) in [
            ("RELEASE_TOKEN", ""),
            ("DEPENDENCIES_READY", "false"),
            ("GITHUB_EVENT_NAME", "pull_request"),
            ("MIILO_PUBLISH", "false"),
        ] {
            let mut values = valid.clone();
            values.insert(k.into(), v.into());
            assert!(!permitted(&values));
        }
    }
    #[test]
    fn existing_release_is_noop_and_authorization_errors_fail_closed() {
        assert!(exists_status(200).unwrap());
        assert!(!exists_status(404).unwrap());
        assert!(exists_status(401).is_err());
        assert!(exists_status(500).is_err());
    }
    #[test]
    fn publication_graph_and_pinned_actions() {
        fn workflow(name: &str) -> serde_yaml_ng::Value {
            serde_yaml_ng::from_str(
                &fs::read_to_string(root().join(format!(".github/workflows/{name}"))).unwrap(),
            )
            .unwrap()
        }
        let release = workflow("release.yml");
        let auto = workflow("version.yml");
        let policy = &auto["jobs"]["candidate"]["steps"];
        let policy_text = format!("{policy:?}");
        assert!(policy_text.contains("github.event_name == 'push'"));
        assert!(policy_text.contains("refs/heads/main"));
        assert!(policy_text.contains("MIILO_RELEASE_ENABLED != 'false'"));
        let jobs = &release["jobs"];
        assert!(format!("{:?}", jobs["publication-gate"]["needs"]).contains("qualify"));
        assert!(format!("{:?}", jobs["build"]["needs"]).contains("publication-gate"));
        {
            let name = "build";
            assert!(
                jobs[name]["if"]
                    .as_str()
                    .unwrap()
                    .contains("outputs.publish == 'true'")
            );
        }
        assert!(format!("{:?}", jobs["publish"]["needs"]).contains("build"));
        assert!(format!("{:?}", jobs["no-release"]).contains("No release has been made"));
        let version = workflow("version.yml");
        let tag = &version["jobs"]["tag"];
        assert!(format!("{:?}", tag["needs"]).contains("qualify"));
        assert!(
            tag["if"]
                .as_str()
                .unwrap()
                .contains("outputs.ready == 'true'")
        );
        let steps = tag["steps"].as_sequence().unwrap();
        let script = steps.last().unwrap()["run"].as_str().unwrap();
        assert!(script.contains("gh workflow run release.yml"));
        assert!(!script.contains("--force") && !script.contains("git tag -f"));
        for entry in fs::read_dir(root().join(".github/workflows")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|e| e != "yml") {
                continue;
            }
            let data = workflow(path.file_name().unwrap().to_str().unwrap());
            assert!(!format!("{:?}", data["on"]).contains("pull_request_target"));
            for job in data["jobs"].as_mapping().unwrap().values() {
                for step in job["steps"].as_sequence().into_iter().flatten() {
                    if let Some(uses) = step["uses"].as_str()
                        && let Some((_, reference)) = uses.rsplit_once('@')
                    {
                        assert_eq!(reference.len(), 40);
                        assert!(reference.chars().all(|c| c.is_ascii_hexdigit()));
                    }
                }
            }
        }
    }
}
