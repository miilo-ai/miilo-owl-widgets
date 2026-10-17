//! Widget interaction policy; provider authorization is still enforced upstream.
use super::connection::Endpoint;
use chrono::{DateTime, Utc};
use serde_json::Value;

fn empty_input(schema: &Value) -> bool {
    let Some(object) = schema.as_object() else {
        return schema.is_null();
    };
    if object.is_empty() {
        return true;
    }
    object.keys().all(|k| {
        matches!(
            k.as_str(),
            "type"
                | "properties"
                | "required"
                | "additionalProperties"
                | "$schema"
                | "title"
                | "description"
        )
    }) && object.get("type").is_none_or(|v| v == "object")
        && object
            .get("properties")
            .is_none_or(|v| v.as_object().is_some_and(|p| p.is_empty()))
        && object
            .get("required")
            .is_none_or(|v| v.as_array().is_some_and(|r| r.is_empty()))
        && object
            .get("additionalProperties")
            .is_none_or(Value::is_boolean)
}

pub fn unavailable_reason_at(
    endpoint: &Endpoint,
    action: &Value,
    now: DateTime<Utc>,
) -> Option<&'static str> {
    if action["available"] != true {
        return Some("Not currently available from the provider");
    }
    if !endpoint.allow_actions {
        return Some("Actions are disabled for this connection");
    }
    let risk = action["risk"].as_str().unwrap_or("");
    let confirmation = action["confirmation"].as_str().unwrap_or("");
    if !matches!(risk, "read" | "bounded" | "privileged") {
        return Some("Use the provider for this action");
    }
    if !matches!(confirmation, "none" | "explicit")
        || (risk == "privileged" && confirmation != "explicit")
    {
        return Some("This action needs provider confirmation");
    }
    if !empty_input(&action["inputSchema"]) {
        return Some("This action needs input in the provider");
    }
    if action
        .get("expiresAt")
        .filter(|v| !v.is_null())
        .is_some_and(|v| {
            v.as_str()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .is_none_or(|t| t <= now)
        })
    {
        return Some("Action expired; refresh before acting");
    }
    None
}
