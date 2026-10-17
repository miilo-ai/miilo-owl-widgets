use crate::{Result, config::Store};
use miilo_adapter_sdk::{ChatEvent, ChatRequest, MIILO_PROTOCOL_VERSION};
use serde_json::{Value, json};
use std::io::{self, Read, Write};
pub fn run(store: &Store, preview: bool) -> Result<()> {
    let mut input = vec![];
    io::stdin()
        .take(64_001)
        .read_to_end(&mut input)
        .map_err(|_| "Cannot read chat request")?;
    if input.len() > 64_000 {
        return Err("Chat request too large".into());
    }
    let request: ChatRequest =
        serde_json::from_slice(&input).map_err(|_| "Invalid chat request")?;
    request.validate().map_err(str::to_owned)?;
    let mut sequence = 0;
    let mut emit = |event: ChatEvent| -> Result<()> {
        let value = serde_json::to_value(event).map_err(|_| "Cannot encode chat event")?;
        sequence = value["sequence"].as_u64().unwrap_or(0) + 1;
        println!("{value}");
        io::stdout()
            .flush()
            .map_err(|_| "Chat consumer disconnected".into())
    };
    let result = (|| {
        let fresh = crate::snapshot::snapshot(store, &crate::transport::Http::new()?, true)?;
        if preview {
            let context = crate::domain::chat::context(&fresh, &request)?;
            println!(
                "{}",
                serde_json::to_string(&context).map_err(|_| "Cannot encode context")?
            );
            return Ok(());
        }
        let config = super::chat_config::load(store)?;
        crate::application::chat::run(
            &super::chat_http::OpenAiChat::new()?,
            &config,
            &request,
            &fresh,
            &mut emit,
        )
    })();
    if let Err(message) = result {
        let event = ChatEvent::Error {
            protocol_version: MIILO_PROTOCOL_VERSION.into(),
            request_id: request.request_id,
            sequence,
            code: "chat_unavailable".into(),
            message,
        };
        println!(
            "{}",
            serde_json::to_string(&event).map_err(|_| "Cannot encode chat error")?
        );
    }
    Ok(())
}
pub fn voice_unavailable() -> Value {
    json!({"status":"not_implemented","code":"not_implemented","specId":"VOICE-001",
        "message":"Launcher speech is deferred; use text chat.","spec":".specify/specs/004-launcher-voice/spec.md"})
}
