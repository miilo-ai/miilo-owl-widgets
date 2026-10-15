//! Provider-neutral application logic; filesystem and HTTP are replaceable ports.
pub mod action_policy;
pub mod artifacts;
pub mod config;
pub mod display_names;
pub mod graph;
pub mod operations;
pub mod projection;
pub mod setup;
pub mod snapshot;
pub mod transport;

pub type Result<T> = std::result::Result<T, String>;

pub fn key(value: &serde_json::Value) -> String {
    use sha2::{Digest, Sha256};
    // Match Python's compact JSON/ensure_ascii encoding for stable existing keys.
    fn canonical(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => {
                let encoded = serde_json::to_string(s).expect("string serialization");
                encoded
                    .encode_utf16()
                    .map(|c| {
                        if c >= 127 {
                            format!("\\u{c:04x}")
                        } else {
                            char::from_u32(c.into()).unwrap().to_string()
                        }
                    })
                    .collect()
            }
            serde_json::Value::Array(a) => format!(
                "[{}]",
                a.iter().map(canonical).collect::<Vec<_>>().join(",")
            ),
            serde_json::Value::Object(o) => format!(
                "{{{}}}",
                o.iter()
                    .map(|(k, v)| format!("{}:{}", canonical(&k.clone().into()), canonical(v)))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            _ => value.to_string(),
        }
    }
    format!("{:x}", Sha256::digest(canonical(value)))[..32].to_owned()
}
