use crate::domain::{action_policy, connection::Endpoint};
use serde_json::Value;
pub fn unavailable_reason(endpoint: &Endpoint, action: &Value) -> Option<&'static str> {
    action_policy::unavailable_reason_at(endpoint, action, chrono::Utc::now())
}
