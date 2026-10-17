use super::ports::{Clock, SnapshotStore, Transport};
use crate::{
    Result,
    domain::{
        connection::Config,
        projection::{self, COLLECTIONS},
    },
    key,
};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
pub fn seconds(now: DateTime<Utc>) -> f64 {
    now.timestamp_millis() as f64 / 1000.0
}
pub fn collect(config: &Config, source: &dyn Transport, now: DateTime<Utc>) -> Value {
    let mut result = projection::empty();
    let mut errors = vec![];
    let responses = source.snapshots(&config.endpoints);
    for (index, endpoint) in config.endpoints.iter().enumerate() {
        let response = responses
            .get(index)
            .ok_or("Missing endpoint response")
            .and_then(|v| v.as_ref().map_err(|_| "Endpoint unavailable"))
            .and_then(|raw| {
                projection::project_at(endpoint, raw, now).map_err(|_| "Invalid projection")
            });
        match response {
            Ok(data) => {
                for name in COLLECTIONS {
                    result[name]
                        .as_array_mut()
                        .unwrap()
                        .extend(data[name].as_array().unwrap().iter().cloned());
                }
            }
            Err(_) => errors.push(
                json!({"id":endpoint.id,"message":"Unavailable or incompatible; check access"}),
            ),
        }
    }
    json!({"configured":!config.endpoints.is_empty(),"snapshot":result,"status":if errors.is_empty(){"ready"}else{"partial"},"errors":errors,"observedAt":seconds(now)})
}
pub fn snapshot(
    store: &dyn SnapshotStore,
    source: &dyn Transport,
    clock: &dyn Clock,
    force: bool,
) -> Result<Value> {
    let _lock = store.lock()?;
    let config = store.load()?;
    let signature = store.signature(&config).ok();
    let fingerprint = key(&json!([
        serde_json::to_value(&config).map_err(|_| "Invalid configuration")?,
        signature
    ]));
    let now = clock.now();
    if !force
        && signature.is_some()
        && let Ok(cached) = store.cached()
    {
        let age = seconds(now) - cached["data"]["observedAt"].as_f64().unwrap_or(0.0);
        if cached["fingerprint"] == fingerprint && (0.0..8.0).contains(&age) {
            return Ok(cached["data"].clone());
        }
    }
    let data = collect(&config, source, now);
    store.write(
        "snapshot-cache.json",
        &json!({"fingerprint":fingerprint,"data":data}),
    )?;
    Ok(data)
}
