//! Attach an explicitly imported local observer without exposing its credentials to QML.
use crate::{
    Result,
    config::{Endpoint, Store, private_file},
    transport::Transport,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    io::Read,
    path::{Path, PathBuf},
};

pub fn connect(store: &Store, http: &dyn Transport, path: &Path) -> Result<Value> {
    let local: Value = serde_json::from_reader(private_file(path)?.take(2_000_001))
        .map_err(|_| "Invalid local import configuration")?;
    if local["version"] == 2 && local["mode"] == "local" {
        return crate::local_snapshot::connect(store, http, &local);
    }
    if local["version"] != 1 {
        return Err("Unsupported local configuration".into());
    }
    let url = local["endpoint"].as_str().ok_or("Missing local endpoint")?;
    let parsed = url::Url::parse(url).map_err(|_| "Invalid local endpoint")?;
    if parsed.scheme() != "http"
        || parsed.host_str() != Some("127.0.0.1")
        || parsed.path() != "/"
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        return Err("Automatic local import requires a loopback endpoint".into());
    }
    let token_file = PathBuf::from(
        local["tokenFile"]
            .as_str()
            .ok_or("Missing local credential file")?,
    );
    if !token_file.is_absolute() {
        return Err("Local credential file must be absolute".into());
    }
    let sessions = local["sessions"].as_array().ok_or("No selected sessions")?;
    if sessions.is_empty() || sessions.len() > 100 {
        return Err("Select between one and 100 sessions".into());
    }
    let mut providers = vec![];
    for session in sessions {
        let kind = session["kind"].as_str().ok_or("Missing agent kind")?;
        let id = session["id"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or("Missing session ID")?;
        if !matches!(kind, "codex" | "cursor") {
            return Err("Unsupported local agent".into());
        }
        let provider =
            format!("{kind}-{:x}", Sha256::digest(id.as_bytes()))[..kind.len() + 17].to_owned();
        if !providers.contains(&provider) {
            providers.push(provider);
        }
    }
    let mut endpoint = Endpoint {
        id: "local-desktop".into(),
        url: url.into(),
        token: String::new(),
        token_file: Some(token_file),
        providers,
        allow_actions: false,
        local_snapshot_file: None,
        actor_names: Default::default(),
    };
    endpoint.credential()?;
    let snapshot = http.request(&endpoint, "/v1/snapshot", None)?;
    let registered = snapshot["providers"]
        .as_array()
        .ok_or("Local observer is not ready")?;
    if !endpoint
        .providers
        .iter()
        .all(|id| registered.iter().any(|p| p["id"] == *id))
    {
        return Err("Selected sessions have not reached the local observer".into());
    }
    let _lock = store.lock()?;
    let mut config = store.load()?;
    if let Some(existing) = config
        .endpoints
        .iter_mut()
        .find(|e| e.id == "local-desktop")
    {
        endpoint.actor_names = existing.actor_names.clone();
        *existing = endpoint;
    } else {
        config.endpoints.push(endpoint);
    }
    store.save(&config)?;
    Ok(json!({"status":"connected","message":"Local agents imported","sessions":sessions.len()}))
}
