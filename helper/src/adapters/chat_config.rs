use crate::{
    Result,
    config::{Store, private_file},
    domain::chat::ChatConfig,
};
use std::{
    io::{self, Read, Write},
    path::Path,
};
pub fn validate(config: &ChatConfig) -> Result<()> {
    let url = url::Url::parse(&config.base_url).map_err(|_| "Invalid chat URL")?;
    let local = matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
    if config.version != 1
        || !config.consent
        || config.model.trim().is_empty()
        || config.model.len() > 200
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.host_str().is_none()
        || !(url.scheme() == "https" || (local && url.scheme() == "http"))
        || !url.path().trim_end_matches('/').ends_with("/v1")
    {
        return Err("Use an HTTPS /v1 endpoint (HTTP is allowed on loopback), a model, and explicit consent".into());
    }
    if config
        .api_key_file
        .as_ref()
        .is_some_and(|p| !Path::new(p).is_absolute())
    {
        return Err("Credential file must be absolute".into());
    }
    Ok(())
}
pub fn credential(config: &ChatConfig) -> Result<Option<String>> {
    let Some(path) = &config.api_key_file else {
        return Ok(None);
    };
    let mut token = String::new();
    private_file(Path::new(path))?
        .take(4097)
        .read_to_string(&mut token)
        .map_err(|_| "Cannot read chat credential")?;
    let token = token.trim();
    if token.is_empty() || token.len() > 4096 || token.chars().any(char::is_whitespace) {
        return Err("Invalid chat credential".into());
    }
    Ok(Some(token.into()))
}
pub fn load(store: &Store) -> Result<ChatConfig> {
    let file = private_file(&store.directory.join("chat.json"))?;
    let config =
        serde_json::from_reader(file.take(16_385)).map_err(|_| "Invalid chat configuration")?;
    validate(&config)?;
    Ok(config)
}
pub fn save(store: &Store, config: &ChatConfig) -> Result<()> {
    validate(config)?;
    credential(config)?;
    store.write(
        "chat.json",
        &serde_json::to_value(config).map_err(|_| "Invalid configuration")?,
    )
}
fn prompt(label: &str) -> Result<String> {
    print!("{label}: ");
    io::stdout().flush().map_err(|_| "Cannot prompt")?;
    let mut text = String::new();
    io::stdin().lock().take(4097).read_line_bounded(&mut text)?;
    Ok(text.trim().into())
}
trait BoundedLine {
    fn read_line_bounded(&mut self, text: &mut String) -> Result<()>;
}
impl<T: Read> BoundedLine for T {
    fn read_line_bounded(&mut self, text: &mut String) -> Result<()> {
        let mut byte = [0];
        while self
            .read(&mut byte)
            .map_err(|_| "Cannot read setup input")?
            != 0
        {
            if byte[0] == b'\n' {
                break;
            }
            text.push(byte[0] as char);
            if text.len() > 4096 {
                return Err("Setup input too large".into());
            }
        }
        Ok(())
    }
}
pub fn setup(store: &Store) -> Result<()> {
    use std::io::IsTerminal;
    if !io::stdin().is_terminal() {
        return Err("Chat setup requires an interactive terminal".into());
    }
    let base_url = prompt("OpenAI-compatible base URL ending in /v1")?;
    let model = prompt("Model name")?;
    let path = prompt("Private API key file (absolute path, blank for no key)")?;
    let consent = prompt(
        "Send questions and normalized observed titles, summaries and evidence to this endpoint? Type yes",
    )? == "yes";
    save(
        store,
        &ChatConfig {
            version: 1,
            base_url,
            model,
            api_key_file: (!path.is_empty()).then_some(path),
            consent,
        },
    )
}
