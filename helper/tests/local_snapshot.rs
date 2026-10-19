mod common;
use common::*;
use miilo_owl::{
    config::Endpoint,
    local_import, projection,
    snapshot::collect,
    transport::{Http, Transport},
};
use serde_json::{Value, json};
use std::{fs, os::unix::fs::PermissionsExt};
fn private(path: &std::path::Path, value: &Value) {
    fs::write(path, value.to_string()).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
}
#[test]
fn standalone_cache_connects_without_http_preserves_remote_and_fails_stale() {
    use sha2::{Digest, Sha256};
    for kind in [
        "codex", "cursor", "claude", "opencode", "devin", "grok", "hermes",
    ] {
        let temp = tempfile::tempdir().unwrap();
        let store = store(temp.path(), vec![endpoint()]);
        let cache = temp.path().join("snapshot.json");
        let id = format!("{kind}-{:x}", Sha256::digest(b"selected"))[..kind.len() + 17].to_owned();
        let mut raw = fixture();
        raw["observedAt"] = json!(chrono::Utc::now().to_rfc3339());
        raw["providers"][0]["id"] = json!(id);
        raw["workItems"][0]["providerId"] = json!(id);
        raw["workItems"][0]["actions"] = json!([]);
        raw["extensions"] = json!({"dev.miilo.explanations":[{"providerId":id,"workItemId":"task-1","revision":"r1","summary":"Observed test activity","facts":["Provider state: observed"],"limitations":["No verification evidence was supplied."],"evidenceIds":[]}]});
        private(&cache, &raw);
        let local = temp.path().join("local.json");
        private(
            &local,
            &json!({"version":2,"mode":"local","snapshotFile":cache,"sessions":[{"kind":kind,"id":"selected"}]}),
        );
        let http = Http::new().unwrap();
        local_import::connect(&store, &http, &local).unwrap();
        let config = store.load().unwrap();
        assert_eq!(config.endpoints.len(), 2);
        let endpoint = &config.endpoints[1];
        assert!(endpoint.token_file.is_none());
        assert!(!endpoint.allow_actions);
        let projected = projection::project(
            endpoint,
            &http.request(endpoint, "/v1/snapshot", None).unwrap(),
        )
        .unwrap();
        assert_eq!(projected["workItems"].as_array().unwrap().len(), 1);
        let request = json!({"providerId":id,"workItemId":"task-1","expectedRevision":"r1"});
        let explanation = http
            .request(endpoint, "/v1/explanations", Some(&request))
            .unwrap();
        assert_eq!(explanation["spokenSummary"], "Observed test activity");
        assert!(
            http.request(endpoint, "/v1/actions", Some(&request))
                .is_err()
        );
        let mut stale = request;
        stale["expectedRevision"] = json!("old");
        assert!(
            http.request(endpoint, "/v1/explanations", Some(&stale))
                .is_err()
        );
        let only_local = miilo_owl::config::Store {
            directory: temp.path().join("only-local"),
        };
        only_local
            .save(&miilo_owl::config::Config {
                version: 1,
                endpoints: vec![endpoint.clone()],
            })
            .unwrap();
        assert_eq!(
            miilo_owl::snapshot::snapshot(&only_local, &http, false).unwrap()["status"],
            "ready"
        );
        raw["observedAt"] = json!("2020-01-01T00:00:00Z");
        private(&cache, &raw);
        assert!(http.request(endpoint, "/v1/snapshot", None).is_err());
        assert_eq!(
            miilo_owl::snapshot::snapshot(&only_local, &http, false).unwrap()["status"],
            "partial"
        );
        let only = miilo_owl::config::Config {
            version: 1,
            endpoints: vec![endpoint.clone()],
        };
        assert_eq!(collect(&only, &http)["status"], "partial");
    }
}
#[test]
fn local_cache_cannot_enable_actions_or_carry_credentials() {
    let temp = tempfile::tempdir().unwrap();
    let mut endpoint = Endpoint {
        paired_host: None,
        local_snapshot_file: Some(temp.path().join("snapshot.json")),
        url: "local://desktop".into(),
        token: String::new(),
        token_file: None,
        providers: vec![],
        allow_actions: false,
        id: "local".into(),
        actor_names: Default::default(),
    };
    let validate = |e: Endpoint| {
        miilo_owl::config::validate(&miilo_owl::config::Config {
            version: 1,
            endpoints: vec![e],
        })
    };
    assert!(validate(endpoint.clone()).is_ok());
    endpoint.allow_actions = true;
    assert!(validate(endpoint.clone()).is_err());
    endpoint.allow_actions = false;
    endpoint.token = "secret".into();
    assert!(validate(endpoint).is_err());
}
