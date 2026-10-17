mod fixtures;
mod portrait;
mod release;
mod validation;
use serde_json::{Value, json};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned()
}
fn output(command: &mut Command) -> Result<String> {
    let out = command.output()?;
    if !out.status.success() {
        return Err(format!("Command failed: {}", String::from_utf8_lossy(&out.stderr)).into());
    }
    Ok(String::from_utf8(out.stdout)?)
}
fn assemble() -> Result<()> {
    portrait::assemble()?;
    let root = root();
    let manifest = fs::read_to_string(root.join("helper/Cargo.toml"))?;
    let version = manifest
        .lines()
        .find_map(|line| {
            line.strip_prefix("version = \"")
                .and_then(|v| v.strip_suffix('"'))
        })
        .ok_or("Missing version")?;
    for (index, name, internal, description) in [
        (
            70,
            "feed",
            "fleet",
            "Animated agents and work across selected systems",
        ),
        (
            71,
            "attention",
            "attention",
            "Compact review queue with evidence and provider actions",
        ),
        (
            72,
            "board",
            "board",
            "Tasks moving through selected factories",
        ),
    ] {
        let title = if index == 70 {
            "miilo".into()
        } else {
            format!("miilo {internal}")
        };
        let mut text = format!(
            "widget \"{title}\" {{\n  meta {{ owl: 6; description: \"{description}\"; version: \"{version}\"; license: \"MIT\"; category: \"dev\" }}\n  page: 1\n  order: {}\n  size: 3x2\n  minSize: 3x2\n  padding: 8\n",
            index - 63
        );
        let mut parts = vec![
            "appearance",
            "common",
            "progress",
            "artifacts",
            "work-detail",
            "discovery",
        ];
        if index != 72 {
            parts.extend(["robot", "colors"]);
        }
        if index == 70 {
            parts.extend(["orbit-geometry", "orbit-status", "orbit", "quiet"]);
        }
        parts.push(name);
        let mut fragments = vec![];
        for part in parts {
            let source = fs::read_to_string(root.join(format!("widgets/{part}.inc")))?;
            fragments.push(source);
        }
        text.push_str(&fragments.join("\n"));
        text.push_str("}\n");
        fs::write(root.join(format!("{index}-miilo-{internal}.owl")), text)?;
    }
    fs::copy(
        root.join("70-miilo-fleet.owl"),
        root.join("desktop/70-miilo-fleet.owl"),
    )?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("assemble")=>assemble()?,
        Some("header-variant")=>{let destination=args.get(1).ok_or("Provide an output path")?;let mut source=fs::read_to_string(root().join("70-miilo-fleet.owl"))?;if args.iter().any(|a|a=="--preview-robots"){source=source.replace("state previewRobots = false","state previewRobots = true");}fs::write(destination,source)?;},
        Some("render-reference")=>{let source=Path::new(args.get(1).ok_or("Provide a tree directory")?);let destination=args.get(2).ok_or("Provide an output path")?;let mut items=vec![];for theme in ["dark","light"]{for name in ["70-miilo-fleet","71-miilo-attention","72-miilo-board"]{let data:Value=serde_json::from_str(&fs::read_to_string(source.join(format!("{name}-{theme}.json")))?)?;items.push(json!({"name":name,"theme":theme,"tile":data["tile"],"screen":data["screen"],"width":data["width"],"height":data["height"]}));}}fs::write(destination,fs::read_to_string(root().join("tools/reference.html"))?.replace("/*DATA*/",&serde_json::to_string(&items)?))?;},
        Some("check-desktop")=>{let folder=tempfile::tempdir()?;fixtures::write(folder.path())?;print!("{}",output(Command::new("node").arg(root().join("tools/test-desktop.cjs")).arg(folder.path()))?);},
        Some("check-owl")=>validation::check()?,
        Some("release")=>release::run(&args[1..])?,
        _=>return Err("Usage: cargo xtask assemble | check-desktop | check-owl | header-variant OUTPUT [--preview-robots] | render-reference TREES OUTPUT".into()),
    }
    Ok(())
}
