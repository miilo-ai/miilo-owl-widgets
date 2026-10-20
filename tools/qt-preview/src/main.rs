mod artifacts;
mod config;
mod runner;

use std::{env, fs, path::PathBuf};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut config = config::prepare(&args, |key| env::var(key).ok())?;
    let scratch = tempfile::Builder::new()
        .prefix("miilo-qml-preview-")
        .tempdir()?;
    let output = PathBuf::from(config["output"].as_str().ok_or("Missing output path")?);
    let captures = scratch.path().join("captures");
    fs::create_dir_all(&captures)?;
    config["output"] = serde_json::json!(captures.to_str().ok_or("Non-UTF8 capture path")?);
    fs::write(
        scratch.path().join("PreviewConfig.js"),
        format!(".pragma library\nvar settings = {};\n", config),
    )?;
    let templates = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("qml");
    for file in ["tst_preview.qml", "Capture.js", "Orbit.js", "Crew.js"] {
        fs::copy(templates.join(file), scratch.path().join(file))?;
    }
    if config["record"] == true {
        fs::create_dir_all(captures.join("frames"))?;
    }
    let executable = env::var_os("MIILO_QMLTESTRUNNER").unwrap_or_else(|| "qmltestrunner".into());
    let mut command = std::process::Command::new(executable);
    command
        .args(["-input"])
        .arg(scratch.path().join("tst_preview.qml"))
        .args(["-import"])
        .arg(PathBuf::from(&args[3]).join("tests/qml/stubs"));
    for (key, value) in [
        ("QT_QPA_PLATFORM", "offscreen"),
        ("QT_QUICK_BACKEND", "software"),
    ] {
        if env::var_os(key).is_none() {
            command.env(key, value);
        }
    }
    command.env("QT_QUICK_CONTROLS_STYLE", "Basic");
    runner::run_bounded(&mut command, std::time::Duration::from_secs(90))?;
    artifacts::publish(&config, &captures, &output)?;
    println!(
        "PASS: isolated QML fixture captures and interactions; no shell or device qualification."
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("QML preview failed: {error}");
        std::process::exit(1);
    }
}
