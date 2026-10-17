use miilo_owl::{
    Result, config::Store, display_names, operations, setup, snapshot, transport::Http,
};
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
        ["local-agents"] => {
            let home =
                std::path::PathBuf::from(std::env::var_os("HOME").ok_or("Home unavailable")?);
            let paths = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
                .collect::<Vec<_>>();
            let mut detected = miilo_owl::local_agents::detect(&home, &paths);
            let fingerprint = miilo_owl::key(&detected["agents"]);
            let dismissed =
                miilo_owl::config::private_file(&store.directory.join("local-discovery.json"))
                    .ok()
                    .and_then(|file| {
                        serde_json::from_reader::<_, Value>(std::io::Read::take(file, 4096)).ok()
                    })
                    .is_some_and(|value| value["fingerprint"] == fingerprint);
            detected["fingerprint"] = json!(fingerprint);
            detected["dismissed"] = json!(dismissed);
            Ok(Some(detected))
        }
        ["dismiss-local", fingerprint]
            if fingerprint.len() == 32 && fingerprint.bytes().all(|b| b.is_ascii_hexdigit()) =>
        {
            store.write("local-discovery.json", &json!({"fingerprint":fingerprint}))?;
            Ok(Some(json!({"status":"saved"})))
        }
        ["connect-local", path] => {
            miilo_owl::local_import::connect(&store, &http, std::path::Path::new(path)).map(Some)
        }
        ["install-desktop", root, mode] if matches!(*mode, "portable" | "omarchy") => {
            miilo_owl::desktop_install::install(
                std::path::Path::new(root),
                &std::env::current_exe().map_err(|_| "Cannot locate helper")?,
                &miilo_owl::desktop_install::Paths::from_env()?,
                *mode == "omarchy",
            )?;
            Ok(Some(
                json!({"status":"installed","activation":"explicit","mode":mode}),
            ))
        }
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
        ["name-agent", actor, label] => {
            display_names::name_agent(&store, &http, actor, label).map(Some)
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
            "miilo-owl: setup | snapshot | refresh | explain ITEM REVISION | action ITEM ACTION REVISION | name-agent ACTOR NAME"
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
