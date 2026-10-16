mod common;
use common::endpoint;
use std::{fs, os::unix::fs::PermissionsExt};
#[test]
fn protected_credentials_are_bounded_and_reread_after_rotation() {
    let folder = tempfile::tempdir().unwrap();
    let file = folder.path().join("ce.token");
    fs::write(&file, "first-credential\n").unwrap();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).unwrap();
    let mut endpoint = endpoint();
    endpoint.token.clear();
    endpoint.token_file = Some(file.clone());
    assert_eq!(endpoint.credential().unwrap(), "first-credential");
    fs::write(&file, "rotated-credential").unwrap();
    assert_eq!(endpoint.credential().unwrap(), "rotated-credential");
    fs::set_permissions(&file, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(endpoint.credential().is_err());
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).unwrap();
    fs::write(file, "x".repeat(4097)).unwrap();
    assert!(endpoint.credential().is_err());
}
