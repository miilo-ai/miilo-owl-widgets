pub mod carrier;
pub mod config;
use chrono::Utc;
use miilo_adapter_sdk::telemetry::{CompletedOperation, Operation};
use miilo_telemetry::{Config, Telemetry};
pub use miilo_telemetry::{Context, ErrorKind, Outcome};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct Host {
    runtime: Telemetry,
    path: PathBuf,
    revision: Vec<u8>,
}
impl Host {
    pub fn load(path: &Path) -> Result<Option<Arc<Self>>, &'static str> {
        let Some((options, revision)) = config::load(path)? else {
            return Ok(None);
        };
        let config = Config::new(&options.endpoint, config::token(&options)?)
            .map_err(|_| "Invalid telemetry endpoint or credential.")?;
        Ok(Some(Arc::new(Self {
            runtime: Telemetry::start(config)?,
            path: path.into(),
            revision,
        })))
    }
    pub fn operation(self: &Arc<Self>, operation: Operation, parent: Context) -> Pending {
        Pending {
            host: self.clone(),
            operation,
            parent,
            started_at: Utc::now(),
            finished: false,
        }
    }
    pub fn chat(self: &Arc<Self>, parent: Context) -> Pending {
        self.operation(
            Operation::ExecuteTool {
                name: "chat_request".into(),
            },
            parent,
        )
    }
    pub fn transcribe(self: &Arc<Self>) -> Pending {
        self.operation(
            Operation::ExecuteTool {
                name: "transcribe".into(),
            },
            Context::new(),
        )
    }
    pub fn enabled(&self) -> bool {
        config::private_bytes(&self.path).is_ok_and(|bytes| bytes == self.revision)
    }
    pub fn shutdown(&self) {
        let _ = self.runtime.shutdown();
    }
}
pub struct Pending {
    host: Arc<Host>,
    operation: Operation,
    parent: Context,
    started_at: chrono::DateTime<Utc>,
    finished: bool,
}
impl Pending {
    fn record(&self, outcome: Outcome) -> Option<String> {
        if !self.host.enabled() {
            return None;
        }
        self.host
            .runtime
            .record(
                &CompletedOperation {
                    operation: self.operation.clone(),
                    provider: None,
                    started_at: self.started_at,
                    ended_at: Utc::now(),
                    outcome,
                },
                &self.parent,
            )
            .ok()
            .flatten()
            .and_then(|value| carrier::encode(&value))
    }
    pub fn finish(mut self, outcome: Outcome) -> Option<String> {
        self.finished = true;
        self.record(outcome)
    }
}
impl Drop for Pending {
    fn drop(&mut self) {
        if !self.finished {
            self.record(Outcome::Cancelled);
        }
    }
}
impl Drop for Host {
    fn drop(&mut self) {
        let _ = self.runtime.shutdown();
    }
}
pub fn validate_endpoint(endpoint: &str, token: Option<String>) -> Result<(), &'static str> {
    Config::new(endpoint, token)
        .map(|_| ())
        .map_err(|_| "Invalid telemetry endpoint or credential.")
}

pub fn configured(path: &Path) -> bool {
    config::load(path)
        .ok()
        .flatten()
        .is_some_and(|(options, _)| {
            config::token(&options)
                .is_ok_and(|token| validate_endpoint(&options.endpoint, token).is_ok())
        })
}
