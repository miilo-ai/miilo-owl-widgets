use crate::{Result, config::Endpoint, graph, key};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use std::collections::HashSet;

pub const COLLECTIONS: [&str; 6] = [
    "providers",
    "actors",
    "workItems",
    "attention",
    "attempts",
    "evidence",
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
    let schema_empty = action
        .get("inputSchema")
        .is_none_or(|v| v.is_null() || v.as_object().is_some_and(|o| o.is_empty()));
    let expired = action
        .get("expiresAt")
        .filter(|v| !v.is_null())
        .is_some_and(|v| {
            v.as_str()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .is_none_or(|t| t <= Utc::now())
        });
    endpoint.allow_actions
        && action["available"] == true
        && matches!(action["risk"].as_str(), Some("read" | "bounded"))
        && matches!(action["confirmation"].as_str(), Some("none" | "explicit"))
        && schema_empty
        && !expired
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
                projected.push(action);
            }
            item["actions"] = json!(projected);
            result[name].as_array_mut().unwrap().push(item);
        }
    }
    graph::link(&mut result);
    Ok(result)
}
