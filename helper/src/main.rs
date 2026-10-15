use miilo_owl::{Result, config::Store, operations, setup, snapshot, transport::Http};
use serde_json::{Value, json};

fn run(args: &[String]) -> Result<Option<Value>> {
    let store = Store::from_env()?;
    let http = Http::new()?;
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["setup"] => {
            setup::setup(&store, &http, &mut setup::Terminal)?;
            Ok(None)
        }
        ["snapshot"] => snapshot::snapshot(&store, &http, false).map(Some),
        ["refresh"] => snapshot::snapshot(&store, &http, true).map(Some),
        ["explain", item, revision] => operations::explain(&store, &http, item, revision).map(Some),
        ["action", item, action, revision] => {
            operations::action(&store, &http, item, action, revision).map(Some)
        }
        _ => Err("Invalid command".into()),
    }
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if matches!(
        args.first().map(String::as_str),
        Some("--version" | "version")
    ) {
        println!("miilo-owl {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.is_empty() || matches!(args.first().map(String::as_str), Some("--help" | "-h")) {
        println!(
            "miilo-owl: setup | snapshot | refresh | explain ITEM REVISION | action ITEM ACTION REVISION"
        );
        return;
    }
    match run(&args) {
        Ok(Some(value)) => println!("{value}"),
        Ok(None) => {}
        Err(_) => {
            // Do not echo raw errors, endpoint URLs, tokens or remote payloads.
            println!(
                "{}",
                json!({"status":"failed","message":"Request failed. Refresh and check setup/access."})
            );
            std::process::exit(1);
        }
    }
}
