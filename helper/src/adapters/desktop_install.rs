//! Desktop packaging stays native; installation never activates shell plugins.
use crate::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Paths {
    pub data: PathBuf,
    pub config: PathBuf,
    pub bin: PathBuf,
}

impl Paths {
    pub fn from_env() -> Result<Self> {
        let home = PathBuf::from(
            std::env::var_os("MIILO_DESKTOP_HOME")
                .or_else(|| std::env::var_os("HOME"))
                .ok_or("Home directory unavailable")?,
        );
        let staging = std::env::var_os("MIILO_DESKTOP_HOME").is_some();
        Ok(Self {
            data: if staging {
                home.join(".local/share")
            } else {
                std::env::var_os("XDG_DATA_HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| home.join(".local/share"))
            },
            config: if staging {
                home.join(".config")
            } else {
                std::env::var_os("XDG_CONFIG_HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| home.join(".config"))
            },
            bin: home.join(".local/bin"),
        })
    }
}

fn copy_tree(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target).map_err(|_| "Cannot create install directory")?;
    for entry in fs::read_dir(source).map_err(|_| "Cannot read desktop package")? {
        let entry = entry.map_err(|_| "Cannot read desktop package")?;
        let name = entry.file_name();
        if name == ".errors" || name == "Smoke.qml" || name == "manifest.json" {
            continue;
        }
        let kind = entry
            .file_type()
            .map_err(|_| "Cannot inspect desktop package")?;
        if kind.is_symlink() {
            return Err("Desktop package must not contain symlinks".into());
        }
        if kind.is_dir() {
            copy_tree(&entry.path(), &target.join(name))?;
        } else {
            copy_changed(&entry.path(), &target.join(name))?;
        }
    }
    Ok(())
}

fn copy_changed(source: &Path, target: &Path) -> Result<()> {
    let bytes = fs::read(source).map_err(|_| "Cannot read desktop file")?;
    if fs::read(target).is_ok_and(|existing| existing == bytes) {
        return Ok(());
    }
    fs::copy(source, target).map_err(|_| "Cannot copy desktop file")?;
    Ok(())
}

fn install_binary(source: &Path, target: &Path) -> Result<()> {
    let parent = target.parent().ok_or("Invalid binary location")?;
    let temporary = tempfile::NamedTempFile::new_in(parent).map_err(|_| "Cannot stage binary")?;
    fs::copy(source, temporary.path()).map_err(|_| "Cannot copy binary")?;
    temporary
        .persist(target)
        .map_err(|_| "Cannot install binary")?;
    Ok(())
}

pub fn install(root: &Path, binary: &Path, paths: &Paths, native: bool) -> Result<()> {
    if !binary.is_file() || !root.join("desktop/shell.qml").is_file() {
        return Err("A built helper and desktop source package are required".into());
    }
    fs::create_dir_all(&paths.bin).map_err(|_| "Cannot create executable directory")?;
    install_binary(binary, &paths.bin.join("miilo-owl"))?;
    for (variable, name) in [
        ("MIILO_LOCAL_BINARY", "miilo-local"),
        ("MIILO_CE_BINARY", "miilo-ce"),
    ] {
        if let Some(source) = std::env::var_os(variable) {
            install_binary(Path::new(&source), &paths.bin.join(name))?;
        }
    }
    install_binary(
        &root.join("tools/miilo-desktop"),
        &paths.bin.join("miilo-desktop"),
    )?;
    let desktop = paths.data.join("miilo/desktop");
    copy_tree(&root.join("desktop"), &desktop)?;
    copy_changed(
        &root.join("70-miilo-fleet.owl"),
        &desktop.join("70-miilo-fleet.owl"),
    )?;
    let applications = paths.data.join("applications");
    fs::create_dir_all(&applications).map_err(|_| "Cannot create application directory")?;
    // Desktop Entry quoting is distinct from shell quoting.
    let command = paths
        .bin
        .join("miilo-desktop")
        .to_string_lossy()
        .replace('\\', "\\\\\\\\")
        .replace('"', "\\\\\"")
        .replace('`', "\\\\`")
        .replace('$', "\\\\$")
        .replace('%', "%%");
    fs::write(applications.join("miilo.desktop"), format!("[Desktop Entry]\nType=Application\nName=Miilo\nComment=Observe automated work\nExec=\"{command}\" dashboard\nIcon=utilities-system-monitor\nTerminal=false\nCategories=Utility;\n")).map_err(|_| "Cannot write desktop entry")?;
    if native {
        for name in ["bar", "dashboard", "desktop"] {
            let target = paths
                .config
                .join(format!("omarchy/plugins/io.miilo.{name}"));
            copy_tree(&root.join("desktop"), &target)?;
            copy_changed(
                &root.join("70-miilo-fleet.owl"),
                &target.join("70-miilo-fleet.owl"),
            )?;
            copy_changed(
                &root.join(format!("desktop/manifests/{name}.json")),
                &target.join("manifest.json"),
            )?;
        }
    }
    Ok(())
}
