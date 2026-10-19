use crate::{
    Result,
    config::{Endpoint, Store},
    projection::valid_records,
    setup::Console,
};
use miilo_pairing::{
    adapters::{client, system::SystemClock},
    application::Clock,
};
use serde_json::{Value, json};

pub fn run(store: &Store, args: &[String], ui: &mut dyn Console) -> Result<Value> {
    ui.say("Pairing shares only the computer's selected observations. Agent logins stay on that computer. Compare both codes before host approval.")?;
    let (text, name, selection) = if args.is_empty() {
        (
            ui.read("Private invitation (hidden): ", true)?
                .ok_or("Pairing cancelled")?,
            ui.read("Name for this device: ", false)?
                .ok_or("Pairing cancelled")?,
            None,
        )
    } else {
        let value = |key: &str| {
            args.windows(2)
                .find(|a| a[0] == key)
                .map(|a| a[1].clone())
                .ok_or_else(|| format!("Missing {key}"))
        };
        if args.len() != 6
            || args.chunks(2).any(|p| {
                !matches!(
                    p[0].as_str(),
                    "--invitation-file" | "--name" | "--providers"
                )
            })
        {
            return Err(
                "Usage: pair [--invitation-file FILE --name NAME --providers all|ID,ID]".into(),
            );
        }
        let mut bytes = String::new();
        use std::io::Read;
        crate::config::private_file(std::path::Path::new(&value("--invitation-file")?))?
            .take(16_385)
            .read_to_string(&mut bytes)
            .map_err(|_| "Cannot read invitation")?;
        (bytes, value("--name")?, Some(value("--providers")?))
    };
    let invitation = client::decode(text.trim(), SystemClock.now())?;
    let id = format!("peer-{}", invitation.host_id);
    let _lease = store.lock()?;
    let mut config = store.load()?;
    if config.endpoints.len() >= 8 {
        return Err("At most eight connections are supported".into());
    }
    if config.endpoints.iter().any(|e| e.id == id) {
        return Err("Computer already paired; remove that connection before pairing again".into());
    }
    // Release the configuration lease while waiting for the user's host-side approval.
    drop(_lease);
    ui.say(&format!("Connecting to {}", invitation.name))?;
    let peer = client::connect(invitation, &name, |code| {
        ui.say(&format!(
            "Comparison code: {code}. Approve only if the host shows the same code."
        ))
    })?;
    let snapshot = client::snapshot(&peer)?;
    valid_records(&snapshot)?;
    let providers = snapshot["providers"]
        .as_array()
        .ok_or("Invalid host providers")?;
    for p in providers {
        ui.say(&format!(
            "{}: {}",
            p["id"].as_str().unwrap_or(""),
            p["name"].as_str().unwrap_or("System")
        ))?;
    }
    let choice = match selection {
        Some(choice) => choice,
        None => ui
            .read("Observe which IDs? Comma-separated, or all: ", false)?
            .ok_or("Selection cancelled; revoke this device on the host if no longer needed")?,
    };
    let ids: Vec<String> = if choice == "all" {
        providers
            .iter()
            .filter_map(|p| p["id"].as_str().map(String::from))
            .collect()
    } else {
        choice.split(',').map(|v| v.trim().to_owned()).collect()
    };
    if ids.is_empty()
        || ids
            .iter()
            .any(|id| !providers.iter().any(|p| p["id"].as_str() == Some(id)))
    {
        return Err("Select at least one provider returned by this host; revoke unused approval on the host".into());
    }
    let _lease = store.lock()?;
    config = store.load()?;
    if config.endpoints.iter().any(|e| e.id == id) || config.endpoints.len() >= 8 {
        return Err("Connections changed while pairing; revoke the unused host approval".into());
    }
    let host_name = peer.name.clone();
    config.endpoints.push(Endpoint {
        id: id.clone(),
        url: peer.url.clone(),
        token: String::new(),
        token_file: None,
        local_snapshot_file: None,
        providers: ids,
        allow_actions: false,
        actor_names: Default::default(),
        paired_host: Some(peer),
    });
    store.save(&config)?;
    Ok(json!({"status":"paired","connection":id,"name":host_name,"readOnly":true}))
}

/// Only fixed local error messages are safe to show; never echo remote payloads.
pub fn diagnostic(error: &str) -> &str {
    match error {
        "Invitation expired; create another on the host"
        | "Device clock differs from host; synchronize both clocks"
        | "Invalid or expired invitation"
        | "Invalid invitation"
        | "Invitation too large"
        | "Use a complete Miilo pairing invitation"
        | "Invalid host certificate"
        | "Host certificate fingerprint mismatch"
        | "Cannot initialize paired connection"
        | "Cannot read private file"
        | "Cannot inspect private file"
        | "Configuration must be private (chmod 600)"
        | "Cannot read invitation"
        | "Cannot create configuration directory"
        | "Cannot protect configuration"
        | "Invalid configuration directory"
        | "Cannot lock configuration"
        | "Computer already paired; remove that connection before pairing again"
        | "Paired host unavailable or certificate changed" => error,
        _ => "Pairing failed. Check the invitation, connection and host approval.",
    }
}
