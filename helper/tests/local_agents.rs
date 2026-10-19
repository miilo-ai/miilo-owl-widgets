use miilo_owl::local_agents::detect;
use std::{fs, os::unix::fs::PermissionsExt};
#[test]
fn detects_known_folders_and_executable_commands_without_reading_contents() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let bin = temp.path().join("bin");
    fs::create_dir_all(home.join(".codex")).unwrap();
    fs::create_dir_all(home.join(".config/devin")).unwrap();
    fs::create_dir_all(home.join(".local/share/opencode")).unwrap();
    fs::create_dir_all(&bin).unwrap();
    fs::write(
        home.join(".codex/auth.json"),
        "not JSON; must never be opened",
    )
    .unwrap();
    fs::set_permissions(
        home.join(".codex/auth.json"),
        fs::Permissions::from_mode(0o0),
    )
    .unwrap();
    let executable = bin.join("cursor");
    fs::write(&executable, "not a runnable program; must never execute").unwrap();
    fs::set_permissions(executable, fs::Permissions::from_mode(0o755)).unwrap();
    let result = detect(&home, &[bin]);
    let agents = result["agents"].as_array().unwrap();
    let agent = |id: &str| agents.iter().find(|a| a["id"] == id).unwrap();
    assert_eq!(agent("codex")["configFound"], true);
    assert_eq!(agent("cursor")["commandFound"], true);
    assert_eq!(agent("devin")["found"], true);
    assert_eq!(agent("devin")["observationSupported"], true);
    assert_eq!(agent("claude")["observationSupported"], true);
    assert_eq!(agent("opencode")["found"], true);
    assert_eq!(agent("opencode")["observationSupported"], true);
    assert_eq!(agent("grok")["observationSupported"], true);
    assert_eq!(agent("hermes")["observationSupported"], true);
    assert_eq!(agent("claude")["found"], false);
    assert_eq!(result["metadataOnly"], true);
    assert!(!result.to_string().contains("auth.json"));
}
