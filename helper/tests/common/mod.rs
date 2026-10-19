#![allow(dead_code)]
use miilo_owl::{
    Result,
    config::{Config, Endpoint, Store},
    transport::Transport,
};
use serde_json::{Value, json};
use std::sync::Mutex;

pub fn fixture() -> Value {
    json!({"protocolVersion":"0.1","providers":[{"id":"factory","name":"Factory"}],
    "workItems":[{"id":"task-1","providerId":"factory","title":"Review recovery","kind":"task",
    "revision":"r1","lifecycle":"waiting","phase":"reviewing","attention":"approval",
    "actions":[{"id":"approve","label":"Approve","available":true,"risk":"bounded","confirmation":"explicit","inputSchema":null}]}],
    "actors":[],"attention":[],"attempts":[],"evidence":[]})
}
pub fn endpoint() -> Endpoint {
    Endpoint {
        paired_host: None,
        token_file: None,
        id: "home".into(),
        url: "http://127.0.0.1:3700".into(),
        token: "private-token".into(),
        providers: vec!["factory".into()],
        allow_actions: true,
        local_snapshot_file: None,
        actor_names: Default::default(),
    }
}
pub fn store(dir: &std::path::Path, endpoints: Vec<Endpoint>) -> Store {
    let s = Store {
        directory: dir.join("config"),
    };
    s.save(&Config {
        version: 1,
        endpoints,
    })
    .unwrap();
    s
}
pub struct Fake {
    pub raw: Mutex<Value>,
    pub calls: Mutex<Vec<(String, String, Option<Value>)>>,
    pub receipt: Mutex<Result<Value>>,
}
impl Default for Fake {
    fn default() -> Self {
        Self {
            raw: Mutex::new(fixture()),
            calls: Mutex::new(vec![]),
            receipt: Mutex::new(Ok(json!({"status":"accepted"}))),
        }
    }
}
impl Transport for Fake {
    fn request(&self, e: &Endpoint, path: &str, body: Option<&Value>) -> Result<Value> {
        self.calls
            .lock()
            .unwrap()
            .push((e.id.clone(), path.into(), body.cloned()));
        if e.id == "offline" {
            return Err("secret network details".into());
        }
        match path {
            "/v1/snapshot" => Ok(self.raw.lock().unwrap().clone()),
            "/v1/explanations" => Ok(
                json!({"revision":body.unwrap()["expectedRevision"],"spokenSummary":"Recovery passed."}),
            ),
            _ => self.receipt.lock().unwrap().clone(),
        }
    }
}
