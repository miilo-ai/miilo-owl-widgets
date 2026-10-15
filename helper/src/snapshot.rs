use crate::{
    Result,
    config::{Config, Store, private_file},
    key,
    projection::{self, COLLECTIONS},
    transport::Transport,
};
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}
pub fn collect(config: &Config, http: &dyn Transport) -> Value {
    let mut result = projection::empty();
    let mut errors = vec![];
    // Four scoped workers maximum; endpoint order stays stable for widget selection.
    for batch in config.endpoints.chunks(4) {
        let responses = std::thread::scope(|scope| {
            batch
                .iter()
                .map(|endpoint| {
                    scope.spawn(move || {
                        http.request(endpoint, "/v1/snapshot", None)
                            .and_then(|raw| projection::project(endpoint, &raw))
                    })
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|h| {
                    h.join()
                        .unwrap_or_else(|_| Err("Endpoint worker failed".into()))
                })
                .collect::<Vec<_>>()
        });
        for (endpoint, response) in batch.iter().zip(responses) {
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
    }
    json!({"configured":!config.endpoints.is_empty(),"snapshot":result,"status":if errors.is_empty() {"ready"} else {"partial"},"errors":errors,"observedAt":now()})
}
pub fn snapshot(store: &Store, http: &dyn Transport, force: bool) -> Result<Value> {
    let _lock = store.lock()?;
    let config = store.load()?;
    let fingerprint = key(&serde_json::to_value(&config).map_err(|_| "Invalid configuration")?);
    if !force
        && let Ok(file) = private_file(&store.directory.join("snapshot-cache.json"))
        && let Ok(cached) =
            serde_json::from_reader::<_, Value>(std::io::Read::take(file, 20_000_001))
    {
        let age = now() - cached["data"]["observedAt"].as_f64().unwrap_or(0.0);
        if cached["fingerprint"] == fingerprint && (0.0..8.0).contains(&age) {
            return Ok(cached["data"].clone());
        }
    }
    let data = collect(&config, http);
    store.write(
        "snapshot-cache.json",
        &json!({"fingerprint":fingerprint,"data":data}),
    )?;
    Ok(data)
}
