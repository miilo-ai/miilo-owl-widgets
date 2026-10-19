use serde_json::Value;
use std::{
    fs,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::{Command, Output},
};
struct Fixture {
    _temp: tempfile::TempDir,
    root: PathBuf,
    widgets: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::Builder::new()
            .prefix("miilo removal ")
            .tempdir()
            .unwrap();
        let root = temp.path().canonicalize().unwrap();
        let widgets = root.join("widgets");
        fs::create_dir(&widgets).unwrap();
        fs::write(widgets.join("70-miilo-fleet.owl"), "widget synthetic").unwrap();
        fs::write(widgets.join("other.owl"), "untouched").unwrap();
        fs::write(root.join("private-token"), "fixture-only").unwrap();
        Self {
            _temp: temp,
            root,
            widgets,
        }
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_miilo-owl"))
            .args(args)
            .env("HOME", &self.root)
            .env("XDG_STATE_HOME", self.root.join("state"))
            .env("MIILO_OWL_CONFIG_DIR", self.root.join("config"))
            .output()
            .unwrap()
    }
    fn record(&self) {
        let out = self.run(&[
            "record-widget-install",
            self.widgets.to_str().unwrap(),
            "/retained/miilo-owl",
        ]);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stdout)
        );
    }
    fn plan(&self) -> Value {
        let out = self.run(&["uninstall", "widgets"]);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stdout)
        );
        serde_json::from_slice(&out.stdout).unwrap()
    }
}
#[test]
fn preview_confirm_repeat_and_reinstall_preserve_private_data() {
    let f = Fixture::new();
    f.record();
    let plan = f.plan();
    assert!(f.widgets.join("70-miilo-fleet.owl").exists());
    assert!(
        !f.run(&["uninstall", "widgets", "--confirm", "wrong"])
            .status
            .success()
    );
    let args = [
        "uninstall",
        "widgets",
        "--confirm",
        plan["plan_id"].as_str().unwrap(),
    ];
    for _ in 0..2 {
        assert!(f.run(&args).status.success());
    }
    assert!(!f.widgets.join("70-miilo-fleet.owl").exists());
    assert_eq!(
        fs::read_to_string(f.widgets.join("other.owl")).unwrap(),
        "untouched"
    );
    assert_eq!(
        fs::read_to_string(f.root.join("private-token")).unwrap(),
        "fixture-only"
    );
    fs::write(f.widgets.join("70-miilo-fleet.owl"), "new widget release").unwrap();
    f.record();
    assert_eq!(f.plan()["remove"].as_array().unwrap().len(), 1);
}
#[test]
fn modified_files_and_symlinks_are_preserved() {
    let f = Fixture::new();
    f.record();
    fs::write(f.widgets.join("70-miilo-fleet.owl"), "user edited").unwrap();
    assert!(!f.run(&["uninstall", "widgets"]).status.success());
    fs::remove_file(f.widgets.join("70-miilo-fleet.owl")).unwrap();
    symlink(
        f.root.join("private-token"),
        f.widgets.join("70-miilo-fleet.owl"),
    )
    .unwrap();
    assert!(!f.run(&["uninstall", "widgets"]).status.success());
    assert_eq!(
        fs::read_to_string(f.root.join("private-token")).unwrap(),
        "fixture-only"
    );
}
#[test]
fn no_receipt_and_replaced_parent_fail_without_removal() {
    let f = Fixture::new();
    assert!(!f.run(&["uninstall", "widgets"]).status.success());
    f.record();
    let plan = f.plan();
    fs::rename(&f.widgets, f.root.join("preserved-widgets")).unwrap();
    symlink(f.root.join("preserved-widgets"), &f.widgets).unwrap();
    assert!(
        !f.run(&[
            "uninstall",
            "widgets",
            "--confirm",
            plan["plan_id"].as_str().unwrap()
        ])
        .status
        .success()
    );
    assert!(f.root.join("preserved-widgets/70-miilo-fleet.owl").exists());
}
#[test]
fn stale_confirmation_and_purge_are_rejected() {
    let f = Fixture::new();
    f.record();
    let plan = f.plan();
    fs::write(f.widgets.join("70-miilo-fleet.owl"), "new widget release").unwrap();
    f.record();
    assert!(
        !f.run(&[
            "uninstall",
            "widgets",
            "--confirm",
            plan["plan_id"].as_str().unwrap()
        ])
        .status
        .success()
    );
    assert!(!f.run(&["uninstall", "widgets", "--purge"]).status.success());
    assert!(f.widgets.join("70-miilo-fleet.owl").exists());
}
#[test]
fn actual_installer_remove_reinstall_roundtrip() {
    let f = Fixture::new();
    let package = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    for _ in 0..2 {
        let out = Command::new("bash")
            .arg(package.join("install.sh"))
            .arg("--no-setup")
            .env("HOME", &f.root)
            .env("XDG_STATE_HOME", f.root.join("state"))
            .env("MIILO_OWL_BINARY", env!("CARGO_BIN_EXE_miilo-owl"))
            .env("MIILO_OWL_INSTALL_ROOT", f.root.join("local"))
            .env("MIILO_OWL_WIDGET_DIR", &f.widgets)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{} {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        let plan = f.plan();
        assert!(
            f.run(&[
                "uninstall",
                "widgets",
                "--confirm",
                plan["plan_id"].as_str().unwrap()
            ])
            .status
            .success()
        );
        assert!(f.root.join("local/bin/miilo-owl").is_file());
    }
    assert_eq!(
        fs::read_to_string(f.root.join("private-token")).unwrap(),
        "fixture-only"
    );
}
