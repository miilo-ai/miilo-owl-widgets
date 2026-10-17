//! Bounded SSE decoding; tool calls and hidden reasoning are never executed/rendered.
use crate::Result;
use serde_json::Value;
use std::io::{BufRead, BufReader, Read};
pub fn read(reader: impl Read, delta: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
    let mut reader = BufReader::new(reader.take(256_001));
    let mut total = 0;
    let mut frame = String::new();
    loop {
        let mut bytes = vec![];
        let size = (&mut reader)
            .take(65_537)
            .read_until(b'\n', &mut bytes)
            .map_err(|_| "Chat stream interrupted")?;
        if size == 0 {
            return Err("Chat stream ended without a completion marker".into());
        }
        total += size;
        if size > 65_536 || total > 256_000 {
            return Err("Chat stream exceeded the limit".into());
        }
        let line = std::str::from_utf8(&bytes)
            .map_err(|_| "Invalid chat stream encoding")?
            .trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            if frame.is_empty() {
                continue;
            }
            let data = std::mem::take(&mut frame);
            if data.trim() == "[DONE]" {
                return Ok(());
            }
            let value: Value =
                serde_json::from_str(&data).map_err(|_| "Invalid chat stream frame")?;
            if value.get("error").is_some() {
                return Err("Chat provider failed".into());
            }
            if let Some(choice) = value["choices"].as_array().and_then(|v| v.first()) {
                if choice["delta"].get("tool_calls").is_some()
                    || choice["delta"].get("function_call").is_some()
                    || matches!(
                        choice["finish_reason"].as_str(),
                        Some("tool_calls" | "function_call" | "length" | "content_filter")
                    )
                {
                    return Err("Chat response incomplete or unsupported".into());
                }
                if let Some(text) = choice["delta"]["content"]
                    .as_str()
                    .filter(|s| !s.is_empty())
                {
                    delta(text)?;
                }
            }
        } else if let Some(data) = line.strip_prefix("data:") {
            if !frame.is_empty() {
                frame.push('\n');
            }
            frame.push_str(data.trim_start());
            if frame.len() > 65_536 {
                return Err("Chat frame exceeded the limit".into());
            }
        }
    }
}
