use serde_json::Value;
use std::{fs, os::unix::fs::PermissionsExt, path::Path, process::Command};
fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}
#[test]
fn android_install_preserves_config_backups_and_paths_with_spaces() {
    let temp = tempfile::Builder::new()
        .prefix("miilo install ")
        .tempdir()
        .unwrap();
    let base = temp.path().canonicalize().unwrap();
    let local = base.join("local");
    let widgets = base.join("widgets");
    let config = base.join("config");
    fs::create_dir_all(&widgets).unwrap();
    fs::create_dir_all(&config).unwrap();
    for (name, text) in [
        ("70-miilo-fleet.owl", "old widget"),
        ("71-miilo-attention.owl", "old attention"),
        ("72-miilo-board.owl", "old board"),
    ] {
        fs::write(widgets.join(name), text).unwrap();
    }
    let original = r#"{"version":1,"endpoints":[]}"#;
    fs::write(config.join("config.json"), original).unwrap();
    fs::set_permissions(
        config.join("config.json"),
        fs::Permissions::from_mode(0o600),
    )
    .unwrap();
    for _ in 0..2 {
        let output = Command::new("bash")
            .arg(root().join("install.sh"))
            .arg("--no-setup")
            .env("MIILO_OWL_BINARY", env!("CARGO_BIN_EXE_miilo-owl"))
            .env("MIILO_OWL_INSTALL_ROOT", &local)
            .env("MIILO_OWL_WIDGET_DIR", &widgets)
            .env("MIILO_OWL_CONFIG_DIR", &config)
            .env("XDG_STATE_HOME", base.join("state"))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let snapshot = Command::new(local.join("bin/miilo-owl"))
        .arg("snapshot")
        .env("MIILO_OWL_CONFIG_DIR", &config)
        .env("XDG_STATE_HOME", base.join("state"))
        .output()
        .unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&snapshot.stdout).unwrap()["configured"],
        false
    );
    assert_eq!(
        fs::read(widgets.join("70-miilo-fleet.owl")).unwrap(),
        fs::read(root().join("70-miilo-fleet.owl")).unwrap()
    );
    assert_eq!(
        fs::read_to_string(widgets.join("70-miilo-fleet.owl.before-miilo-1.3")).unwrap(),
        "old widget"
    );
    assert_eq!(
        fs::read_to_string(widgets.join("71-miilo-attention.owl.disabled")).unwrap(),
        "old attention"
    );
    assert_eq!(
        fs::read_to_string(widgets.join("72-miilo-board.owl.disabled")).unwrap(),
        "old board"
    );
    assert_eq!(
        fs::read_to_string(config.join("config.json")).unwrap(),
        original
    );
}
#[test]
fn missing_android_binary_does_not_create_widgets() {
    let temp = tempfile::tempdir().unwrap();
    let widgets = temp.path().join("widgets");
    let out = Command::new("bash")
        .arg(root().join("install.sh"))
        .arg("--no-setup")
        .env("MIILO_OWL_BINARY", "/nonexistent/miilo-owl")
        .env("MIILO_OWL_INSTALL_ROOT", temp.path().join("local"))
        .env("MIILO_OWL_WIDGET_DIR", &widgets)
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(!widgets.exists());
}
