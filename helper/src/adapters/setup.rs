use crate::{
    Result,
    config::{Config, Endpoint, Store, safe_url},
    projection::valid_records,
    transport::Transport,
};
use std::io::{BufRead, Write};

pub trait Console {
    fn say(&mut self, message: &str) -> Result<()>;
    fn read(&mut self, prompt: &str, secret: bool) -> Result<Option<String>>;
}
pub struct Terminal;
impl Console for Terminal {
    fn say(&mut self, message: &str) -> Result<()> {
        // Provider names may contain terminal control sequences; never execute them.
        println!(
            "{}",
            message
                .chars()
                .filter(|c| !c.is_control())
                .collect::<String>()
        );
        Ok(())
    }
    fn read(&mut self, prompt: &str, secret: bool) -> Result<Option<String>> {
        if secret {
            return rpassword::prompt_password(prompt)
                .map(Some)
                .map_err(|_| "Cannot read hidden credential".into());
        }
        print!("{prompt}");
        std::io::stdout()
            .flush()
            .map_err(|_| "Cannot display prompt")?;
        let mut line = String::new();
        if std::io::stdin()
            .lock()
            .read_line(&mut line)
            .map_err(|_| "Cannot read setup input")?
            == 0
        {
            return Ok(None);
        }
        Ok(Some(line.trim().to_owned()))
    }
}
fn ask(ui: &mut dyn Console, prompt: &str) -> Result<String> {
    ui.read(prompt, false)?.ok_or("Setup cancelled".into())
}
fn add(
    config: &Config,
    http: &dyn Transport,
    ui: &mut dyn Console,
    name: String,
) -> Result<Config> {
    let mut endpoint = Endpoint {
        paired_host: None,
        id: name,
        url: safe_url(&ask(ui, "Miilo API URL: ")?)?,
        token: ui
            .read(
                "Bearer token or @/absolute/credential-file (hidden; blank if not required): ",
                true,
            )?
            .ok_or("Setup cancelled")?,
        token_file: None,
        providers: vec![],
        allow_actions: false,
        local_snapshot_file: None,
        actor_names: Default::default(),
    };
    if let Some(file) = endpoint.token.strip_prefix('@') {
        let path = std::path::PathBuf::from(file);
        if !path.is_absolute() {
            return Err("Use @ followed by an absolute credential-file path".into());
        }
        endpoint.token_file = Some(path);
        endpoint.token.clear();
        endpoint.credential()?;
    }
    let raw = http.request(&endpoint, "/v1/snapshot", None)?;
    valid_records(&raw)?;
    let providers = raw["providers"]
        .as_array()
        .ok_or("No providers registered; add them in Miilo first")?;
    if providers.is_empty() {
        return Err("No providers registered; add them in Miilo first".into());
    }
    for (i, p) in providers.iter().enumerate() {
        ui.say(&format!(
            "  {}. {}",
            i + 1,
            p["name"]
                .as_str()
                .unwrap_or(p["id"].as_str().unwrap_or("System"))
        ))?;
    }
    let choice = ask(
        ui,
        "Observe which? Enter numbers separated by commas, or all: ",
    )?;
    let positions: Vec<usize> = if choice == "all" {
        (1..=providers.len()).collect()
    } else {
        choice
            .split(',')
            .map(|x| {
                x.trim()
                    .parse::<usize>()
                    .map_err(|_| "Choose valid provider numbers".to_owned())
            })
            .collect::<Result<_>>()?
    };
    if positions.is_empty() || positions.iter().any(|i| *i == 0 || *i > providers.len()) {
        return Err("Choose at least one valid provider".into());
    }
    for i in positions {
        let id = providers[i - 1]["id"]
            .as_str()
            .ok_or("Invalid provider ID")?
            .to_owned();
        if !endpoint.providers.contains(&id) {
            endpoint.providers.push(id);
        }
    }
    endpoint.allow_actions = ask(
        ui,
        "Allow provider actions with explicit confirmation (excluding destructive actions)? [y/N]: ",
    )?
    .eq_ignore_ascii_case("y");
    let mut next = config.clone();
    if let Some(old) = config.endpoints.iter().find(|e| e.id == endpoint.id) {
        endpoint.actor_names = old.actor_names.clone();
    }
    next.endpoints.retain(|e| e.id != endpoint.id);
    next.endpoints.push(endpoint);
    Ok(next)
}
pub fn setup(store: &Store, http: &dyn Transport, ui: &mut dyn Console) -> Result<()> {
    let mut config = store.load()?;
    ui.say("Miilo · choose the systems your widgets observe")?;
    loop {
        for e in &config.endpoints {
            ui.say(&format!(
                "  {}: {} selected providers",
                e.id,
                e.providers.len()
            ))?;
        }
        let Some(op) = ui.read("[a] add/update endpoint, [r] remove, [q] finish: ", false)? else {
            return Ok(());
        };
        match op.to_lowercase().as_str() {
            "q" => return Ok(()),
            "a" | "r" => {
                let name = ask(ui, "Endpoint name (e.g. home, work): ")?;
                let next = if op.eq_ignore_ascii_case("r") {
                    let mut next = config.clone();
                    next.endpoints.retain(|e| e.id != name);
                    Ok(next)
                } else if config
                    .endpoints
                    .iter()
                    .any(|e| e.id == name && e.paired_host.is_some())
                {
                    crate::adapters::paired_selection::update(&config, http, ui, &name)
                } else {
                    add(&config, http, ui, name)
                };
                match next.and_then(|next| { store.save(&next)?; Ok(next) }) {
                    Ok(next) => { config=next; ui.say("Saved. New providers require explicit selection.")?; }
                    Err(_) => ui.say("Not saved. Check the name, provider selection, network and access; try again.")?,
                }
            }
            _ => ui.say("Choose a, r or q.")?,
        }
    }
}
