use super::connection::Config;
use crate::Result;
pub fn safe_url(value: &str) -> Result<String> {
    let u = url::Url::parse(value).map_err(|_| "Invalid endpoint URL")?;
    if !matches!(u.scheme(), "http" | "https")
        || u.host_str().is_none()
        || !u.username().is_empty()
        || u.password().is_some()
        || u.query().is_some()
        || u.fragment().is_some()
    {
        return Err("Use an http(s) endpoint without credentials, query or fragment".into());
    }
    Ok(value.trim_end_matches('/').into())
}

pub fn validate(config: &Config) -> Result<()> {
    if config.version != 1 || config.endpoints.len() > 8 {
        return Err("At most eight endpoints are supported".into());
    }
    let mut seen = std::collections::HashSet::new();
    for e in &config.endpoints {
        let valid = !e.id.is_empty()
            && e.id.len() <= 40
            && e.id
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            && e.id.as_bytes()[0].is_ascii_alphanumeric();
        if !valid || !seen.insert(&e.id) {
            return Err("Endpoint names must be unique lowercase words/dashes".into());
        }
        if e.paired_host.as_ref().is_some_and(|peer| {
            e.url != peer.url
                || !e.url.starts_with("https://")
                || e.allow_actions
                || !e.token.is_empty()
                || e.token_file.is_some()
                || e.local_snapshot_file.is_some()
        }) {
            return Err(
                "Paired computers require their pinned HTTPS origin and read-only access".into(),
            );
        }
        if let Some(path) = &e.local_snapshot_file {
            if !path.is_absolute()
                || !e.token.is_empty()
                || e.token_file.is_some()
                || e.allow_actions
                || e.url != "local://desktop"
            {
                return Err("Local snapshot sources require an absolute file, no credential, and read-only access".into());
            }
        } else {
            safe_url(&e.url)?;
        }
        if e.token_file.is_some() && !e.token.is_empty() {
            return Err("Choose a credential or a credential file, not both".into());
        }
        if e.token_file.as_ref().is_some_and(|p| !p.is_absolute()) {
            return Err("Credential file paths must be absolute".into());
        }
        if e.token.contains(['\r', '\n']) {
            return Err("Invalid credential".into());
        }
    }
    Ok(())
}
