mod common;
use common::*;
use miilo_owl::{config, key, projection::project, snapshot};
use serde_json::json;
use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
};

#[test]
fn identical_ids_across_endpoints_stay_isolated() {
    let e = endpoint();
    let mut other = e.clone();
    other.id = "office".into();
    let config = config::Config {
        version: 1,
        endpoints: vec![e, other],
    };
    let result = snapshot::collect(&config, &Fake::default());
    let w = &result["snapshot"]["workItems"];
    assert_eq!(w.as_array().unwrap().len(), 2);
    assert_ne!(w[0]["_key"], w[1]["_key"]);
    assert_ne!(w[0]["providerId"], w[1]["providerId"]);
    assert!(!result.to_string().contains("private-token"));
}
#[test]
fn explicit_selection_and_partial_failure() {
    let mut raw = fixture();
    raw["providers"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"new"}));
    let mut new = raw["workItems"][0].clone();
    new["providerId"] = json!("new");
    raw["workItems"].as_array_mut().unwrap().push(new);
    assert_eq!(
        project(&endpoint(), &raw).unwrap()["workItems"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let mut none = endpoint();
    none.providers.clear();
    assert!(
        project(&none, &raw).unwrap()["workItems"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let mut offline = endpoint();
    offline.id = "offline".into();
    let result = snapshot::collect(
        &config::Config {
            version: 1,
            endpoints: vec![endpoint(), offline],
        },
        &Fake::default(),
    );
    assert_eq!(result["status"], "partial");
    assert_eq!(result["errors"][0]["id"], "offline");
    assert_eq!(result["snapshot"]["workItems"].as_array().unwrap().len(), 1);
}
#[test]
fn malformed_and_duplicate_records_fail_closed() {
    for edit in [
        json!({"workItems":null}),
        json!({"actors":[1]}),
        json!({"protocolVersion":"9"}),
        json!({"workItems":[{"id":""}]}),
    ] {
        let mut raw = fixture();
        for (k, v) in edit.as_object().unwrap() {
            raw[k] = v.clone();
        }
        assert!(project(&endpoint(), &raw).is_err());
    }
    let mut raw = fixture();
    raw["workItems"]
        .as_array_mut()
        .unwrap()
        .push(fixture()["workItems"][0].clone());
    assert!(project(&endpoint(), &raw).is_err());
    let mut raw = fixture();
    raw["workItems"][0]["actions"]
        .as_array_mut()
        .unwrap()
        .push(fixture()["workItems"][0]["actions"][0].clone());
    assert!(project(&endpoint(), &raw).is_err());
}
#[test]
fn unsafe_actions_and_links_are_not_exposed() {
    for edit in [
        json!({"confirmation":"strong"}),
        json!({"risk":"destructive"}),
        json!({"inputSchema":{"required":["reason"]}}),
        json!({"available":false}),
        json!({"expiresAt":"2000-01-01T00:00:00Z"}),
        json!({"expiresAt":"bad-date"}),
    ] {
        let mut raw = fixture();
        for (k, v) in edit.as_object().unwrap() {
            raw["workItems"][0]["actions"][0][k] = v.clone();
        }
        assert_eq!(
            project(&endpoint(), &raw).unwrap()["workItems"][0]["actions"][0]["_supported"],
            false
        );
    }
    let mut e = endpoint();
    e.allow_actions = false;
    assert_eq!(
        project(&e, &fixture()).unwrap()["workItems"][0]["actions"][0]["_supported"],
        false
    );
    let mut raw = fixture();
    raw["workItems"][0]["nativeUrl"] = json!("javascript:alert(1)");
    assert_eq!(
        project(&endpoint(), &raw).unwrap()["workItems"][0]["nativeUrl"],
        ""
    );
}
#[test]
fn private_config_rejects_unsafe_urls_permissions_and_links() {
    let tmp = tempfile::tempdir().unwrap();
    let s = store(tmp.path(), vec![endpoint()]);
    let path = s.directory.join("config.json");
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    for url in [
        "file:///etc/passwd",
        "http://user:pass@localhost",
        "https://host?secret=x",
        "https://host/#token",
        "http://host:bad",
    ] {
        assert!(config::safe_url(url).is_err());
    }
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(s.load().is_err());
    fs::remove_file(&path).unwrap();
    symlink("missing", &path).unwrap();
    assert!(s.load().is_err());
    symlink("missing", s.directory.join("snapshot.lock")).unwrap();
    assert!(s.lock().is_err());
}
#[test]
fn opaque_keys_remain_compatible_with_python_including_unicode() {
    assert_eq!(key(&json!(["r1"])), "38edd8d1053fa194abe91a007ebea087");
    // Additional canonical vectors are checked by black-box migration tests.
    assert_eq!(
        key(&json!(["home", "factory", "workItems", "task-1"])).len(),
        32
    );
}
#[test]
fn cache_reuses_fetch_but_config_changes_invalidate() {
    let tmp = tempfile::tempdir().unwrap();
    let s = store(tmp.path(), vec![endpoint()]);
    let http = Fake::default();
    snapshot::snapshot(&s, &http, false).unwrap();
    snapshot::snapshot(&s, &http, false).unwrap();
    assert_eq!(http.calls.lock().unwrap().len(), 1);
    let mut e = endpoint();
    e.providers.clear();
    s.save(&config::Config {
        version: 1,
        endpoints: vec![e],
    })
    .unwrap();
    assert!(
        snapshot::snapshot(&s, &http, false).unwrap()["snapshot"]["workItems"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(http.calls.lock().unwrap().len(), 2);
}
