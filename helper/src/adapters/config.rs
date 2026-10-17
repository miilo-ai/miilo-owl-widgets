use crate::Result;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

pub use crate::domain::{
    config_policy::{safe_url, validate},
    connection::{Config, Endpoint},
};
impl Endpoint {
    pub fn credential(&self) -> Result<String> {
        let Some(path) = &self.token_file else {
            return Ok(self.token.clone());
        };
        let mut token = String::new();
        private_file(path)?
            .take(4097)
            .read_to_string(&mut token)
            .map_err(|_| "Cannot read credential file")?;
        let token = token.trim();
        if token.is_empty() || token.len() > 4096 || token.chars().any(char::is_whitespace) {
            return Err("Invalid credential file".into());
        }
        Ok(token.into())
    }
}

pub struct Store {
    pub directory: PathBuf,
}
impl Store {
    pub fn from_env() -> Result<Self> {
        let directory = match std::env::var_os("MIILO_OWL_CONFIG_DIR") {
            Some(p) => PathBuf::from(p),
            None => {
                let home =
                    PathBuf::from(std::env::var_os("HOME").ok_or("Home directory unavailable")?);
                let root = std::env::var_os("MIILO_HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| home.join(".miilo"));
                if !root.is_absolute() {
                    return Err("MIILO_HOME must be absolute".into());
                }
                if fs::symlink_metadata(&root).is_ok_and(|m| !m.is_dir() || m.file_type().is_symlink()) {
                    return Err("MIILO_HOME must be a directory, not a symlink".into());
                }
                let manifest = root.join("config.json");
                if manifest
                    .try_exists()
                    .map_err(|_| "Cannot inspect Miilo configuration")?
                    || fs::symlink_metadata(&manifest).is_ok()
                {
                    let value: serde_json::Value =
                        serde_json::from_reader(private_file(&manifest)?.take(2_000_001))
                            .map_err(|_| "Invalid Miilo configuration")?;
                    if value["schemaVersion"] != 1 {
                        return Err("Unsupported Miilo configuration".into());
                    }
                    root.join("connections")
                } else {
                    home.join(".config/miilo-owl")
                }
            }
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
