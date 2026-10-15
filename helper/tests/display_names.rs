mod common;
use common::*;
use miilo_owl::{config::Config, display_names, projection::project, snapshot};
use serde_json::json;

#[test]
fn display_names_are_local_scoped_and_preserve_ids() {
    let tmp = tempfile::tempdir().unwrap();
    let mut office = endpoint();
    office.id = "office".into();
    let store = store(tmp.path(), vec![endpoint(), office]);
    let http = Fake::default();
    http.raw.lock().unwrap()["actors"] =
        json!([{"id":"worker","providerId":"factory","name":"tower-openshell"}]);
    let raw = http.raw.lock().unwrap().clone();
    let original = project(&endpoint(), &raw).unwrap()["actors"][0].clone();
    display_names::name_agent(
        &store,
        &http,
        original["_key"].as_str().unwrap(),
        "example-factory",
    )
    .unwrap();
    let config = store.load().unwrap();
    let observed = snapshot::collect(&config, &http);
    let actors = &observed["snapshot"]["actors"];
    assert_eq!(actors[0]["name"], "example-factory");
    assert_eq!(actors[1]["name"], "tower-openshell");
    assert_eq!(actors[0]["id"], original["id"]);
    assert_eq!(actors[0]["_key"], original["_key"]);
    assert!(
        http.calls
            .lock()
            .unwrap()
            .iter()
            .all(|(_, _, body)| body.is_none())
    );
    for label in ["", "bad\nname"] {
        assert!(
            display_names::name_agent(&store, &http, original["_key"].as_str().unwrap(), label)
                .is_err()
        );
    }
    store
        .save(&Config {
            version: 1,
            endpoints: config.endpoints,
        })
        .unwrap();
}
