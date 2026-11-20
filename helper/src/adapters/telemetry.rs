use crate::{Result, config::Store};
use miilo_session_telemetry::{
    Host,
    config::{Options, token},
};
use serde_json::{Value, json};
use std::sync::Arc;
pub fn load(store: &Store) -> Option<Arc<Host>> {
    match Host::load(&store.directory.join("telemetry.json")) {
        Ok(host) => host,
        Err(_) => {
            eprintln!("Optional telemetry is unavailable; continuing without export.");
            None
        }
    }
}
pub fn dispatch(store: &Store, args: &[&str]) -> Result<Value> {
    let path = store.directory.join("telemetry.json");
    match args {
        ["status"] => Ok(json!({"enabled":miilo_session_telemetry::configured(&path),"configPath":path})),
        ["disable"] => {
            store.write("telemetry.json", &json!({"enabled":false,"endpoint":"","tokenFile":null}))?;
            Ok(json!({"enabled":false}))
        }
        ["configure", "--consent", "--endpoint", endpoint, rest @ ..] => {
            let token_file = match rest { [] => None, ["--token-file", path] => Some((*path).into()), _ => return Err("Invalid telemetry options".into()) };
            let options = Options { enabled:true, endpoint:(*endpoint).into(), token_file };
            miilo_telemetry_config(&options)?;
            store.write("telemetry.json", &serde_json::to_value(options).map_err(|_| "Invalid telemetry configuration")?)?;
            Ok(json!({"enabled":true}))
        }
        _ => Err("Use telemetry status, disable, or configure --consent --endpoint URL [--token-file PATH]".into()),
    }
}
fn miilo_telemetry_config(options: &Options) -> Result<()> {
    let value = token(options).map_err(str::to_owned)?;
    miilo_session_telemetry::validate_endpoint(&options.endpoint, value).map_err(str::to_owned)
}
