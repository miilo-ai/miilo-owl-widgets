use miilo_session_telemetry::{Host, carrier};
use std::{fs, os::unix::fs::PermissionsExt};
#[test]
fn rejects_invalid_or_noncanonical_carriers() {
    for value in [
        "",
        "00-00000000000000000000000000000000-1111111111111111-01",
        "00-12345678901234567890123456789012-0000000000000000-01",
        "00-12345678901234567890123456789012-1111111111111111-00",
        "private prompt",
    ] {
        assert!(carrier::parse(value).is_none());
    }
    let value = "00-12345678901234567890123456789012-1111111111111111-01";
    assert_eq!(
        carrier::encode(&carrier::parse(value).unwrap()).as_deref(),
        Some(value)
    );
}
#[test]
fn absent_disabled_and_insecure_configuration_never_start_export() {
    let root = std::env::temp_dir().join(format!("miilo-telemetry-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let path = root.join("telemetry.json");
    assert!(Host::load(&path).unwrap().is_none());
    fs::write(&path, r#"{"enabled":false,"endpoint":"","tokenFile":null}"#).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    assert!(Host::load(&path).unwrap().is_none());
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(Host::load(&path).is_err());
    fs::remove_dir_all(&root).unwrap();
}
