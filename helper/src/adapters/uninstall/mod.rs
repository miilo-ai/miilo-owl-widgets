mod files;
use crate::{
    Result,
    application::removal::{self, RemovalPort},
    domain::removal::{Plan, Receipt},
};
use files::{Directory, hash};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

fn receipt_directory() -> Result<PathBuf> {
    let home = PathBuf::from(std::env::var_os("HOME").ok_or("Home directory unavailable")?);
    Ok(std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/state"))
        .join("miilo/installations"))
}
fn record(widget: &Path, helper: &Path) -> Result<Value> {
    let bytes = Directory::open(widget)?
        .read("70-miilo-fleet.owl", 2_000_000)?
        .ok_or("Installed widget missing")?;
    let root = receipt_directory()?;
    std::fs::create_dir_all(&root).map_err(|_| "Cannot create receipt directory")?;
    let receipt = Receipt {
        schema_version: 1,
        widget_directory: widget.to_str().ok_or("Invalid widget path")?.into(),
        sha256: hash(&bytes),
        helper: helper.to_str().ok_or("Invalid helper path")?.into(),
    };
    Directory::open(&root)?.write(
        "widgets.json",
        &serde_json::to_vec(&receipt).map_err(|_| "Cannot encode receipt")?,
    )?;
    Ok(json!({"status":"recorded","surface":"widgets"}))
}
struct Widgets {
    receipt: Receipt,
}
impl Widgets {
    fn load() -> Result<Self> {
        let bytes = Directory::open(&receipt_directory()?)?.read("widgets.json", 16_000)?.ok_or("No widget installation receipt; reinstall with the current installer before automatic removal")?;
        let receipt: Receipt =
            serde_json::from_slice(&bytes).map_err(|_| "Invalid installation receipt")?;
        if receipt.schema_version != 1
            || receipt.sha256.len() != 64
            || !receipt.sha256.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err("Unsupported installation receipt".into());
        }
        Ok(Self { receipt })
    }
}
impl RemovalPort for Widgets {
    fn inspect(&self) -> Result<Plan> {
        let bytes = Directory::open(Path::new(&self.receipt.widget_directory))?
            .read("70-miilo-fleet.owl", 2_000_000)?;
        if bytes
            .as_ref()
            .is_some_and(|bytes| hash(bytes) != self.receipt.sha256)
        {
            return Err("Widget was modified; preserve it and review removal manually".into());
        }
        Ok(Plan::widgets(&self.receipt, bytes.is_some()))
    }
    fn remove(&self, plan: &Plan) -> Result<()> {
        if self.inspect()?.plan_id != plan.plan_id {
            return Err("Widget changed; inspect a fresh plan".into());
        }
        if !plan.remove.is_empty() {
            Directory::open(Path::new(&self.receipt.widget_directory))?
                .remove_widget(&self.receipt.sha256)?;
        }
        Ok(())
    }
}
pub fn dispatch(args: &[String]) -> Option<Result<Value>> {
    let words = args.iter().map(String::as_str).collect::<Vec<_>>();
    match words.as_slice() {
        ["record-widget-install", widget, helper] => {
            Some(record(Path::new(widget), Path::new(helper)))
        }
        ["uninstall", "--help"] => Some(Ok(
            json!({"usage":"uninstall widgets [--confirm PLAN_ID]","scope":"Only the receipt-owned fleet widget; helper, private data and observers remain"}),
        )),
        ["uninstall", "widgets"] => Some(
            Widgets::load()
                .and_then(|port| removal::run(&port, None))
                .and_then(|plan| {
                    serde_json::to_value(plan).map_err(|_| "Cannot encode removal plan".into())
                }),
        ),
        ["uninstall", "widgets", "--confirm", id] => Some(
            Widgets::load()
                .and_then(|port| removal::run(&port, Some(id)))
                .and_then(|plan| {
                    serde_json::to_value(plan).map_err(|_| "Cannot encode removal result".into())
                }),
        ),
        ["uninstall", ..] | ["record-widget-install", ..] => Some(Err(
            "Use uninstall widgets [--confirm PLAN_ID]; data purge is not supported".into(),
        )),
        _ => None,
    }
}
