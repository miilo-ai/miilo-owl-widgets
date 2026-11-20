//! Explicit worker controls through Miilo CLI; no scheduler/provider policy here.
use super::worker_process::{capture, credential};
use crate::{
    Result,
    config::{Store, private_file},
    domain::workers::{WorkerConfig, identifier, validate},
};
use serde_json::{Value, json};
use std::{io::Read, process::Command, time::Duration};

pub fn configure(store: &Store, input: impl Read) -> Result<Value> {
    let config: WorkerConfig =
        serde_json::from_reader(input.take(16_385)).map_err(|_| "Invalid worker configuration")?;
    validate(&config)?;
    store.write(
        "workers.json",
        &serde_json::to_value(config).map_err(|_| "Invalid configuration")?,
    )?;
    Ok(json!({"status":"configured"}))
}
pub fn dispatch(store: &Store, args: &[String]) -> Result<Value> {
    let path = store.directory.join("workers.json");
    if !path.exists() {
        return Ok(json!({"status":"unconfigured","workers":[]}));
    }
    let config: WorkerConfig = serde_json::from_reader(private_file(&path)?.take(16_385))
        .map_err(|_| "Invalid worker configuration")?;
    validate(&config)?;
    let values: Vec<&str> = args.iter().map(String::as_str).collect();
    let mut arguments = vec![
        "--endpoint".to_owned(),
        config.endpoint.clone(),
        "--json".into(),
        "workers".into(),
    ];
    match values.as_slice() {
        ["list"] => arguments.push("list".into()),
        [mode, node, revision, request_id]
            if matches!(*mode, "available" | "pause" | "drain" | "idle-only")
                && identifier(node, 1)
                && identifier(request_id, 8)
                && revision.parse::<u64>().is_ok() =>
        {
            arguments.extend([
                mode.to_string(),
                node.to_string(),
                "--revision".into(),
                revision.to_string(),
                "--request-id".into(),
                request_id.to_string(),
            ]);
        }
        _ => return Err("Invalid worker command".into()),
    }
    let token = credential(&config)?;
    let mut command = Command::new(&config.cli_executable);
    command
        .args(arguments)
        .env_clear()
        .env("MIILO_TOKEN", token);
    if let Some(home) = std::env::var_os("HOME") {
        command.env("HOME", home);
    }
    let bytes = capture(&mut command, 2_000_000, Duration::from_secs(20))?;
    serde_json::from_slice(&bytes).map_err(|_| "Invalid worker response".into())
}
