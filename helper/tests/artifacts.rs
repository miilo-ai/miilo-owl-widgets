mod common;
use common::*;
use miilo_owl::{config::Config, projection::project, snapshot};
use serde_json::json;

fn raw() -> serde_json::Value {
    let mut raw = fixture();
    raw["attempts"] = json!([{"id":"run","providerId":"factory","taskId":"task-1"}]);
    raw["artifacts"] = json!([
        {"id":"file","providerId":"factory","attemptId":"run","kind":"file","name":"src/main.rs",
         "uri":"javascript:alert(1)","extensions":{"custom":{"preview":"","fileDiff":"@@ -1 +1 @@\n-old\n+new"}}},
        {"id":"unrelated","providerId":"factory","attemptId":"other","kind":"file","name":"private.rs"}
    ]);
    raw["evidence"] = json!([{"id":"proof","providerId":"factory","attemptId":"run","artifactIds":["file","unrelated","missing"]}]);
    raw
}

#[test]
fn files_and_previews_survive_projection_without_unrelated_artifacts() {
    let p = project(&endpoint(), &raw()).unwrap();
    assert_eq!(p["artifacts"].as_array().unwrap().len(), 2);
    assert_eq!(
        p["workItems"][0]["_artifactKeys"],
        json!([p["artifacts"][0]["_key"]])
    );
    assert_eq!(
        p["evidence"][0]["_artifactKeys"],
        p["workItems"][0]["_artifactKeys"]
    );
    assert_eq!(p["evidence"][0]["_missingArtifactCount"], 2);
    assert_eq!(p["artifacts"][0]["_preview"], "@@ -1 +1 @@\n-old\n+new");
    assert_eq!(p["artifacts"][0]["_url"], "");
}

#[test]
fn artifact_links_stay_isolated_across_connections_and_providers() {
    let http = Fake::default();
    let mut raw = raw();
    raw["providers"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"other"}));
    let mut foreign = raw["artifacts"][0].clone();
    foreign["providerId"] = json!("other");
    raw["artifacts"].as_array_mut().unwrap().push(foreign);
    *http.raw.lock().unwrap() = raw;
    let mut home = endpoint();
    home.providers.push("other".into());
    let mut office = home.clone();
    office.id = "office".into();
    let p = snapshot::collect(
        &Config {
            version: 1,
            endpoints: vec![home, office],
        },
        &http,
    );
    let s = &p["snapshot"];
    assert_eq!(s["artifacts"].as_array().unwrap().len(), 6);
    assert_ne!(
        s["workItems"][0]["_artifactKeys"],
        s["workItems"][1]["_artifactKeys"]
    );
    assert_eq!(
        s["workItems"][0]["_artifactKeys"].as_array().unwrap().len(),
        1
    );
}

#[test]
fn previews_are_unicode_bounded_and_absent_collections_remain_compatible() {
    let mut raw = raw();
    raw["artifacts"][0]["extensions"] = json!({"diff":"界".repeat(9000)});
    let p = project(&endpoint(), &raw).unwrap();
    assert_eq!(
        p["artifacts"][0]["_preview"]
            .as_str()
            .unwrap()
            .chars()
            .count(),
        8000
    );
    assert_eq!(p["artifacts"][0]["_previewTruncated"], true);
    let p = project(&endpoint(), &fixture()).unwrap();
    assert_eq!(p["artifacts"], json!([]));
    assert_eq!(p["workItems"][0]["_artifactKeys"], json!([]));
    raw["artifacts"] = json!([1]);
    assert!(project(&endpoint(), &raw).is_err());
}
