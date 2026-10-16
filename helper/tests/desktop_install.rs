use miilo_owl::desktop_install::{self, Paths};
use serde_json::Value;
use std::{fs, path::Path, process::Command};
fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}
#[test]
fn independent_plugins_preserve_connections_and_never_activate_themselves() {
    let temp = tempfile::Builder::new()
        .prefix("miilo desktop ")
        .tempdir()
        .unwrap();
    let home = temp.path();
    let paths = Paths {
        data: home.join(".local/share"),
        config: home.join(".config"),
        bin: home.join(".local/bin"),
    };
    let config = paths.config.join("miilo-owl/config.json");
    fs::create_dir_all(config.parent().unwrap()).unwrap();
    fs::write(&config, "private existing configuration").unwrap();
    for _ in 0..2 {
        desktop_install::install(
            root(),
            Path::new(env!("CARGO_BIN_EXE_miilo-owl")),
            &paths,
            true,
        )
        .unwrap();
    }
    assert_eq!(
        fs::read_to_string(config).unwrap(),
        "private existing configuration"
    );
    for surface in ["bar", "dashboard", "desktop"] {
        let plugin = paths
            .config
            .join(format!("omarchy/plugins/io.miilo.{surface}"));
        let manifest: Value =
            serde_json::from_slice(&fs::read(plugin.join("manifest.json")).unwrap()).unwrap();
        assert_eq!(manifest["id"], format!("io.miilo.{surface}"));
        assert_eq!(manifest["kinds"].as_array().unwrap().len(), 1);
        for entry in manifest["entryPoints"].as_object().unwrap().values() {
            assert!(plugin.join(entry.as_str().unwrap()).is_file());
        }
        assert_eq!(
            fs::read(plugin.join("70-miilo-fleet.owl")).unwrap(),
            fs::read(root().join("70-miilo-fleet.owl")).unwrap()
        );
        assert!(!plugin.join(".errors").exists());
    }
    assert!(!paths.config.join("omarchy/shell.json").exists());
    assert!(
        fs::read_to_string(paths.data.join("applications/miilo.desktop"))
            .unwrap()
            .contains(&format!(
                "Exec=\"{}\" dashboard",
                paths.bin.join("miilo-desktop").display()
            ))
    );
    assert!(
        Command::new(paths.bin.join("miilo-owl"))
            .arg("--version")
            .status()
            .unwrap()
            .success()
    );
}
#[test]
fn missing_binary_does_not_modify_home() {
    let temp = tempfile::tempdir().unwrap();
    let paths = Paths {
        data: temp.path().join("data"),
        config: temp.path().join("config"),
        bin: temp.path().join("bin"),
    };
    assert!(
        desktop_install::install(root(), Path::new("/nonexistent/miilo-owl"), &paths, false)
            .is_err()
    );
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 0);
}
