//! Explanation projected from an existing observation, without a model or mutation.
use crate::Result;
use serde_json::{Value, json};
pub fn explain(snapshot: &Value, request: &Value) -> Result<Value> {
    let work = snapshot["workItems"]
        .as_array()
        .ok_or("No observed work")?
        .iter()
        .find(|w| w["providerId"] == request["providerId"] && w["id"] == request["workItemId"])
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
