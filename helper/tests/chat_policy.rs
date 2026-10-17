use miilo_adapter_sdk::{ChatEvent, ChatRequest};
use miilo_owl::{
    Result,
    application::chat::{self, ChatBackend},
    domain::chat::{ChatConfig, ChatContext, context},
};
use serde_json::json;
use std::cell::Cell;
fn request() -> ChatRequest {
    serde_json::from_value(json!({"protocolVersion":"0.1","requestId":"turn-1","text":"Explain this","focus":{"workKey":"w","revisionKey":"r"}})).unwrap()
}
fn config() -> ChatConfig {
    serde_json::from_value(json!({"version":1,"baseUrl":"https://example.invalid/v1","model":"example","consent":true})).unwrap()
}
fn snapshot() -> serde_json::Value {
    json!({"status":"partial","observedAt":1234.0,"snapshot":{"workItems":[{"_key":"w","_revisionKey":"r","title":"Task","summary":"Needs review","phase":"reviewing","attention":"action","providerId":"p","_evidenceKeys":["e"],"extensions":{"secret":"must-not-leak"},"actions":[{"id":"approve"}]}],"evidence":[{"_key":"e","providerId":"p","outcome":"failed","summary":"Test failed"},{"_key":"e","providerId":"other","outcome":"passed","summary":"wrong provider"}]}})
}
struct Backend {
    calls: Cell<usize>,
    fail: bool,
}
impl ChatBackend for Backend {
    fn stream(
        &self,
        _: &ChatConfig,
        _: &ChatRequest,
        ctx: &ChatContext,
        emit: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()> {
        self.calls.set(self.calls.get() + 1);
        assert_eq!(ctx.status, "partial");
        emit("Test failed.")?;
        if self.fail {
            Err("Disconnected".into())
        } else {
            Ok(())
        }
    }
}
#[test]
fn context_is_revision_bound_provider_isolated_and_minimal() {
    let mut q = request();
    let ctx = context(&snapshot(), &q).unwrap();
    assert_eq!(ctx.work[0].evidence.len(), 1);
    let body = serde_json::to_string(&ctx).unwrap();
    assert!(!body.contains("must-not-leak"));
    assert!(!body.contains("approve"));
    assert!(!body.contains("wrong provider"));
    q.focus.as_mut().unwrap().revision_key = "stale".into();
    assert!(context(&snapshot(), &q).is_err());
}
#[test]
fn consent_and_stale_target_prevent_inference() {
    let backend = Backend {
        calls: Cell::new(0),
        fail: false,
    };
    let mut cfg = config();
    cfg.consent = false;
    assert!(chat::run(&backend, &cfg, &request(), &snapshot(), &mut |_| Ok(())).is_err());
    cfg.consent = true;
    let mut q = request();
    q.focus.as_mut().unwrap().work_key = "missing".into();
    assert!(chat::run(&backend, &cfg, &q, &snapshot(), &mut |_| Ok(())).is_err());
    assert_eq!(backend.calls.get(), 0);
}
#[test]
fn successful_stream_is_ordered_but_interruption_cannot_complete() {
    let backend = Backend {
        calls: Cell::new(0),
        fail: false,
    };
    let mut events = vec![];
    chat::run(&backend, &config(), &request(), &snapshot(), &mut |e| {
        events.push(e);
        Ok(())
    })
    .unwrap();
    assert!(matches!(&events[0],ChatEvent::Context{sequence:0,status,..} if status=="partial"));
    assert!(matches!(&events[2], ChatEvent::Done { sequence: 2, .. }));
    let failing = Backend {
        calls: Cell::new(0),
        fail: true,
    };
    events.clear();
    assert!(
        chat::run(&failing, &config(), &request(), &snapshot(), &mut |e| {
            events.push(e);
            Ok(())
        })
        .is_err()
    );
    assert!(!events.iter().any(|e| matches!(e, ChatEvent::Done { .. })));
}
#[test]
fn consumer_cancellation_stops_inference_output() {
    let backend = Backend {
        calls: Cell::new(0),
        fail: false,
    };
    assert!(
        chat::run(&backend, &config(), &request(), &snapshot(), &mut |_| Err(
            "closed".into()
        ))
        .is_err()
    );
    assert_eq!(backend.calls.get(), 0);
}
#[test]
fn voice_placeholder_is_explicit_and_spec_linked() {
    let value = miilo_owl::adapters::chat_command::voice_unavailable();
    assert_eq!(value["status"], "not_implemented");
    assert_eq!(value["specId"], "VOICE-001");
}
