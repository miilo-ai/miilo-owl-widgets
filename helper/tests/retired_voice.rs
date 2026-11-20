use std::process::Command;

#[test]
fn retired_voice_command_is_not_advertised_or_accepted() {
    let home = tempfile::tempdir().unwrap();
    let run = |argument| {
        Command::new(env!("CARGO_BIN_EXE_miilo-owl"))
            .arg(argument)
            .env("HOME", home.path())
            .env("XDG_CONFIG_HOME", home.path().join("config"))
            .output()
            .unwrap()
    };
    let help = run("--help");
    assert!(help.status.success());
    assert!(!String::from_utf8_lossy(&help.stdout).contains("voice"));

    let retired = run("voice");
    assert!(!retired.status.success());
    let result: serde_json::Value = serde_json::from_slice(&retired.stdout).unwrap();
    assert_eq!(result["status"], "failed");
    assert!(!home.path().join(".local/share/miilo/voice").exists());
}
