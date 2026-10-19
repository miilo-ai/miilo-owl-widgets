use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Clone, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(
        default,
        rename = "pairedHost",
        skip_serializing_if = "Option::is_none"
    )]
    pub paired_host: Option<miilo_pairing::domain::PairedHost>,
    pub id: String,
    pub url: String,
    #[serde(default)]
    pub token: String,
    #[serde(default, rename = "tokenFile", skip_serializing_if = "Option::is_none")]
    pub token_file: Option<PathBuf>,
    #[serde(
        default,
        rename = "localSnapshotFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_snapshot_file: Option<PathBuf>,
    pub providers: Vec<String>,
    #[serde(default, rename = "allowActions")]
    pub allow_actions: bool,
    #[serde(
        default,
        rename = "actorNames",
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub actor_names: std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub endpoints: Vec<Endpoint>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            endpoints: vec![],
        }
    }
}
