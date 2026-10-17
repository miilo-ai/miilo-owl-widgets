//! Artifact presentation and task links, scoped by the projected provider identity.
use serde_json::{Value, json};

const PREVIEW_LIMIT: usize = 8_000;

pub fn preview(value: &Value) -> Option<String> {
    let object = value.as_object()?;
    for key in ["fileDiff", "preview", "delta", "diff"] {
        if let Some(text) = object
            .get(key)
            .and_then(Value::as_str)
            .filter(|s| !s.trim().is_empty())
        {
            return Some(text.to_owned());
        }
    }
    object.values().find_map(preview)
}

pub fn present(artifact: &mut Value) {
    let text = preview(&artifact["extensions"]).unwrap_or_default();
    artifact["_preview"] = json!(text.chars().take(PREVIEW_LIMIT).collect::<String>());
    artifact["_previewTruncated"] = json!(text.chars().count() > PREVIEW_LIMIT);
    artifact["_url"] = json!(crate::projection::web_url(&artifact["uri"]));
}

pub fn link(data: &mut Value) {
    use std::collections::HashMap;
    let attempts: HashMap<_, _> = data["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|a| {
            Some((
                (
                    a["providerId"].as_str()?.to_owned(),
                    a["id"].as_str()?.to_owned(),
                ),
                a["taskId"].as_str()?.to_owned(),
            ))
        })
        .collect();
    let mut by_task: HashMap<(String, String), Vec<Value>> = HashMap::new();
    let mut by_reference = HashMap::new();
    for a in data["artifacts"].as_array().unwrap() {
        let (Some(provider), Some(attempt), Some(id)) = (
            a["providerId"].as_str(),
            a["attemptId"].as_str(),
            a["id"].as_str(),
        ) else {
            continue;
        };
        by_reference.insert(
            (provider.to_owned(), attempt.to_owned(), id.to_owned()),
            a["_key"].clone(),
        );
        if let Some(task) = attempts.get(&(provider.to_owned(), attempt.to_owned())) {
            by_task
                .entry((provider.to_owned(), task.clone()))
                .or_default()
                .push(a["_key"].clone());
        }
    }
    for work in data["workItems"].as_array_mut().unwrap() {
        let identity = (
            work["providerId"].as_str().unwrap_or("").to_owned(),
            work["id"].as_str().unwrap_or("").to_owned(),
        );
        work["_artifactKeys"] = json!(by_task.get(&identity).cloned().unwrap_or_default());
    }
    for evidence in data["evidence"].as_array_mut().unwrap() {
        let provider = evidence["providerId"].as_str().unwrap_or("");
        let attempt = evidence["attemptId"].as_str().unwrap_or("");
        let mut keys = vec![];
        let mut missing = 0;
        for id in evidence["artifactIds"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            if let Some(key) =
                by_reference.get(&(provider.to_owned(), attempt.to_owned(), id.to_owned()))
            {
                keys.push(key.clone());
            } else {
                missing += 1;
            }
        }
        evidence["_missingArtifactCount"] = json!(missing);
        evidence["_artifactKeys"] = json!(keys);
    }
}
