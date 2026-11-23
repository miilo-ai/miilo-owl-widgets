//! Android's line stream replaces its value per line; emit cumulative text.
//! NUL-delimited stdin keeps arbitrary questions out of shell syntax and files.
use crate::{Result, config::Store};
use miilo_adapter_sdk::ChatRequest;
use serde_json::{Value, json};
use std::io::{self, Read, Write};

pub fn request(input: &[u8]) -> Result<(ChatRequest, bool)> {
    if input.len() > 64_000 || !input.ends_with(&[0]) {
        return Err("Invalid widget chat input".into());
    }
    let fields = input[..input.len() - 1]
        .split(|b| *b == 0)
        .map(std::str::from_utf8)
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| "Invalid widget chat text")?;
    if fields.len() < 5 || fields.len() > 21 || (fields.len() - 5) % 2 != 0 {
        return Err("Invalid widget chat fields".into());
    }
    let [id, text, share, key, revision] = fields[..5] else {
        unreachable!()
    };
    if !matches!(share, "true" | "false") {
        return Err("Invalid context choice".into());
    }
    let history = fields[5..]
        .chunks_exact(2)
        .map(|pair| json!({"role":pair[0],"text":pair[1]}))
        .collect::<Vec<_>>();
    let focus = if share == "true" && !key.is_empty() {
        json!({"workKey":key,"revisionKey":revision})
    } else {
        Value::Null
    };
    let request: ChatRequest = serde_json::from_value(json!({
        "protocolVersion":"0.1", "requestId":id, "text":text,
        "focus":focus, "history":history
    }))
    .map_err(|_| "Invalid widget chat request")?;
    request.validate().map_err(str::to_owned)?;
    Ok((request, share == "true"))
}

pub fn run(store: &Store) -> Result<()> {
    let mut input = Vec::new();
    io::stdin()
        .take(64_001)
        .read_to_end(&mut input)
        .map_err(|_| "Cannot read widget chat")?;
    let (request, share) = match request(&input) {
        Ok(value) => value,
        Err(message) => {
            let id = input
                .split(|b| *b == 0)
                .next()
                .and_then(|b| std::str::from_utf8(b).ok())
                .unwrap_or("");
            return emit(&json!({"requestId":id.chars().take(80).collect::<String>(),
                "text":"","done":true,"error":message}));
        }
    };
    let id = request.request_id.clone();
    let mut answer = String::new();
    let result = (|| {
        // Do not resubmit a paid model call when the host restarts a stream.
        // Retain bounded request identifiers, never prompts or answers.
        let _guard = super::widget_chat_guard::claim(store, &id)?;
        let config = super::chat_config::load(store)?;
        let snapshot = if share {
            crate::snapshot::snapshot(store, &crate::transport::Http::new()?, true)?
        } else {
            json!({"status":"context_not_shared","observedAt":0,
                "snapshot":{"workItems":[],"evidence":[]}})
        };
        crate::application::chat::run(
            &super::chat_http::OpenAiChat::new()?,
            &config,
            &request,
            &snapshot,
            &mut |event| {
                let event = serde_json::to_value(event).map_err(|_| "Cannot encode reply")?;
                if let Some(text) = event["text"].as_str() {
                    answer.push_str(text);
                }
                emit(&json!({"requestId":id,"text":answer,
                    "done":event["type"] == "done","error":null}))
            },
        )
    })();
    if let Err(message) = result {
        emit(&json!({"requestId":id,"text":answer,"done":true,"error":message}))?;
    }
    Ok(())
}
fn emit(value: &Value) -> Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer(&mut out, value).map_err(|_| "Chat consumer disconnected")?;
    writeln!(out)
        .and_then(|_| out.flush())
        .map_err(|_| "Chat consumer disconnected".into())
}
