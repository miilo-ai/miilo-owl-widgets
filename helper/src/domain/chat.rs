//! Minimal, bounded evidence projection for text chat. No transcript/extension transport.
use crate::Result;
use miilo_adapter_sdk::{ChatReference, ChatRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChatConfig {
    pub version: u32,
    pub base_url: String,
    pub model: String,
    pub api_key_file: Option<String>,
    pub consent: bool,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextWork {
    pub reference: ChatReference,
    pub summary: String,
    pub phase: String,
    pub attention: String,
    pub evidence: Vec<ContextEvidence>,
}
#[derive(Serialize)]
pub struct ContextEvidence {
    pub outcome: String,
    pub summary: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatContext {
    pub status: String,
    pub observed_at: f64,
    pub work: Vec<ContextWork>,
    pub omitted: usize,
}
fn text(value: &Value, name: &str, limit: usize) -> String {
    value[name]
        .as_str()
        .unwrap_or("")
        .chars()
        .take(limit)
        .collect()
}
pub fn context(data: &Value, request: &ChatRequest) -> Result<ChatContext> {
    request.validate().map_err(str::to_owned)?;
    let all = data["snapshot"]["workItems"]
        .as_array()
        .ok_or("No observation available")?;
    let selected: Vec<_> = if let Some(focus) = &request.focus {
        let work = all
            .iter()
            .find(|w| w["_key"] == focus.work_key)
            .ok_or("Selected workflow unavailable; refresh")?;
        if work["_revisionKey"] != focus.revision_key {
            return Err("Selected workflow changed; refresh".into());
        }
        vec![work]
    } else {
        all.iter().take(20).collect()
    };
    let work = selected
        .into_iter()
        .map(|w| {
            let evidence = data["snapshot"]["evidence"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|e| {
                    e["providerId"] == w["providerId"]
                        && w["_evidenceKeys"]
                            .as_array()
                            .is_some_and(|ks| ks.contains(&e["_key"]))
                })
                .take(8)
                .map(|e| ContextEvidence {
                    outcome: text(e, "outcome", 40),
                    summary: text(e, "summary", 600),
                })
                .collect();
            ContextWork {
                reference: ChatReference {
                    work_key: text(w, "_key", 128),
                    revision_key: text(w, "_revisionKey", 128),
                    title: text(w, "title", 200),
                },
                summary: text(w, "summary", 600),
                phase: text(w, "phase", 40),
                attention: text(w, "attention", 40),
                evidence,
            }
        })
        .collect::<Vec<_>>();
    let omitted = if request.focus.is_some() {
        0
    } else {
        all.len().saturating_sub(work.len())
    };
    Ok(ChatContext {
        status: text(data, "status", 40),
        observed_at: data["observedAt"].as_f64().unwrap_or(0.0),
        work,
        omitted,
    })
}
