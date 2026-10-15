use crate::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

#[derive(Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub id: String,
    pub url: String,
    #[serde(default)]
    pub token: String,
    pub providers: Vec<String>,
    #[serde(default, rename = "allowActions")]
    pub allow_actions: bool,
    #[serde(
        default,
        rename = "actorNames",
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub actor_names: std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub endpoints: Vec<Endpoint>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            endpoints: vec![],
        }
    }
}

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
        safe_url(&e.url)?;
        if e.token.contains(['\r', '\n']) {
            return Err("Invalid credential".into());
        }
    }
    Ok(())
}

pub struct Store {
    pub directory: PathBuf,
}
impl Store {
    pub fn from_env() -> Result<Self> {
        let directory = match std::env::var_os("MIILO_OWL_CONFIG_DIR") {
            Some(p) => PathBuf::from(p),
            None => PathBuf::from(std::env::var_os("HOME").ok_or("Home directory unavailable")?)
                .join(".config/miilo-owl"),
        };
        Ok(Self { directory })
    }
    pub fn load(&self) -> Result<Config> {
        let path = self.directory.join("config.json");
        if !path
            .try_exists()
            .map_err(|_| "Cannot inspect configuration")?
        {
            // Reject a dangling symlink as well as a live one.
            if fs::symlink_metadata(&path).is_ok() {
                return Err("Invalid configuration".into());
            }
            return Ok(Config::default());
        }
        let file = private_file(&path)?;
        let data: Config =
            serde_json::from_reader(file.take(2_000_001)).map_err(|_| "Invalid configuration")?;
        validate(&data)?;
        Ok(data)
    }
    pub fn save(&self, config: &Config) -> Result<()> {
        validate(config)?;
        self.write(
            "config.json",
            &serde_json::to_value(config).map_err(|_| "Invalid configuration")?,
        )?;
        self.invalidate()
    }
    pub fn prepare(&self) -> Result<()> {
        fs::create_dir_all(&self.directory).map_err(|_| "Cannot create configuration directory")?;
        if fs::symlink_metadata(&self.directory)
            .map_err(|_| "Cannot inspect directory")?
            .file_type()
            .is_symlink()
        {
            return Err("Invalid configuration directory".into());
        }
        fs::set_permissions(&self.directory, fs::Permissions::from_mode(0o700))
            .map_err(|_| "Cannot protect configuration".to_owned())
    }
    pub fn write(&self, name: &str, value: &serde_json::Value) -> Result<()> {
        self.prepare()?;
        let mut temp =
            NamedTempFile::new_in(&self.directory).map_err(|_| "Cannot save configuration")?;
        temp.as_file()
            .set_permissions(fs::Permissions::from_mode(0o600))
            .map_err(|_| "Cannot protect configuration")?;
        serde_json::to_writer(&mut temp, value).map_err(|_| "Cannot save configuration")?;
        temp.flush()
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|_| "Cannot save configuration")?;
        temp.persist(self.directory.join(name))
            .map_err(|_| "Cannot save configuration")?;
        Ok(())
    }
    pub fn invalidate(&self) -> Result<()> {
        match fs::remove_file(self.directory.join("snapshot-cache.json")) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => Err("Cannot invalidate cache".into()),
        }
    }
    pub fn lock(&self) -> Result<File> {
        self.prepare()?;
        let path = self.directory.join("snapshot.lock");
        reject_link(&path)?;
        let file = OpenOptions::new()
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .mode(0o600)
            .open(&path)
            .map_err(|_| "Cannot open cache lock")?;
        fs2::FileExt::lock_exclusive(&file).map_err(|_| "Cannot lock cache")?;
        Ok(file)
    }
}
pub fn reject_link(path: &Path) -> Result<()> {
    if let Ok(m) = fs::symlink_metadata(path)
        && (!m.is_file() || m.file_type().is_symlink())
    {
        return Err("Expected a private regular file".into());
    }
    Ok(())
}
pub fn private_file(path: &Path) -> Result<File> {
    reject_link(path)?;
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| "Cannot read private file")?;
    if !file
        .metadata()
        .map_err(|_| "Cannot inspect private file")?
        .is_file()
    {
        return Err("Expected a private regular file".into());
    }
    if file
        .metadata()
        .map_err(|_| "Cannot inspect private file")?
        .permissions()
        .mode()
        & 0o077
        != 0
    {
        return Err("Configuration must be private (chmod 600)".into());
    }
    Ok(file)
}
