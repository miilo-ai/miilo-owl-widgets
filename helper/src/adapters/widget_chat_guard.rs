use crate::{
    Result,
    config::{Store, private_file, reject_link},
};
use std::{
    fs::{File, OpenOptions},
    io::Read,
    os::unix::fs::OpenOptionsExt,
};

pub fn claim(store: &Store, id: &str) -> Result<File> {
    store.prepare()?;
    let path = store.directory.join("widget-chat.lock");
    reject_link(&path)?;
    let guard = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| "Cannot protect chat session")?;
    fs2::FileExt::try_lock_exclusive(&guard).map_err(|_| "Another reply is running")?;
    let receipt = store.directory.join("widget-chat-request.json");
    let mut ids = Vec::<String>::new();
    if receipt
        .try_exists()
        .map_err(|_| "Cannot inspect chat session")?
    {
        let mut bytes = Vec::new();
        private_file(&receipt)?
            .take(64_001)
            .read_to_end(&mut bytes)
            .map_err(|_| "Cannot inspect chat session")?;
        let value: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|_| "Invalid chat session receipt")?;
        ids = serde_json::from_value(value["requestIds"].clone())
            .map_err(|_| "Invalid chat session receipt")?;
        if ids.len() > 128 || ids.iter().any(|v| v.len() > 320) {
            return Err("Invalid chat session receipt".into());
        }
        if ids.iter().any(|seen| seen == id) {
            return Err("Reply interrupted or already finished. Send again to retry.".into());
        }
    }
    ids.push(id.to_owned());
    if ids.len() > 128 {
        ids.remove(0);
    }
    store.write(
        "widget-chat-request.json",
        &serde_json::json!({"requestIds":ids}),
    )?;
    Ok(guard)
}
