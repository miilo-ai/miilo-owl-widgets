use crate::{
    Result,
    application::ports::{Clock, SnapshotStore, StoreLease},
    config::{Store, private_file},
    domain::connection::Config,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
impl SnapshotStore for Store {
    fn lock(&self) -> Result<Box<dyn StoreLease>> {
        Ok(Box::new(Store::lock(self)?))
    }
    fn load(&self) -> Result<Config> {
        Store::load(self)
    }
    fn signature(&self, config: &Config) -> Result<Value> {
        crate::local_snapshot::cache_signature(config)
    }
    fn cached(&self) -> Result<Value> {
        let file = private_file(&self.directory.join("snapshot-cache.json"))?;
        serde_json::from_reader(std::io::Read::take(file, 20_000_001))
            .map_err(|_| "Invalid cache".into())
    }
    fn write(&self, name: &str, value: &Value) -> Result<()> {
        Store::write(self, name, value)
    }
    fn invalidate(&self) -> Result<()> {
        Store::invalidate(self)
    }
}

impl StoreLease for std::fs::File {}
