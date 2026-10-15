use crate::{Result, config::Endpoint, graph, key};
use serde_json::{Value, json};
use std::collections::HashSet;

pub const COLLECTIONS: [&str; 7] = [
    "providers",
    "actors",
    "workItems",
    "attention",
    "attempts",
    "evidence",
    "artifacts",
];
pub fn empty() -> Value {
    COLLECTIONS
        .iter()
        .map(|n| (n.to_string(), json!([])))
        .collect()
}
pub fn web_url(value: &Value) -> String {
    value
        .as_str()
        .and_then(|s| {
            url::Url::parse(s)
                .ok()
                .filter(|u| {
                    matches!(u.scheme(), "http" | "https")
                        && u.host_str().is_some()
                        && u.username().is_empty()
                        && u.password().is_none()
                })
                .map(|_| s.to_owned())
        })
        .unwrap_or_default()
}
pub fn valid_records(raw: &Value) -> Result<()> {
    if raw["protocolVersion"] != "0.1" {
        return Err("Unsupported Miilo snapshot protocol".into());
    }
    for name in COLLECTIONS {
        if let Some(v) = raw.get(name) {
            let a = v.as_array().ok_or("Invalid snapshot collection")?;
            if a.iter()
                .any(|i| !i.is_object() || i["id"].as_str().is_none_or(str::is_empty))
            {
                return Err("Invalid snapshot records".into());
            }
        }
    }
    Ok(())
}
pub fn supported(endpoint: &Endpoint, action: &Value) -> bool {
    crate::action_policy::unavailable_reason(endpoint, action).is_none()
}

pub fn project(endpoint: &Endpoint, raw: &Value) -> Result<Value> {
    valid_records(raw)?;
    let available: HashSet<_> = raw["providers"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|p| p["id"].as_str())
        .collect();
    let chosen: HashSet<_> = endpoint
        .providers
        .iter()
        .map(String::as_str)
        .filter(|p| available.contains(p))
        .collect();
    let mut result = empty();
    for name in COLLECTIONS {
        let mut seen = HashSet::new();
        for original in raw[name].as_array().into_iter().flatten() {
            let provider = if name == "providers" {
                &original["id"]
            } else {
                &original["providerId"]
            };
            let Some(provider) = provider.as_str().filter(|p| chosen.contains(p)) else {
                continue;
            };
            let id = original["id"].as_str().ok_or("Missing record ID")?;
            if !seen.insert((provider, id)) {
                return Err("Duplicate identity in provider snapshot".into());
            }
            let mut item = original.clone();
            item["_source"] = json!(endpoint.id);
            item["_actionsEnabled"] = json!(endpoint.allow_actions);
            item["_provider"] = json!(provider);
            item["_key"] = json!(key(&json!([endpoint.id, provider, name, id])));
            item["_revisionKey"] = json!(key(&json!([item.get("revision").unwrap_or(&json!(""))])));
            item["nativeUrl"] = json!(web_url(&item["nativeUrl"]));
            let scoped_provider = key(&json!([endpoint.id, provider]));
            if name == "providers" {
                item["id"] = json!(scoped_provider);
                item["name"] = json!(format!(
                    "{} / {}",
                    endpoint.id,
                    original["name"].as_str().unwrap_or(provider)
                ));
            } else {
                item["providerId"] = json!(scoped_provider);
            }
            if name == "actors"
                && let Some(label) = endpoint
                    .actor_names
                    .get(provider)
                    .and_then(|names| names.get(id))
            {
                item["name"] = json!(label);
            }
            if name == "artifacts" {
                crate::artifacts::present(&mut item);
            }
            let actions = match original.get("actions") {
                None | Some(Value::Null) => vec![],
                Some(v) => v.as_array().ok_or("Invalid advertised actions")?.clone(),
            };
            let mut unique = HashSet::new();
            let mut projected = vec![];
            for mut action in actions {
                let aid = action["id"]
                    .as_str()
                    .ok_or("Invalid advertised action")?
                    .to_owned();
                if !unique.insert(aid.clone()) {
                    return Err("Duplicate action identity".into());
                }
                action["_key"] = json!(key(&json!([endpoint.id, provider, item["id"], aid])));
                action["nativeUrl"] = json!(web_url(&action["nativeUrl"]));
                action["_supported"] = json!(supported(endpoint, &action));
                action["_unavailableReason"] =
                    json!(crate::action_policy::unavailable_reason(endpoint, &action));
                projected.push(action);
            }
            item["actions"] = json!(projected);
            result[name].as_array_mut().unwrap().push(item);
        }
    }
    graph::link(&mut result);
    crate::artifacts::link(&mut result);
    Ok(result)
}
