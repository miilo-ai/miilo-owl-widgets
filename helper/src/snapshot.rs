use crate::{
    Result,
    adapters::store::SystemClock,
    application::{self, ports::Clock},
    config::{Config, Store},
    transport::Transport,
};
use serde_json::Value;
pub fn now() -> f64 {
    application::snapshot::seconds(SystemClock.now())
}
pub fn collect(config: &Config, source: &dyn Transport) -> Value {
    application::snapshot::collect(config, source, SystemClock.now())
}
pub fn snapshot(store: &Store, source: &dyn Transport, force: bool) -> Result<Value> {
    application::snapshot::snapshot(store, source, &SystemClock, force)
}
