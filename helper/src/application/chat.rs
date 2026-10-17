use crate::{
    Result,
    domain::chat::{ChatConfig, ChatContext, context},
};
use miilo_adapter_sdk::{ChatEvent, ChatRequest, MIILO_PROTOCOL_VERSION};
use serde_json::Value;
pub trait ChatBackend {
    fn stream(
        &self,
        config: &ChatConfig,
        request: &ChatRequest,
        context: &ChatContext,
        delta: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()>;
}
pub fn run(
    backend: &dyn ChatBackend,
    config: &ChatConfig,
    request: &ChatRequest,
    fresh_snapshot: &Value,
    emit: &mut dyn FnMut(ChatEvent) -> Result<()>,
) -> Result<()> {
    if config.version != 1 || !config.consent {
        return Err("Chat requires explicit endpoint/context consent".into());
    }
    let context = context(fresh_snapshot, request)?;
    let mut sequence = 0;
    let envelope = || {
        (
            MIILO_PROTOCOL_VERSION.to_owned(),
            request.request_id.clone(),
        )
    };
    let (protocol_version, request_id) = envelope();
    emit(ChatEvent::Context {
        protocol_version,
        request_id,
        sequence,
        status: context.status.clone(),
        references: context.work.iter().map(|w| w.reference.clone()).collect(),
        omitted: context.omitted,
    })?;
    sequence += 1;
    let mut output = String::new();
    backend.stream(config, request, &context, &mut |text| {
        if output.len() + text.len() > 64_000 {
            return Err("Chat response exceeded the limit".into());
        }
        output.push_str(text);
        let (protocol_version, request_id) = envelope();
        emit(ChatEvent::TextDelta {
            protocol_version,
            request_id,
            sequence,
            text: text.into(),
        })?;
        sequence += 1;
        Ok(())
    })?;
    if output.trim().is_empty() {
        return Err("Chat provider returned no answer".into());
    }
    let (protocol_version, request_id) = envelope();
    emit(ChatEvent::Done {
        protocol_version,
        request_id,
        sequence,
    })
}
