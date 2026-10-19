//! Update selected systems without replacing a paired computer's credential.
use crate::{
    Result,
    config::{Config, validate},
    setup::Console,
    transport::Transport,
};
pub fn update(
    config: &Config,
    http: &dyn Transport,
    ui: &mut dyn Console,
    id: &str,
) -> Result<Config> {
    let endpoint = config
        .endpoints
        .iter()
        .find(|e| e.id == id && e.paired_host.is_some())
        .ok_or("Paired computer unavailable")?;
    let snapshot = http.request(endpoint, "/v1/snapshot", None)?;
    crate::projection::valid_records(&snapshot)?;
    let providers = snapshot["providers"]
        .as_array()
        .ok_or("No shared systems")?;
    for (index, provider) in providers.iter().enumerate() {
        ui.say(&format!(
            "{}. {}",
            index + 1,
            provider["name"].as_str().unwrap_or("System")
        ))?;
    }
    let answer = ui
        .read("Observe which numbers? Comma-separated, or all: ", false)?
        .ok_or("Selection cancelled")?;
    let numbers: Vec<usize> = if answer == "all" {
        (1..=providers.len()).collect()
    } else {
        answer
            .split(',')
            .map(|v| {
                v.trim()
                    .parse()
                    .map_err(|_| "Choose valid system numbers".to_owned())
            })
            .collect::<Result<_>>()?
    };
    if numbers.is_empty() || numbers.iter().any(|n| *n == 0 || *n > providers.len()) {
        return Err("Choose at least one shared system".into());
    }
    let mut next = config.clone();
    let selected = next.endpoints.iter_mut().find(|e| e.id == id).unwrap();
    selected.providers = numbers
        .iter()
        .map(|n| {
            providers[n - 1]["id"]
                .as_str()
                .map(String::from)
                .ok_or("Invalid provider ID".into())
        })
        .collect::<Result<_>>()?;
    validate(&next)?;
    Ok(next)
}
