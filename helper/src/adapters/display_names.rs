//! Local presentation preferences never alter provider identities or authorization.
use crate::{Result, config::Store, snapshot::collect, transport::Transport};
use serde_json::{Value, json};

pub fn name_agent(
    store: &Store,
    http: &dyn Transport,
    actor_key: &str,
    label: &str,
) -> Result<Value> {
    if actor_key.len() != 32
        || !actor_key
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err("Invalid agent identity".into());
    }
    let label = label.trim();
    if label.is_empty() || label.chars().count() > 80 || label.chars().any(char::is_control) {
        return Err("Use a short display name without control characters".into());
    }
    let mut config = store.load()?;
    let observed = collect(&config, http);
    let actor = observed["snapshot"]["actors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["_key"] == actor_key)
        .ok_or("Agent unavailable; refresh first")?;
    let endpoint = config
        .endpoints
        .iter_mut()
        .find(|e| actor["_source"] == e.id)
        .ok_or("Endpoint unavailable")?;
    endpoint
        .actor_names
        .entry(
            actor["_provider"]
                .as_str()
                .ok_or("Invalid provider")?
                .into(),
        )
        .or_default()
        .insert(
            actor["id"].as_str().ok_or("Invalid agent")?.into(),
            label.into(),
        );
    store.save(&config)?;
    Ok(json!({"status":"completed","message":"Display name saved"}))
}
