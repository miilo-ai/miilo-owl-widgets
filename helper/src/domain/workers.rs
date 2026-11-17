use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkerConfig {
    pub version: u32,
    pub endpoint: String,
    pub cli_executable: String,
    pub collection: String,
    pub application: String,
    pub workstation: String,
    pub gateway: String,
}
pub fn identifier(value: &str, min: usize) -> bool {
    value.len() >= min
        && value.len() <= 80
        && value
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_alphanumeric())
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c))
}
pub fn validate(config: &WorkerConfig) -> crate::Result<()> {
    let url = url::Url::parse(&config.endpoint).map_err(|_| "Invalid worker endpoint")?;
    let local = matches!(url.host_str(), Some("127.0.0.1" | "localhost" | "[::1]"));
    if config.version != 1
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.host_str().is_none()
        || !(url.scheme() == "https" || local && url.scheme() == "http")
        || !std::path::Path::new(&config.cli_executable).is_absolute()
        || config.cli_executable.chars().any(char::is_control)
        || !config
            .collection
            .starts_with("/org/freedesktop/secrets/collection/")
        || config.collection.len() > 200
        || [&config.application, &config.workstation, &config.gateway]
            .iter()
            .any(|v| !identifier(v, 1))
    {
        return Err(
            "Worker control needs a secure endpoint, absolute CLI and explicit keyring reference"
                .into(),
        );
    }
    Ok(())
}
