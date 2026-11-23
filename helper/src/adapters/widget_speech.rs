//! Synthesis only. Android owns microphone permissions, audio focus and playback.
use crate::{Result, config::Store, domain::chat::ChatConfig};
use reqwest::{blocking::Client, redirect::Policy};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    time::Duration,
};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpeechConfig {
    pub version: u32,
    pub base_url: String,
    pub model: String,
    pub voice: String,
    pub api_key_file: Option<String>,
    pub consent: bool,
}
fn chat_config(config: &SpeechConfig) -> Result<ChatConfig> {
    if config.voice.trim().is_empty() || config.voice.len() > 200 {
        return Err("A speech voice is required".into());
    }
    let chat = ChatConfig {
        version: config.version,
        base_url: config.base_url.clone(),
        model: config.model.clone(),
        api_key_file: config.api_key_file.clone(),
        consent: config.consent,
        max_output_tokens: 128,
        template_thinking: None,
    };
    super::chat_config::validate(&chat)?;
    Ok(chat)
}
pub fn load(store: &Store) -> Result<SpeechConfig> {
    let file = crate::config::private_file(&store.directory.join("speech.json"))
        .map_err(|_| "Configure a speech model in Settings / Spoken replies first")?;
    let config: SpeechConfig =
        serde_json::from_reader(file.take(16_385)).map_err(|_| "Invalid speech configuration")?;
    chat_config(&config)?;
    Ok(config)
}
pub fn save(store: &Store, config: &SpeechConfig) -> Result<()> {
    let chat = chat_config(config)?;
    super::chat_config::credential(&chat)?;
    store.write(
        "speech.json",
        &serde_json::to_value(config).map_err(|_| "Invalid speech configuration")?,
    )
}
pub fn synthesize(store: &Store, text: &str) -> Result<String> {
    if text.trim().is_empty() || text.len() > 16_000 {
        return Err("Speech text must contain 1–16000 bytes".into());
    }
    let config = load(store)?;
    let chat = chat_config(&config)?;
    let client = Client::builder()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|_| "Cannot initialize speech client")?;
    let mut request = client.post(format!("{}/audio/speech",config.base_url.trim_end_matches('/')))
        .json(&serde_json::json!({"model":config.model,"voice":config.voice,"input":text,"response_format":"wav"}));
    if let Some(key) = super::chat_config::credential(&chat)? {
        request = request.bearer_auth(key);
    }
    let response = request.send().map_err(|_| "Speech endpoint unavailable")?;
    if !response.status().is_success() {
        return Err("Speech endpoint rejected the request".into());
    }
    let mut audio = Vec::new();
    response
        .take(5_000_001)
        .read_to_end(&mut audio)
        .map_err(|_| "Cannot read speech audio")?;
    if audio.len() > 5_000_000
        || audio.len() < 44
        || &audio[..4] != b"RIFF"
        || &audio[8..12] != b"WAVE"
    {
        return Err("Speech endpoint must return bounded WAV audio".into());
    }
    cleanup(store)?;
    let mut file = tempfile::Builder::new()
        .prefix("widget-speech-")
        .suffix(".wav")
        .tempfile_in(&store.directory)
        .map_err(|_| "Cannot protect speech audio")?;
    file.write_all(&audio)
        .map_err(|_| "Cannot save speech audio")?;
    let (_, path) = file.keep().map_err(|_| "Cannot retain speech audio")?;
    Ok(path.to_string_lossy().into())
}
pub fn cleanup(store: &Store) -> Result<()> {
    store.prepare()?;
    for entry in std::fs::read_dir(&store.directory).map_err(|_| "Cannot inspect speech cache")? {
        let entry = entry.map_err(|_| "Cannot inspect speech cache")?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with("widget-speech-")
            && name.ends_with(".wav")
            && entry
                .file_type()
                .map_err(|_| "Cannot inspect speech audio")?
                .is_file()
        {
            std::fs::remove_file(entry.path()).map_err(|_| "Cannot remove speech audio")?;
        }
    }
    Ok(())
}
pub fn run(store: &Store) -> Result<()> {
    let mut input = Vec::new();
    std::io::stdin()
        .take(17_001)
        .read_to_end(&mut input)
        .map_err(|_| "Cannot read speech text")?;
    let id = input
        .split(|b| *b == 0)
        .next()
        .and_then(|b| std::str::from_utf8(b).ok())
        .unwrap_or("")
        .chars()
        .take(80)
        .collect::<String>();
    let result = (|| {
        let text = std::str::from_utf8(&input).map_err(|_| "Invalid speech text")?;
        let (id, text) = text.split_once('\0').ok_or("Invalid speech request")?;
        if id.is_empty() || id.len() > 128 {
            return Err("Invalid speech request".into());
        }
        Ok(serde_json::json!({"requestId":id,"path":synthesize(store,text)?,"error":null}))
    })();
    let value = result
        .unwrap_or_else(|message: String| serde_json::json!({"requestId":id,"error":message}));
    println!("{value}");
    Ok(())
}
