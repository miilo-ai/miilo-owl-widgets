use crate::{
    Result,
    domain::connection::{Config, Endpoint},
};
use chrono::{DateTime, Utc};
use serde_json::Value;
pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}
pub trait StoreLease {}
pub trait SnapshotStore {
    fn lock(&self) -> Result<Box<dyn StoreLease>>;
    fn load(&self) -> Result<Config>;
    fn signature(&self, config: &Config) -> Result<Value>;
    fn cached(&self) -> Result<Value>;
    fn write(&self, name: &str, value: &Value) -> Result<()>;
    fn invalidate(&self) -> Result<()>;
}
pub trait Transport: Sync {
    fn request(&self, endpoint: &Endpoint, path: &str, body: Option<&Value>) -> Result<Value>;
    fn snapshots(&self, endpoints: &[Endpoint]) -> Vec<Result<Value>> {
        endpoints
            .iter()
            .map(|e| self.request(e, "/v1/snapshot", None))
            .collect()
    }
}
