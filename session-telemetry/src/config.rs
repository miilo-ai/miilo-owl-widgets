use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::Read,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::{Path, PathBuf},
};
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Options {
    pub enabled: bool,
    pub endpoint: String,
    pub token_file: Option<PathBuf>,
}
pub fn private_bytes(path: &Path) -> Result<Vec<u8>, &'static str> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| "Cannot open private telemetry configuration.")?;
    let meta = file
        .metadata()
        .map_err(|_| "Cannot inspect telemetry configuration.")?;
    if !meta.is_file() || meta.mode() & 0o077 != 0 || meta.uid() != unsafe { libc::geteuid() } {
        return Err("Telemetry configuration must be a private file owned by this user.");
    }
    let mut bytes = vec![];
    file.take(8193)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read telemetry configuration.")?;
    if bytes.len() > 8192 {
        return Err("Telemetry configuration is too large.");
    }
    Ok(bytes)
}
pub fn load(path: &Path) -> Result<Option<(Options, Vec<u8>)>, &'static str> {
    if !path
        .try_exists()
        .map_err(|_| "Cannot inspect telemetry configuration.")?
    {
        return Ok(None);
    }
    let bytes = private_bytes(path)?;
    let options: Options =
        serde_json::from_slice(&bytes).map_err(|_| "Invalid telemetry configuration.")?;
    Ok(options.enabled.then_some((options, bytes)))
}
pub fn token(options: &Options) -> Result<Option<String>, &'static str> {
    options
        .token_file
        .as_ref()
        .map(|path| {
            if !path.is_absolute() {
                return Err("Telemetry credential file must be absolute.");
            }
            let bytes = private_bytes(path)?;
            let text = std::str::from_utf8(&bytes)
                .map_err(|_| "Invalid telemetry credential.")?
                .trim();
            if text.is_empty() || text.len() > 4096 {
                return Err("Invalid telemetry credential.");
            }
            Ok(text.to_owned())
        })
        .transpose()
}
