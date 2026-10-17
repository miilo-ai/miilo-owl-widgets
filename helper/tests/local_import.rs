mod common;
use common::*;
use miilo_owl::{local_import, transport::Transport};
use serde_json::{Value, json};
use std::{fs, os::unix::fs::PermissionsExt, path::Path, process::Command};
struct Registered;
impl Transport for Registered {
    fn request(
        &self,
        e: &miilo_owl::config::Endpoint,
        path: &str,
        _: Option<&Value>,
    ) -> miilo_owl::Result<Value> {
        assert_eq!(path, "/v1/snapshot");
        Ok(json!({"providers":e.providers.iter().map(|id|json!({"id":id})).collect::<Vec<_>>()}))
    }
}
fn private(path: &Path, value: &Value) {
    fs::write(path, value.to_string()).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
}
#[test]
fn confirmed_local_connection_preserves_existing_endpoints_and_uses_credential_reference() {
    let temp = tempfile::tempdir().unwrap();
    let store = store(temp.path(), vec![endpoint()]);
    let credential = temp.path().join("local.token");
    fs::write(&credential, "private-local-credential").unwrap();
    fs::set_permissions(&credential, fs::Permissions::from_mode(0o600)).unwrap();
    let local = temp.path().join("local.json");
    private(
        &local,
        &json!({"version":1,"endpoint":"http://127.0.0.1:3717","tokenFile":credential,"sessions":[{"kind":"codex","id":"selected"}]}),
    );
    let result = local_import::connect(&store, &Registered, &local).unwrap();
    assert_eq!(result["status"], "connected");
    assert!(!result.to_string().contains("credential"));
    let config = store.load().unwrap();
    assert_eq!(config.endpoints.len(), 2);
    assert_eq!(config.endpoints[0].id, "home");
    let imported = &config.endpoints[1];
    assert_eq!(imported.id, "local-desktop");
    assert!(imported.token.is_empty());
    assert_eq!(imported.token_file.as_ref(), Some(&credential));
    assert!(!imported.allow_actions);
    local_import::connect(&store, &Registered, &local).unwrap();
    assert_eq!(store.load().unwrap().endpoints.len(), 2);
}
#[test]
fn incomplete_or_remote_import_cannot_replace_widget_connections() {
    let temp = tempfile::tempdir().unwrap();
    let store = store(temp.path(), vec![endpoint()]);
    let local = temp.path().join("local.json");
    let before = fs::read(store.directory.join("config.json")).unwrap();
    for config in [
        json!({"version":1,"endpoint":"https://remote.example","sessions":[{"kind":"codex","id":"selected"}]}),
        json!({"version":1,"endpoint":"http://127.0.0.1:3717","tokenFile":"/tmp/not-present","sessions":[{"kind":"devin","id":"unsupported"}]}),
    ] {
        private(&local, &config);
        assert!(local_import::connect(&store, &Registered, &local).is_err());
        assert_eq!(
            fs::read(store.directory.join("config.json")).unwrap(),
            before
        );
    }
}
#[test]
fn not_now_persists_for_this_discovery_and_new_folders_trigger_an_invitation() {
    let temp = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        let result = Command::new(env!("CARGO_BIN_EXE_miilo-owl"))
            .args(args)
            .env("HOME", temp.path())
            .env_remove("CODEX_HOME")
            .env_remove("MIILO_CURSOR_PROJECTS")
            .env("MIILO_OWL_CONFIG_DIR", temp.path().join("miilo-config"))
            .output()
            .unwrap();
        assert!(result.status.success());
        serde_json::from_slice::<Value>(&result.stdout).unwrap()
    };
    let found = run(&["local-agents"]);
    assert_eq!(found["dismissed"], false);
    run(&["dismiss-local", found["fingerprint"].as_str().unwrap()]);
    assert_eq!(run(&["local-agents"])["dismissed"], true);
    fs::create_dir(temp.path().join(".cursor")).unwrap();
    assert_eq!(run(&["local-agents"])["dismissed"], false);
}
