use crate::Result;
use serde_json::{Value, json};
use std::{fs, path::Path};

pub fn file_url(path: &Path) -> Result<String> {
    let absolute = fs::canonicalize(path)?;
    let mut url = String::from("file://");
    for byte in absolute.to_str().ok_or("Non-UTF8 fixture path")?.bytes() {
        if byte.is_ascii_alphanumeric() || b"/-._~:".contains(&byte) {
            url.push(byte as char);
        } else {
            url.push_str(&format!("%{byte:02X}"));
        }
    }
    Ok(url)
}

fn number(
    env: &impl Fn(&str) -> Option<String>,
    key: &str,
    default: u32,
    min: u32,
    max: u32,
) -> Result<u32> {
    let value = env(key)
        .map(|s| s.parse::<u32>())
        .transpose()?
        .unwrap_or(default);
    if !(min..=max).contains(&value) {
        return Err(format!("{key} must be {min}..{max}").into());
    }
    Ok(value)
}

pub fn prepare(args: &[String], env: impl Fn(&str) -> Option<String>) -> Result<Value> {
    if args.len() != 6 || !["orbit", "crew"].contains(&args[0].as_str()) {
        return Err("Usage: miilo-qt-preview MODE FIXTURE OUTPUT ROOT DATA SOURCE".into());
    }
    let view = env("MIILO_PREVIEW_VIEW").unwrap_or_else(|| "orbit".into());
    let motion = env("MIILO_PREVIEW_MOTION").unwrap_or_else(|| "full".into());
    if !["orbit", "inspect", "quiet", "quiet-expanded"].contains(&view.as_str()) {
        return Err("Invalid preview view".into());
    }
    if !["full", "reduced", "off"].contains(&motion.as_str()) {
        return Err("Invalid preview motion".into());
    }
    let mut properties = json!({});
    if args[0] == "orbit" {
        let data: Value = serde_json::from_str(&fs::read_to_string(&args[4])?)?;
        if !data.is_object() {
            return Err("Fixture JSON must be an object".into());
        }
        properties = json!({
            "widgetSource": fs::read_to_string(&args[5])?, "sceneData": data,
            "sceneTheme": env("MIILO_PREVIEW_PALETTE").map(fs::read_to_string).transpose()?.unwrap_or_default(),
            "sceneView": view, "sceneMotion": motion,
            "sceneLight": env("MIILO_PREVIEW_THEME").as_deref() == Some("light"),
            "width": number(&env, "MIILO_PREVIEW_WIDTH", 900, 100, 4096)?,
            "height": number(&env, "MIILO_PREVIEW_HEIGHT", 850, 100, 4096)?
        });
    }
    let output = std::path::absolute(&args[2])?;
    Ok(json!({
        "mode": args[0], "fixture": file_url(Path::new(&args[1]))?,
        "output": output.to_str().ok_or("Non-UTF8 output path")?,
        "properties": properties, "view": view, "motion": motion,
        "record": env("MIILO_PREVIEW_RECORD").as_deref() == Some("1"),
        "seconds": number(&env, "MIILO_PREVIEW_SECONDS", 20, 2, 20)?
    }))
}

#[cfg(test)]
mod tests {
    use super::{file_url, number, prepare};
    use std::fs;
    #[test]
    fn bounds_and_invalid_values_fail() {
        assert_eq!(number(&|_| None, "size", 900, 100, 4096).unwrap(), 900);
        for value in ["99", "4097", "no", "-1"] {
            assert!(number(&|_| Some(value.into()), "size", 900, 100, 4096).is_err());
        }
    }
    #[test]
    fn url_encodes_special_paths() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fixture #?é.qml");
        fs::write(&path, "").unwrap();
        assert!(
            file_url(&path)
                .unwrap()
                .ends_with("fixture%20%23%3F%C3%A9.qml")
        );
    }
    #[test]
    fn malformed_invocation_fails() {
        assert!(prepare(&[], |_| None).is_err());
        assert!(prepare(&vec!["unknown".into(); 6], |_| None).is_err());
    }
}
