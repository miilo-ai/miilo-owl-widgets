mod common;
use common::*;
use miilo_owl::projection::project;
use serde_json::json;

#[test]
fn twenty_factories_thousand_tasks_keep_graph_links_scoped() {
    let mut raw = json!({"protocolVersion":"0.1","providers":[],"workItems":[],"actors":[],"attempts":[],"evidence":[],"attention":[]});
    let mut e = endpoint();
    e.providers.clear();
    for i in 0..20 {
        let p = format!("factory-{i}");
        e.providers.push(p.clone());
        raw["providers"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":p,"name":p}));
        raw["actors"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"worker","providerId":p}));
        for j in 0..50 {
            let id = format!("task-{j}");
            let mut w = fixture()["workItems"][0].clone();
            w["id"] = json!(id);
            w["providerId"] = json!(p);
            raw["workItems"].as_array_mut().unwrap().push(w);
            raw["attempts"]
                .as_array_mut()
                .unwrap()
                .push(json!({"id":id,"providerId":p,"taskId":id,"actorIds":["worker"]}));
            raw["evidence"]
                .as_array_mut()
                .unwrap()
                .push(json!({"id":id,"providerId":p,"attemptId":id,"summary":"Passed"}));
            raw["attention"]
                .as_array_mut()
                .unwrap()
                .push(json!({"id":id,"providerId":p,"subject":{"kind":"task","id":id}}));
        }
    }
    let result = project(&e, &raw).unwrap();
    let w = &result["workItems"];
    assert_eq!(w.as_array().unwrap().len(), 1000);
    assert_ne!(w[0]["_actorKeys"], w[50]["_actorKeys"]);
    assert_ne!(w[0]["_evidenceKeys"], w[50]["_evidenceKeys"]);
    assert_eq!(result["attention"][0]["_workKey"], w[0]["_key"]);
    raw["attention"][0]["subject"]["kind"] = json!("actor");
    assert_eq!(project(&e, &raw).unwrap()["attention"][0]["_workKey"], "");
}
