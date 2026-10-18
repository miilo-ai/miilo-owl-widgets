//! File transport for the local engine's normalized, consent-scoped projection.
use crate::{
    Result,
    config::{Endpoint, Store, private_file},
    transport::Transport,
};
use serde_json::{Value, json};
use std::{io::Read, path::PathBuf};
fn read(endpoint: &Endpoint) -> Result<Value> {
    let path = endpoint
        .local_snapshot_file
        .as_ref()
        .ok_or("No local snapshot configured")?;
    let mut bytes = vec![];
    private_file(path)?
        .take(2_000_001)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read local snapshot")?;
    if bytes.len() > 2_000_000 {
        return Err("Local snapshot exceeds the widget limit".into());
    }
    let snapshot: Value = serde_json::from_slice(&bytes).map_err(|_| "Invalid local snapshot")?;
    if snapshot["protocolVersion"] != "0.1" || !snapshot["providers"].is_array() {
        return Err("Unsupported local snapshot".into());
    }
    let at = chrono::DateTime::parse_from_rfc3339(
        snapshot["observedAt"]
            .as_str()
            .ok_or("Missing observation time")?,
    )
    .map_err(|_| "Invalid observation time")?;
    if !(0..=15).contains(&(chrono::Utc::now() - at.with_timezone(&chrono::Utc)).num_seconds()) {
        return Err("Local observation is stale; start miilo local run".into());
    }
    Ok(snapshot)
}
pub(super) fn request(endpoint: &Endpoint, path: &str, body: Option<&Value>) -> Result<Value> {
    let snapshot = read(endpoint)?;
    match (path, body) {
        ("/v1/snapshot", None) => Ok(snapshot),
        ("/health", None) => Ok(json!({"status":"ok","mode":"local"})),
        ("/v1/connections", None) => Ok(
            json!({"connections":snapshot["providers"].as_array().unwrap().iter().map(|p|json!({"id":p["id"],"name":p["name"],"enabled":true,"adapter":"local-observer"})).collect::<Vec<_>>()}),
        ),
        ("/v1/explanations", Some(request)) => {
            let work = snapshot["workItems"]
                .as_array()
                .ok_or("No observed work")?
                .iter()
                .find(|w| {
                    w["providerId"] == request["providerId"] && w["id"] == request["workItemId"]
                })
                .ok_or("Observed work unavailable")?;
            if work["revision"] != request["expectedRevision"] {
                return Err("Observation changed; refresh before explaining".into());
            }
            let explanation = snapshot["extensions"]["dev.miilo.explanations"]
                .as_array()
                .and_then(|items| {
                    items.iter().find(|e| {
                        e["providerId"] == work["providerId"]
                            && e["workItemId"] == work["id"]
                            && e["revision"] == work["revision"]
                    })
                })
                .ok_or("Local explanation unavailable; update the local engine")?;
            let evidence: Vec<_> = snapshot["evidence"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|e| {
                    e["providerId"] == work["providerId"]
                        && explanation["evidenceIds"]
                            .as_array()
                            .is_some_and(|ids| ids.contains(&e["id"]))
                })
                .cloned()
                .collect();
            Ok(
                json!({"revision":work["revision"],"spokenSummary":explanation["summary"],"changes":[],"evidence":evidence,"facts":explanation["facts"],"limitations":explanation["limitations"]}),
            )
        }
        _ => Err(
            "This local observation source is read-only and does not support that operation".into(),
        ),
    }
}
pub(super) fn connect(store: &Store, http: &dyn Transport, local: &Value) -> Result<Value> {
    let path = PathBuf::from(
        local["snapshotFile"]
            .as_str()
            .ok_or("Missing local snapshot file")?,
    );
    if !path.is_absolute() {
        return Err("Local snapshot file must be absolute".into());
    }
    let sessions = local["sessions"]
        .as_array()
        .filter(|s| !s.is_empty() && s.len() <= 100)
        .ok_or("Select between one and 100 sessions")?;
    let mut endpoint = Endpoint {
        id: "local-desktop".into(),
        url: "local://desktop".into(),
        token: String::new(),
        token_file: None,
        local_snapshot_file: Some(path),
        providers: vec![],
        allow_actions: false,
        actor_names: Default::default(),
    };
    let snapshot = http.request(&endpoint, "/v1/snapshot", None)?;
    for session in sessions {
        use sha2::{Digest, Sha256};
        let kind = session["kind"]
            .as_str()
            .filter(|k| matches!(*k, "codex" | "cursor" | "claude" | "opencode" | "devin"))
            .ok_or("Unsupported local agent")?;
        let id = session["id"]
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("Missing session ID")?;
        let provider =
            format!("{kind}-{:x}", Sha256::digest(id.as_bytes()))[..kind.len() + 17].to_owned();
        if !snapshot["providers"]
            .as_array()
            .is_some_and(|ps| ps.iter().any(|p| p["id"] == provider))
        {
            return Err("Selected session has not reached local observation".into());
        }
    }
    let _lock = store.lock()?;
    let mut config = store.load()?;
    if let Some(old) = config.endpoints.iter_mut().find(|e| e.id == endpoint.id) {
        endpoint.actor_names = old.actor_names.clone();
        *old = endpoint;
    } else {
        config.endpoints.push(endpoint);
    }
    store.save(&config)?;
    Ok(
        json!({"status":"connected","mode":"local","sessions":sessions.len(),"message":"Local agents imported"}),
    )
}

/// Include local revision/freshness in the presentation cache key. A dead source
/// must not acquire another cache lifetime merely because the helper refreshed it.
pub(crate) fn cache_signature(config: &crate::config::Config) -> Result<Value> {
    let mut signatures = vec![];
    for endpoint in &config.endpoints {
        if endpoint.local_snapshot_file.is_some() {
            let snapshot = read(endpoint)?;
            signatures.push(json!([
                endpoint.id,
                snapshot["snapshotId"],
                snapshot["cursor"],
                snapshot["observedAt"]
            ]));
        }
    }
    Ok(json!(signatures))
}
