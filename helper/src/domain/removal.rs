use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub schema_version: u8,
    pub widget_directory: String,
    pub sha256: String,
    pub helper: String,
}
#[derive(Clone, Serialize)]
pub struct Plan {
    pub status: String,
    pub surface: String,
    pub plan_id: String,
    pub remove: Vec<String>,
    pub retained: Vec<String>,
}
impl Plan {
    pub fn widgets(receipt: &Receipt, present: bool) -> Self {
        let id = format!(
            "{:x}",
            Sha256::digest(format!(
                "{}\n{}\n{present}",
                receipt.widget_directory, receipt.sha256
            ))
        );
        Self {
            status: "planned".into(),
            surface: "widgets".into(),
            plan_id: id,
            remove: if present {
                vec![format!("{}/70-miilo-fleet.owl", receipt.widget_directory)]
            } else {
                vec![]
            },
            retained: vec![
                receipt.helper.clone(),
                "Private settings, tokens, caches, installation receipt, backups and other widgets"
                    .into(),
                "Independent observers and source agents are not stopped".into(),
            ],
        }
    }
}
