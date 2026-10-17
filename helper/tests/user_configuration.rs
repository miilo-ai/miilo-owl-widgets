use std::{fs, os::unix::fs::PermissionsExt, path::Path, process::Command};

fn private(path: &Path, data: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, data).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
}
fn command(home: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_miilo-owl"));
    command
        .env("HOME", home)
        .env_remove("MIILO_HOME")
        .env_remove("MIILO_OWL_CONFIG_DIR")
        .arg("snapshot");
    command
}

#[test]
fn root_manifest_selects_shared_store_and_explicit_override_still_wins() {
    let scratch = tempfile::tempdir().unwrap();
    let home = scratch.path();
    private(
        &home.join(".config/miilo-owl/config.json"),
        r#"{"version":1,"endpoints":[]}"#,
    );
    assert!(command(home).output().unwrap().status.success());
    private(&home.join(".miilo/config.json"), r#"{"schemaVersion":1}"#);
    // Invalid shared connection data proves that legacy settings are not silently used.
    private(
        &home.join(".miilo/connections/config.json"),
        "bad shared data",
    );
    assert!(!command(home).output().unwrap().status.success());
    assert!(
        command(home)
            .env("MIILO_OWL_CONFIG_DIR", home.join(".config/miilo-owl"))
            .output()
            .unwrap()
            .status
            .success()
    );
    private(
        &home.join(".miilo/connections/config.json"),
        r#"{"version":1,"endpoints":[]}"#,
    );
    assert!(command(home).output().unwrap().status.success());
}

#[test]
fn unsupported_manifest_and_relative_home_fail_without_overwriting() {
    let scratch = tempfile::tempdir().unwrap();
    let home = scratch.path();
    let root = home.join("custom");
    private(&root.join("config.json"), r#"{"schemaVersion":99}"#);
    let result = command(home).env("MIILO_HOME", &root).output().unwrap();
    assert!(!result.status.success());
    // The executable sanitizes internal errors; its public contract is JSON stdout.
    let failure: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(failure["status"], "failed");
    assert_eq!(
        failure["message"],
        "Request failed. Refresh and check setup/access."
    );
    assert!(result.stderr.is_empty());
    assert_eq!(
        fs::read_to_string(root.join("config.json")).unwrap(),
        r#"{"schemaVersion":99}"#
    );
    assert!(!root.join("connections").exists());
    assert!(
        !command(home)
            .env("MIILO_HOME", "relative")
            .output()
            .unwrap()
            .status
            .success()
    );
}
