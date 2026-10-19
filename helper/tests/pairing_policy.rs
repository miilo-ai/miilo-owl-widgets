mod common;
use miilo_owl::{
    application::ports::Transport,
    config::{Config, validate},
    transport::Http,
};
use miilo_pairing::domain::PairedHost;
#[test]
fn paired_computers_cannot_enable_actions_change_origins_or_mix_credentials() {
    let mut endpoint = common::endpoint();
    endpoint.url = "https://example.invalid:38761".into();
    endpoint.token.clear();
    endpoint.allow_actions = false;
    endpoint.paired_host = Some(PairedHost {
        host_id: "a".repeat(32),
        device_id: "b".repeat(32),
        name: "Test host".into(),
        url: endpoint.url.clone(),
        certificate_pem: "invalid test certificate".into(),
        fingerprint: "c".repeat(64),
        token: "d".repeat(64),
        expires_at: 1,
    });
    let config = |e| Config {
        version: 1,
        endpoints: vec![e],
    };
    validate(&config(endpoint.clone())).unwrap();
    let http = Http::new().unwrap();
    assert!(
        http.request(&endpoint, "/v1/actions", Some(&serde_json::json!({})))
            .unwrap_err()
            .contains("read-only")
    );
    assert!(
        http.request(&endpoint, "/v1/snapshot", None)
            .unwrap_err()
            .contains("certificate")
    );
    let mut changed = endpoint.clone();
    changed.url = "https://other.invalid".into();
    assert!(validate(&config(changed)).is_err());
    let mut changed = endpoint.clone();
    changed.allow_actions = true;
    assert!(validate(&config(changed)).is_err());
    let mut changed = endpoint.clone();
    changed.token = "other-token".into();
    assert!(validate(&config(changed)).is_err());
    let mut changed = endpoint;
    changed.token_file = Some("/tmp/test-token".into());
    assert!(validate(&config(changed)).is_err());
}

#[test]
fn pairing_errors_do_not_echo_remote_payloads_or_credentials() {
    use miilo_owl::adapters::pairing::diagnostic;
    assert_eq!(
        diagnostic("Cannot read invitation"),
        "Cannot read invitation"
    );
    assert!(!diagnostic("server returned credential=private-value").contains("private-value"));
}

#[test]
fn paired_explanations_use_observed_data_and_never_post_to_the_host() {
    use miilo_owl::{operations, projection::project};
    use serde_json::json;
    let mut endpoint = common::endpoint();
    endpoint.token.clear();
    endpoint.allow_actions = false;
    endpoint.url = "https://example.invalid".into();
    endpoint.paired_host = Some(PairedHost {
        host_id: "a".repeat(32),
        device_id: "b".repeat(32),
        name: "Workstation".into(),
        url: endpoint.url.clone(),
        certificate_pem: "fixture".into(),
        fingerprint: "c".repeat(64),
        token: "d".repeat(64),
        expires_at: 1,
    });
    let tmp = tempfile::tempdir().unwrap();
    let store = common::store(tmp.path(), vec![endpoint.clone()]);
    let transport = common::Fake::default();
    transport.raw.lock().unwrap()["extensions"] = json!({"dev.miilo.explanations":[{
        "providerId":"factory","workItemId":"task-1","revision":"r1",
        "summary":"Observed summary.","limitations":["Inferred activity."],"evidenceIds":[],"facts":[]
    }]});
    let work = project(&endpoint, &transport.raw.lock().unwrap()).unwrap()["workItems"][0].clone();
    assert_eq!(work["_pairedComputer"], true);
    let result = operations::explain(
        &store,
        &transport,
        work["_key"].as_str().unwrap(),
        work["_revisionKey"].as_str().unwrap(),
    )
    .unwrap();
    assert!(
        result["message"]
            .as_str()
            .unwrap()
            .contains("Inferred activity.")
    );
    assert!(
        transport
            .calls
            .lock()
            .unwrap()
            .iter()
            .all(|(_, path, body)| path == "/v1/snapshot" && body.is_none())
    );
    let raw = transport.raw.lock().unwrap();
    assert!(
        miilo_owl::domain::observation_explanation::explain(
            &raw,
            &json!({"providerId":"factory","workItemId":"task-1","expectedRevision":"r2"})
        )
        .is_err()
    );
}
