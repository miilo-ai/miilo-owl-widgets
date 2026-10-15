mod common;
use common::*;
use miilo_owl::{action_policy::unavailable_reason, projection::project};
use serde_json::json;

#[test]
fn standard_sdk_approval_and_reject_are_supported_with_explicit_confirmation() {
    let mut raw = fixture();
    raw["workItems"][0]["actions"] = json!([
        {"id":"approve","label":"Approve","available":true,"risk":"privileged","confirmation":"explicit",
         "inputSchema":{"type":"object","additionalProperties":false}},
        {"id":"reject","label":"Reject","available":true,"risk":"bounded","confirmation":"explicit",
         "inputSchema":{"type":"object","properties":{},"required":[]}}
    ]);
    let p = project(&endpoint(), &raw).unwrap();
    assert!(
        p["workItems"][0]["actions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|a| a["_supported"] == true)
    );
    let mut e = endpoint();
    e.allow_actions = false;
    let p = project(&e, &raw).unwrap();
    assert_eq!(p["workItems"][0]["_actionsEnabled"], false);
    assert!(
        p["workItems"][0]["actions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|a| a["_supported"] == false)
    );
    assert_eq!(
        p["workItems"][0]["actions"][0]["_unavailableReason"],
        "Actions are disabled for this connection"
    );
}

#[test]
fn destructive_strong_and_parameterized_actions_never_dispatch() {
    let action = fixture()["workItems"][0]["actions"][0].clone();
    for patch in [
        json!({"risk":"destructive"}),
        json!({"risk":"privileged","confirmation":"none"}),
        json!({"confirmation":"strong"}),
        json!({"inputSchema":{"type":"object","required":["reason"]}}),
        json!({"inputSchema":{"oneOf":[{}]}}),
        json!({"inputSchema":{"minProperties":1}}),
        json!({"expiresAt":"2000-01-01T00:00:00Z"}),
        json!({"available":false}),
    ] {
        let mut a = action.clone();
        for (k, v) in patch.as_object().unwrap() {
            a[k] = v.clone();
        }
        assert!(unavailable_reason(&endpoint(), &a).is_some(), "{a}");
    }
}
