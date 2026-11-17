use crate::{Result, domain::workers::WorkerConfig};
use std::{
    io::Read,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

pub fn capture(command: &mut Command, limit: u64, timeout: Duration) -> Result<Vec<u8>> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "Worker utility unavailable")?;
    let output = child.stdout.take().ok_or("Worker output unavailable")?;
    let reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        output
            .take(limit + 1)
            .read_to_end(&mut bytes)
            .map(|_| bytes)
    });
    let deadline = Instant::now() + timeout;
    let success = loop {
        if Instant::now() > deadline {
            let _ = child.kill();
            let _ = child.wait();
            break false;
        }
        match child.try_wait() {
            Ok(Some(status)) => break status.success(),
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                break false;
            }
        }
    };
    let bytes = reader
        .join()
        .map_err(|_| "Worker output failed")?
        .map_err(|_| "Worker output failed")?;
    if !success || bytes.len() > limit as usize {
        return Err("Worker utility failed or timed out".into());
    }
    Ok(bytes)
}
pub fn credential(config: &WorkerConfig) -> Result<String> {
    if !cfg!(target_os = "linux") {
        return Err("Native worker keyring access requires Linux".into());
    }
    let unlocked = capture(
        Command::new("gdbus").args([
            "call",
            "--session",
            "--dest",
            "org.freedesktop.secrets",
            "--object-path",
            &config.collection,
            "--method",
            "org.freedesktop.DBus.Properties.Get",
            "org.freedesktop.Secret.Collection",
            "Locked",
        ]),
        8192,
        Duration::from_secs(3),
    )?;
    if !String::from_utf8_lossy(&unlocked).contains("false") {
        return Err("Worker keyring is locked".into());
    }
    let bytes = capture(
        Command::new("secret-tool").args([
            "lookup",
            "application",
            &config.application,
            "workstation",
            &config.workstation,
            "gateway",
            &config.gateway,
        ]),
        8192,
        Duration::from_secs(3),
    )?;
    let token = String::from_utf8(bytes)
        .map_err(|_| "Invalid worker credential")?
        .trim()
        .to_owned();
    if token.len() < 16 || token.chars().any(char::is_whitespace) {
        return Err("Invalid worker credential".into());
    }
    Ok(token)
}
