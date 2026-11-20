use crate::{Result, config::Store};
use std::io::{self, BufRead, Read, Write};
fn prompt(label: &str) -> Result<String> {
    print!("{label}: ");
    io::stdout().flush().map_err(|_| "Cannot prompt")?;
    let mut input = String::new();
    io::stdin()
        .lock()
        .take(4097)
        .read_line(&mut input)
        .map_err(|_| "Cannot read speech setup")?;
    if input.len() > 4096 {
        return Err("Setup value too long".into());
    }
    Ok(input.trim().to_owned())
}
pub fn run(store: &Store) -> Result<()> {
    println!("Configure spoken replies. Replies are sent only to this endpoint when enabled.");
    let base_url = prompt("OpenAI-compatible HTTPS /v1 URL (loopback HTTP allowed)")?;
    let model = prompt("Speech model")?;
    let voice = prompt("Voice")?;
    let key = prompt("Absolute protected credential-file path (blank for none)")?;
    let consent =
        prompt("Allow Miilo replies to be sent to this speech endpoint? Type yes")? == "yes";
    super::widget_speech::save(
        store,
        &super::widget_speech::SpeechConfig {
            version: 1,
            base_url,
            model,
            voice,
            api_key_file: (!key.is_empty()).then_some(key),
            consent,
        },
    )?;
    println!("Speech configuration saved. Enable spoken replies in Ask Miilo to test.");
    Ok(())
}
