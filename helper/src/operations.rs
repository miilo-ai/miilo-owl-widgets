use crate::{
    Result,
    adapters::store::SystemClock,
    application::operations,
    config::{Endpoint, Store},
    transport::Transport,
};
use serde_json::Value;
pub fn target(
    store: &Store,
    source: &dyn Transport,
    item: &str,
    revision: &str,
) -> Result<(Endpoint, Value)> {
    operations::target(store, source, item, revision, &SystemClock)
}
pub fn explain(store: &Store, source: &dyn Transport, item: &str, revision: &str) -> Result<Value> {
    operations::explain(store, source, item, revision, &SystemClock)
}
pub fn action(
    store: &Store,
    source: &dyn Transport,
    item: &str,
    action: &str,
    revision: &str,
) -> Result<Value> {
    operations::action(store, source, item, action, revision, &SystemClock)
}
