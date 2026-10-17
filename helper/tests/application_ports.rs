use chrono::{DateTime, Utc};
use miilo_owl::{
    Result,
    application::{
        ports::{Clock, SnapshotStore, StoreLease, Transport},
        snapshot,
    },
    domain::connection::{Config, Endpoint},
};
use serde_json::{Value, json};
use std::{
    cell::{Cell, RefCell},
    sync::atomic::{AtomicUsize, Ordering},
};
struct Lease;
impl StoreLease for Lease {}
struct Memory {
    config: Config,
    cached: RefCell<Option<Value>>,
}
impl SnapshotStore for Memory {
    fn lock(&self) -> Result<Box<dyn StoreLease>> {
        Ok(Box::new(Lease))
    }
    fn load(&self) -> Result<Config> {
        Ok(self.config.clone())
    }
    fn signature(&self, _: &Config) -> Result<Value> {
        Ok(json!("current"))
    }
    fn cached(&self) -> Result<Value> {
        self.cached.borrow().clone().ok_or("No cache".into())
    }
    fn write(&self, _: &str, value: &Value) -> Result<()> {
        *self.cached.borrow_mut() = Some(value.clone());
        Ok(())
    }
    fn invalidate(&self) -> Result<()> {
        self.cached.borrow_mut().take();
        Ok(())
    }
}
struct Fixed(Cell<i64>);
impl Clock for Fixed {
    fn now(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.0.get(), 0).unwrap()
    }
}
struct Source {
    calls: AtomicUsize,
    failing: bool,
}
impl Transport for Source {
    fn request(&self, _: &Endpoint, _: &str, _: Option<&Value>) -> Result<Value> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.failing {
            return Err("Unavailable".into());
        }
        Ok(
            json!({"protocolVersion":"0.1","providers":[{"id":"p","name":"Provider"}],"workItems":[],"actors":[],"attempts":[],"artifacts":[],"evidence":[],"attention":[]}),
        )
    }
}
fn memory() -> Memory {
    Memory { config: serde_json::from_value(json!({"version":1,"endpoints":[{"id":"example","url":"https://example.invalid","providers":["p"],"allowActions":false}]})).unwrap(), cached: RefCell::new(None) }
}
#[test]
fn snapshot_use_case_runs_with_memory_ports_and_explicit_time() {
    let store = memory();
    let clock = Fixed(Cell::new(1_700_000_000));
    let source = Source {
        calls: AtomicUsize::new(0),
        failing: false,
    };
    let first = snapshot::snapshot(&store, &source, &clock, false).unwrap();
    assert_eq!(first["status"], "ready");
    assert_eq!(first["observedAt"], 1_700_000_000.0);
    snapshot::snapshot(&store, &source, &clock, false).unwrap();
    assert_eq!(source.calls.load(Ordering::SeqCst), 1);
    clock.0.set(1_700_000_009);
    snapshot::snapshot(&store, &source, &clock, false).unwrap();
    assert_eq!(source.calls.load(Ordering::SeqCst), 2);
}
#[test]
fn outage_never_becomes_healthy_cached_state() {
    let store = memory();
    let clock = Fixed(Cell::new(1_700_000_000));
    let source = Source {
        calls: AtomicUsize::new(0),
        failing: true,
    };
    let result = snapshot::snapshot(&store, &source, &clock, true).unwrap();
    assert_eq!(result["status"], "partial");
    assert_eq!(result["errors"].as_array().unwrap().len(), 1);
    assert!(
        result["snapshot"]["providers"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
