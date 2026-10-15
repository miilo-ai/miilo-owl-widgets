mod common;
use common::*;
use miilo_owl::{operations, projection::project};
use serde_json::json;

#[test]
fn stale_revision_and_malicious_targets_never_post() {
    let tmp = tempfile::tempdir().unwrap();
    let s = store(tmp.path(), vec![endpoint()]);
    let http = Fake::default();
    let item = project(&endpoint(), &fixture()).unwrap()["workItems"][0].clone();
    http.raw.lock().unwrap()["workItems"][0]["revision"] = json!("r2");
    assert!(
        operations::action(
            &s,
            &http,
            item["_key"].as_str().unwrap(),
            item["actions"][0]["_key"].as_str().unwrap(),
            item["_revisionKey"].as_str().unwrap()
        )
        .unwrap_err()
        .contains("changed")
    );
    assert!(
        http.calls
            .lock()
            .unwrap()
            .iter()
            .all(|(_, _, b)| b.is_none())
    );
    assert!(
        operations::target(
            &s,
            &http,
            "$(touch /tmp/no)",
            item["_revisionKey"].as_str().unwrap()
        )
        .is_err()
    );
}
#[test]
fn action_payload_is_scoped_stable_and_does_not_retry() {
    let tmp = tempfile::tempdir().unwrap();
    let s = store(tmp.path(), vec![endpoint()]);
    let http = Fake::default();
    let item = project(&endpoint(), &fixture()).unwrap()["workItems"][0].clone();
    let invoke = || {
        operations::action(
            &s,
            &http,
            item["_key"].as_str().unwrap(),
            item["actions"][0]["_key"].as_str().unwrap(),
            item["_revisionKey"].as_str().unwrap(),
        )
    };
    assert_eq!(invoke().unwrap()["status"], "accepted");
    assert_eq!(invoke().unwrap()["status"], "accepted");
    let calls = http.calls.lock().unwrap();
    let bodies: Vec<_> = calls.iter().filter_map(|(_, _, b)| b.as_ref()).collect();
    assert_eq!(bodies[0]["idempotencyKey"], bodies[1]["idempotencyKey"]);
    assert_eq!(bodies[0]["providerId"], "factory");
    assert_eq!(bodies[0]["expectedRevision"], "r1");
    assert_eq!(bodies[0]["subject"], json!({"kind":"task","id":"task-1"}));
    assert_eq!(bodies[0]["confirmation"]["method"], "manual");
    drop(calls);
    *http.receipt.lock().unwrap() = Err("connection lost".into());
    let before = http.calls.lock().unwrap().len();
    assert!(invoke().is_err());
    assert_eq!(http.calls.lock().unwrap().len() - before, 2); // One fresh read + one mutation, never a retry.
}
#[test]
fn explanation_and_receipts_are_truthful() {
    let tmp = tempfile::tempdir().unwrap();
    let s = store(tmp.path(), vec![endpoint()]);
    let http = Fake::default();
    let item = project(&endpoint(), &fixture()).unwrap()["workItems"][0].clone();
    let key = item["_key"].as_str().unwrap();
    let rev = item["_revisionKey"].as_str().unwrap();
    let act = item["actions"][0]["_key"].as_str().unwrap();
    assert_eq!(
        operations::explain(&s, &http, key, rev).unwrap()["message"],
        "Recovery passed."
    );
    for status in ["accepted", "completed", "rejected", "failed", "conflict"] {
        *http.receipt.lock().unwrap() = Ok(json!({"status":status}));
        assert_eq!(
            operations::action(&s, &http, key, act, rev).unwrap()["status"],
            status
        );
    }
    *http.receipt.lock().unwrap() = Ok(json!({"status":"mystery"}));
    assert!(operations::action(&s, &http, key, act, rev).is_err());
    assert!(operations::action(&s, &http, key, "invalid", rev).is_err());
}
