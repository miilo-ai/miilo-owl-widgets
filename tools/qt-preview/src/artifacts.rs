use crate::Result;
use serde_json::Value;
use std::{fs, io::Read, path::Path};

fn names(config: &Value) -> Result<Vec<String>> {
    let mut names: Vec<String> = match config["mode"].as_str() {
        Some("crew") => [
            "crew-dark.png",
            "crew-expanded.png",
            "crew-light-narrow.png",
        ],
        Some("orbit") => ["orbit.png", "orbit-next-frame.png", "orbit-detail.png"],
        _ => return Err("Unknown artifact mode".into()),
    }
    .into_iter()
    .map(String::from)
    .collect();
    if config["mode"] == "orbit" && config["record"] == true {
        let seconds = config["seconds"].as_u64().ok_or("Missing frame count")?;
        names.extend((0..=seconds * 12).map(|n| format!("frames/{n:04}.png")));
    }
    Ok(names)
}

pub fn publish(config: &Value, source: &Path, output: &Path) -> Result<()> {
    let names = names(config)?;
    reject_stale_frames(config, output, &names)?;
    for name in &names {
        let mut header = [0_u8; 24];
        fs::File::open(source.join(name))?.read_exact(&mut header)?;
        if &header[..8] != b"\x89PNG\r\n\x1a\n"
            || &header[12..16] != b"IHDR"
            || header[16..20] == [0; 4]
            || header[20..24] == [0; 4]
        {
            return Err(format!("Invalid PNG artifact: {name}").into());
        }
    }
    for name in &names {
        let destination = output.join(name);
        fs::create_dir_all(destination.parent().ok_or("No artifact parent")?)?;
        fs::copy(source.join(name), destination)?;
    }
    Ok(())
}

fn reject_stale_frames(config: &Value, output: &Path, expected: &[String]) -> Result<()> {
    if config["mode"] != "orbit" || config["record"] != true || !output.join("frames").exists() {
        return Ok(());
    }
    for entry in fs::read_dir(output.join("frames"))? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        let numbered = name.len() == 8
            && name.ends_with(".png")
            && name.as_bytes()[..4].iter().all(u8::is_ascii_digit);
        if numbered && !expected.contains(&format!("frames/{name}")) {
            return Err(
                "Output contains stale recording frames; select an empty output directory".into(),
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{fs, names, publish};
    use serde_json::json;
    #[test]
    fn shorter_recording_refuses_stale_frames_before_publishing() {
        let directory = tempfile::tempdir().unwrap();
        let frames = directory.path().join("frames");
        fs::create_dir(&frames).unwrap();
        fs::write(frames.join("0240.png"), b"previous recording").unwrap();
        let config = json!({"mode":"orbit", "record":true, "seconds":2});
        let error = publish(&config, directory.path(), directory.path()).unwrap_err();
        assert!(error.to_string().contains("stale recording frames"));
        assert_eq!(
            fs::read(frames.join("0240.png")).unwrap(),
            b"previous recording"
        );
    }
    #[test]
    fn recording_preserves_endpoint_frame() {
        let config = json!({"mode":"orbit", "record":true, "seconds":2});
        let names = names(&config).unwrap();
        assert_eq!(names.len(), 28);
        assert_eq!(names.last().unwrap(), "frames/0024.png");
    }
    #[test]
    fn missing_or_invalid_captures_cannot_publish() {
        let source = tempfile::tempdir().unwrap();
        let output = source.path().join("published");
        let config = json!({"mode":"crew"});
        assert!(publish(&config, source.path(), &output).is_err());
        fs::write(source.path().join("crew-dark.png"), "not a PNG").unwrap();
        assert!(publish(&config, source.path(), &output).is_err());
        assert!(!output.exists());
    }
}
