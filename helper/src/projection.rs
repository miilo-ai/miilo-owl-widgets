use crate::{
    Result,
    domain::{connection::Endpoint, projection},
};
pub use projection::{COLLECTIONS, empty, valid_records, web_url};
use serde_json::Value;
pub fn supported(endpoint: &Endpoint, action: &Value) -> bool {
    projection::supported_at(endpoint, action, chrono::Utc::now())
}
pub fn project(endpoint: &Endpoint, raw: &Value) -> Result<Value> {
    projection::project_at(endpoint, raw, chrono::Utc::now())
}
