use serde_json::{Value, json};
use std::collections::HashMap;

fn identity(v: &Value, field: &str) -> Option<(String, String)> {
    Some((v["providerId"].as_str()?.into(), v[field].as_str()?.into()))
}
pub fn link(data: &mut Value) {
    let works: HashMap<_, _> = data["workItems"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, w)| identity(w, "id").map(|k| (k, i)))
        .collect();
    let actors: HashMap<_, _> = data["actors"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|a| identity(a, "id").map(|k| (k, a["_key"].clone())))
        .collect();
    for work in data["workItems"].as_array_mut().unwrap() {
        work["_actorKeys"] = json!([]);
        work["_evidenceKeys"] = json!([]);
    }
    let mut attempts = HashMap::new();
    for attempt in data["attempts"].as_array().unwrap().clone() {
        let index = identity(&attempt, "taskId").and_then(|k| works.get(&k).copied());
        if let Some(k) = identity(&attempt, "id") {
            attempts.insert(k, index);
        }
        if let Some(i) = index {
            for actor_id in attempt["actorIds"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
            {
                if let Some(key) = actors.get(&(
                    attempt["providerId"].as_str().unwrap_or("").into(),
                    actor_id.into(),
                )) {
                    let keys = data["workItems"][i]["_actorKeys"].as_array_mut().unwrap();
                    if !keys.contains(key) {
                        keys.push(key.clone());
                    }
                }
            }
        }
    }
    for evidence in data["evidence"].as_array().unwrap().clone() {
        if let Some(Some(i)) = identity(&evidence, "attemptId").and_then(|k| attempts.get(&k)) {
            data["workItems"][*i]["_evidenceKeys"]
                .as_array_mut()
                .unwrap()
                .push(evidence["_key"].clone());
        }
    }
    let work_items = data["workItems"].as_array().unwrap().clone();
    for attention in data["attention"].as_array_mut().unwrap() {
        let index = if matches!(
            attention["subject"]["kind"].as_str(),
            Some("task" | "initiative")
        ) {
            attention["subject"]["id"].as_str().and_then(|id| {
                works
                    .get(&(
                        attention["providerId"].as_str().unwrap_or("").into(),
                        id.into(),
                    ))
                    .copied()
            })
        } else {
            None
        };
        attention["_workKey"] = index.map_or(json!(""), |i| work_items[i]["_key"].clone());
        attention["_title"] = index.map_or_else(
            || attention.get("message").cloned().unwrap_or(json!("")),
            |i| work_items[i].get("title").cloned().unwrap_or(json!("")),
        );
    }
}
