use crate::{Result, root};
use miilo_owl::{config::Endpoint, projection};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};
fn project(raw: &Value, alias: &str) -> Result<Value> {
    let endpoint: Endpoint = serde_json::from_value(
        json!({"id":alias,"url":"http://127.0.0.1","providers":["factory","codex"],"allowActions":true}),
    )?;
    Ok(projection::project(&endpoint, raw).map_err(std::io::Error::other)?)
}
pub fn write(folder: &Path) -> Result<BTreeMap<String, Value>> {
    let mut raw: Value =
        serde_json::from_str(&fs::read_to_string(root().join("tests/populated.json"))?)?;
    raw = raw["snapshot"].take();
    raw["protocolVersion"] = json!("0.1");
    for item in raw["attention"].as_array_mut().unwrap() {
        item["subject"]["kind"] = json!("task");
    }
    raw["attempts"] = json!([{"id":"attempt-1","providerId":"factory","taskId":"0","actorIds":["0"],"lifecycle":"active","phase":"verifying"}]);
    raw["evidence"] = json!([{"id":"proof-1","providerId":"factory","attemptId":"attempt-1","outcome":"passed","summary":"No repeated events","artifactIds":["file-1"]}]);
    raw["artifacts"] = json!([{"id":"file-1","providerId":"factory","attemptId":"attempt-1","kind":"file","name":"src/worker.rs","extensions":{"example":{"fileDiff":"@@ -1 +1 @@\n-old\n+new"}}},{"id":"file-2","providerId":"factory","attemptId":"attempt-1","kind":"file","name":"README.md"}]);
    for work in raw["workItems"].as_array_mut().unwrap() {
        let extra = json!({"revision":"r1","summary":"Checks passed; awaiting review.","kind":"task","actions":[{"id":"approve","label":"Approve","risk":"privileged","confirmation":"explicit","inputSchema":{"type":"object","additionalProperties":false},"available":true},{"id":"reject","label":"Reject","risk":"bounded","confirmation":"explicit","available":true},{"id":"retry","label":"Retry","risk":"bounded","confirmation":"explicit","available":true}]});
        work.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
    }
    let selected = project(&raw, "home")?;
    let empty: Value = selected
        .as_object()
        .unwrap()
        .keys()
        .map(|k| (k.clone(), json!([])))
        .collect();
    let populated = json!({"configured":true,"snapshot":selected,"errors":[]});
    let other = project(&raw, "office")?;
    let multiple: Value = selected
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            let mut list = v.as_array().unwrap().clone();
            list.extend(other[k].as_array().unwrap().clone());
            (k.clone(), json!(list))
        })
        .collect();
    let mut partial = populated.clone();
    partial["errors"] = json!([{"id":"office","message":"Unavailable"}]);
    let mut crowded = raw.clone();
    for field in ["workItems", "attention"] {
        let original = crowded[field].as_array().unwrap().clone();
        let mut all = vec![];
        for i in 0..12 {
            for item in &original {
                let mut item = item.clone();
                item["id"] = json!(format!("{i}-{}", item["id"].as_str().unwrap()));
                all.push(item);
            }
        }
        crowded[field] = json!(all);
    }
    for actor in crowded["actors"].as_array_mut().unwrap() {
        actor["name"] = json!("Long independent verification agent name");
    }
    let cases = BTreeMap::from([
        ("populated".into(), populated),
        (
            "disconnected".into(),
            json!({"configured":false,"snapshot":empty,"errors":[]}),
        ),
        (
            "empty".into(),
            json!({"configured":true,"snapshot":empty,"errors":[]}),
        ),
        (
            "multiple".into(),
            json!({"configured":true,"snapshot":multiple,"errors":[]}),
        ),
        ("partial".into(), partial),
        (
            "crowded".into(),
            json!({"configured":true,"snapshot":project(&crowded,"home")?,"errors":[]}),
        ),
    ]);
    for (name, data) in &cases {
        fs::write(
            folder.join(format!("{name}.json")),
            serde_json::to_vec(data)?,
        )?;
    }
    Ok(cases)
}
