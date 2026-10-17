use crate::{
    Result,
    config::{Endpoint, Store},
    key,
    snapshot::collect,
    transport::Transport,
};
use chrono::Utc;
use serde_json::{Value, json};

fn opaque(s: &str) -> bool {
    s.len() == 32
        && s.bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}
pub fn target(
    store: &Store,
    http: &dyn Transport,
    item: &str,
    revision: &str,
) -> Result<(Endpoint, Value)> {
    if !opaque(item) || !opaque(revision) {
        return Err("Invalid target".into());
    }
    let config = store.load()?;
    let data = collect(&config, http);
    let work = data["snapshot"]["workItems"]
        .as_array()
        .unwrap()
        .iter()
        .find(|w| w["_key"] == item)
        .ok_or("Task unavailable; refresh before acting")?
        .clone();
    if work["_revisionKey"] != revision {
        return Err("Task changed; review the latest revision".into());
    }
    let endpoint = config
        .endpoints
        .into_iter()
        .find(|e| work["_source"] == e.id)
        .ok_or("Endpoint unavailable")?;
    Ok((endpoint, work))
}
pub fn explain(store: &Store, http: &dyn Transport, item: &str, revision: &str) -> Result<Value> {
    let (endpoint, work) = target(store, http, item, revision)?;
    let bundle = http.request(&endpoint, "/v1/explanations", Some(&json!({
        "providerId":work["_provider"],"workItemId":work["id"],"expectedRevision":work["revision"],"question":null,
        "preferences":{"detail":"concise","lead":"cause","includeTechnicalDetails":false,"speakSensitiveValues":false}
    })))?;
    if bundle["revision"] != work["revision"] {
        return Err("Explanation revision changed; refresh".into());
    }
    let message = bundle["spokenSummary"]
        .as_str()
        .filter(|s| !s.is_empty())
        .or_else(|| bundle["cause"].as_str().filter(|s| !s.is_empty()))
        .unwrap_or("No explanation provided");
    let message = if endpoint.local_snapshot_file.is_some() {
        let limitations = bundle["limitations"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(" ");
        format!("{message} {limitations}")
    } else {
        message.to_owned()
    };
    Ok(
        json!({"status":"completed","message":message,"changes":bundle.get("changes").unwrap_or(&json!([])),"evidence":bundle.get("evidence").unwrap_or(&json!([]))}),
    )
}
pub fn action(
    store: &Store,
    http: &dyn Transport,
    item: &str,
    action: &str,
    revision: &str,
) -> Result<Value> {
    let (endpoint, work) = target(store, http, item, revision)?;
    let candidate = work["actions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["_key"] == action && a["_supported"] == true)
        .ok_or("Action unavailable here; use the provider interface")?;
    let stable = key(&json!([
        endpoint.id,
        work["_provider"],
        work["id"],
        candidate["id"],
        work["revision"]
    ]));
    let payload = json!({"protocolVersion":"0.1","requestId":stable,"providerId":work["_provider"],"actionId":candidate["id"],
        "subject":{"kind":work.get("kind").unwrap_or(&json!("task")),"id":work["id"]},"expectedRevision":work["revision"],
        "confirmation":{"method":"manual","confirmedAt":Utc::now().to_rfc3339()},"input":{},"idempotencyKey":stable});
    // Never retry mutation requests; a timeout is not evidence that nothing happened.
    let receipt = http.request(&endpoint, "/v1/actions", Some(&payload));
    store.invalidate()?;
    let receipt = receipt?;
    let status = receipt["status"]
        .as_str()
        .filter(|s| {
            matches!(
                *s,
                "accepted" | "completed" | "rejected" | "failed" | "conflict"
            )
        })
        .ok_or("Invalid action receipt; inspect provider before retrying")?;
    Ok(
        json!({"status":status,"message":receipt["message"].as_str().filter(|s| !s.is_empty()).map(str::to_owned).unwrap_or_else(|| format!("Provider: {status}"))}),
    )
}
