//! Exercise the compiled executable over authenticated real localhost HTTP.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    os::unix::fs::PermissionsExt,
    process::{Command, Output},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
fn fixture() -> Value {
    json!({"protocolVersion":"0.1","providers":[{"id":"factory","name":"Factory"}],"workItems":[{"id":"task-1","providerId":"factory","title":"Review recovery","kind":"task","revision":"r1","lifecycle":"waiting","phase":"reviewing","attention":"approval","actions":[{"id":"approve","label":"Approve","available":true,"risk":"bounded","confirmation":"explicit","inputSchema":null}]}],"actors":[],"attention":[],"attempts":[],"evidence":[]})
}
struct State {
    current: Value,
    mode: String,
    conflict: bool,
    explanation_revision: Option<String>,
    visits: Vec<String>,
    payloads: Vec<(String, Value)>,
}
struct Server {
    port: u16,
    state: Arc<Mutex<State>>,
    stop: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}
impl Server {
    fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        listener.set_nonblocking(true).unwrap();
        let state = Arc::new(Mutex::new(State {
            current: fixture(),
            mode: String::new(),
            conflict: false,
            explanation_revision: None,
            visits: vec![],
            payloads: vec![],
        }));
        let stop = Arc::new(AtomicBool::new(false));
        let shared = state.clone();
        let stopping = stop.clone();
        let thread = thread::spawn(move || {
            while !stopping.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => handle(stream, &shared),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2))
                    }
                    Err(e) => panic!("{e}"),
                }
            }
        });
        Self {
            port,
            state,
            stop,
            thread: Some(thread),
        }
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.thread.take().unwrap().join().unwrap();
    }
}
fn handle(mut stream: TcpStream, state: &Mutex<State>) {
    stream.set_nonblocking(false).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut data = vec![];
    let mut chunk = [0; 8192];
    let header_end = loop {
        let n = match stream.read(&mut chunk) {
            Ok(n) => n,
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                return;
            }
            Err(e) => panic!("fixture read failed: {e}"),
        };
        if n == 0 {
            return;
        }
        data.extend_from_slice(&chunk[..n]);
        if let Some(at) = data.windows(4).position(|w| w == b"\r\n\r\n") {
            break at + 4;
        }
    };
    let headers = String::from_utf8_lossy(&data[..header_end]).to_string();
    let mut first = headers.lines().next().unwrap().split_whitespace();
    let method = first.next().unwrap();
    let path = first.next().unwrap().to_owned();
    let length = headers
        .lines()
        .find_map(|l| {
            l.to_ascii_lowercase()
                .strip_prefix("content-length:")
                .and_then(|v| v.trim().parse::<usize>().ok())
        })
        .unwrap_or(0);
    while data.len() < header_end + length {
        let n = match stream.read(&mut chunk) {
            Ok(n) => n,
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                return;
            }
            Err(e) => panic!("fixture read failed: {e}"),
        };
        if n == 0 {
            return;
        }
        data.extend_from_slice(&chunk[..n]);
    }
    let authorized = headers
        .lines()
        .any(|l| l.eq_ignore_ascii_case("authorization: Bearer private-token"));
    let mut state = state.lock().unwrap();
    let mut status = 200;
    let mut extra = "";
    let body;
    if method == "GET" {
        state.visits.push(path.clone());
        if !authorized {
            status = 401;
            body = json!({"detail":"secret should never be echoed"}).to_string();
        } else {
            match state.mode.as_str() {
                "redirect" => {
                    status = 302;
                    extra = "Location: /leaked-token\r\n";
                    body = "{}".into();
                }
                "large" => body = json!({"excess":"x".repeat(2_000_000)}).to_string(),
                "invalid" => body = "not-json private-token".into(),
                _ => body = state.current.to_string(),
            }
        }
    } else if !authorized {
        status = 403;
        body = "{}".into();
    } else {
        let payload: Value =
            serde_json::from_slice(&data[header_end..header_end + length]).unwrap();
        state.payloads.push((path.clone(), payload.clone()));
        if path.ends_with("/v1/explanations") {
            body=json!({"revision":state.explanation_revision.as_ref().map(|s|json!(s)).unwrap_or(payload["expectedRevision"].clone()),"spokenSummary":"Recovery passed. Review before promotion.","changes":[],"evidence":[]}).to_string();
        } else if state.conflict {
            status = 409;
            body = json!({"detail":"revision changed private-token"}).to_string();
        } else {
            body = json!({"status":"accepted","message":"Queued for promotion"}).to_string();
        }
    }
    drop(state);
    let response = format!(
        "HTTP/1.1 {status} Result\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{extra}\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
}
struct Journey {
    server: Server,
    folder: tempfile::TempDir,
    home: Value,
    office: Value,
}
impl Journey {
    fn new() -> Self {
        let server = Server::new();
        let folder = tempfile::tempdir().unwrap();
        let home = json!({"id":"home","url":format!("http://127.0.0.1:{}/home",server.port),"token":"private-token","providers":["factory"],"allowActions":true});
        let mut office = home.clone();
        office["id"] = json!("office");
        office["url"] = json!(format!("http://127.0.0.1:{}/office", server.port));
        let this = Self {
            server,
            folder,
            home,
            office,
        };
        this.save(&[this.home.clone(), this.office.clone()]);
        this
    }
    fn save(&self, endpoints: &[Value]) {
        let file = self.folder.path().join("config.json");
        fs::write(
            &file,
            json!({"version":1,"endpoints":endpoints}).to_string(),
        )
        .unwrap();
        fs::set_permissions(file, fs::Permissions::from_mode(0o600)).unwrap();
    }
    fn command(&self) -> Command {
        let mut command = Command::new(
            std::env::var_os("MIILO_OWL_BINARY")
                .unwrap_or_else(|| env!("CARGO_BIN_EXE_miilo-owl").into()),
        );
        command.env("MIILO_OWL_CONFIG_DIR", self.folder.path());
        command
    }
    fn cli(&self, args: &[&str], ok: bool) -> Value {
        let output = self.command().args(args).output().unwrap();
        parse(output, ok)
    }
    fn item(&self) -> Value {
        self.cli(&["refresh"], true)["snapshot"]["workItems"][0].clone()
    }
}
fn parse(output: Output, ok: bool) -> Value {
    assert_eq!(
        output.status.success(),
        ok,
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(!String::from_utf8_lossy(&output.stdout).contains("private-token"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("private-token"));
    serde_json::from_slice(&output.stdout).unwrap()
}
fn s(value: &Value) -> &str {
    value.as_str().unwrap()
}
#[test]
fn observe_explain_confirm_receipt_and_remove() {
    let j = Journey::new();
    let data = j.cli(&["snapshot"], true);
    let first = &data["snapshot"]["workItems"][0];
    let second = &data["snapshot"]["workItems"][1];
    assert_ne!(first["_key"], second["_key"]);
    let explained = j.cli(
        &["explain", s(&second["_key"]), s(&second["_revisionKey"])],
        true,
    );
    assert!(s(&explained["message"]).contains("Recovery passed"));
    assert!(
        j.server
            .state
            .lock()
            .unwrap()
            .payloads
            .last()
            .unwrap()
            .0
            .starts_with("/office/")
    );
    let result = j.cli(
        &[
            "action",
            s(&first["_key"]),
            s(&first["actions"][0]["_key"]),
            s(&first["_revisionKey"]),
        ],
        true,
    );
    assert_eq!(result["status"], "accepted");
    {
        let state = j.server.state.lock().unwrap();
        let (path, invocation) = state.payloads.last().unwrap();
        assert_eq!(path, "/home/v1/actions");
        assert_eq!(invocation["expectedRevision"], "r1");
        assert_eq!(invocation["confirmation"]["method"], "manual");
    }
    j.save(std::slice::from_ref(&j.home));
    assert_eq!(
        j.cli(&["snapshot"], true)["snapshot"]["workItems"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    j.cli(
        &[
            "action",
            s(&second["_key"]),
            s(&second["actions"][0]["_key"]),
            s(&second["_revisionKey"]),
        ],
        false,
    );
}
#[test]
fn changed_revision_and_explanation_rejected() {
    let j = Journey::new();
    let item = j.item();
    j.server.state.lock().unwrap().explanation_revision = Some("r2".into());
    j.cli(
        &["explain", s(&item["_key"]), s(&item["_revisionKey"])],
        false,
    );
    {
        let mut state = j.server.state.lock().unwrap();
        state.payloads.clear();
        state.current["workItems"][0]["revision"] = json!("r2");
    }
    j.cli(
        &[
            "action",
            s(&item["_key"]),
            s(&item["actions"][0]["_key"]),
            s(&item["_revisionKey"]),
        ],
        false,
    );
    assert!(j.server.state.lock().unwrap().payloads.is_empty());
}
#[test]
fn conflict_is_not_retried_and_invalidates_cache() {
    let j = Journey::new();
    let item = j.item();
    j.server.state.lock().unwrap().conflict = true;
    j.cli(
        &[
            "action",
            s(&item["_key"]),
            s(&item["actions"][0]["_key"]),
            s(&item["_revisionKey"]),
        ],
        false,
    );
    assert_eq!(j.server.state.lock().unwrap().payloads.len(), 1);
    assert!(!j.folder.path().join("snapshot-cache.json").exists());
}
#[test]
fn revoked_credentials_redirects_and_invalid_responses() {
    let j = Journey::new();
    let mut revoked = j.home.clone();
    revoked["token"] = json!("revoked");
    j.save(&[revoked]);
    assert_eq!(j.cli(&["refresh"], true)["status"], "partial");
    j.save(std::slice::from_ref(&j.home));
    for mode in ["redirect", "large", "invalid"] {
        j.server.state.lock().unwrap().mode = mode.into();
        assert_eq!(j.cli(&["refresh"], true)["status"], "partial");
    }
    assert!(
        !j.server
            .state
            .lock()
            .unwrap()
            .visits
            .iter()
            .any(|p| p.contains("leaked-token"))
    );
}
#[test]
fn stable_unicode_keys_match_legacy_wire_encoding() {
    let j = Journey::new();
    for (revision, canonical) in [
        ("r1", r#"["r1"]"#),
        ("révision-😀", r#"["r\u00e9vision-\ud83d\ude00"]"#),
        ("quote\"\\\n", r#"["quote\"\\\n"]"#),
        ("delete\u{7f}", r#"["delete\u007f"]"#),
    ] {
        j.server.state.lock().unwrap().current["workItems"][0]["revision"] = json!(revision);
        let item = j.item();
        let expected = format!("{:x}", Sha256::digest(canonical.as_bytes()))[..32].to_owned();
        assert_eq!(item["_revisionKey"], expected);
    }
}
#[test]
fn concurrent_processes_share_one_fetch() {
    let j = Journey::new();
    j.save(std::slice::from_ref(&j.home));
    let children: Vec<_> = (0..3)
        .map(|_| {
            j.command()
                .arg("snapshot")
                .stdout(std::process::Stdio::piped())
                .spawn()
                .unwrap()
        })
        .collect();
    for child in children {
        assert_eq!(
            parse(child.wait_with_output().unwrap(), true)["status"],
            "ready"
        );
    }
    assert_eq!(j.server.state.lock().unwrap().visits.len(), 1);
}
#[test]
fn artifacts_survive_authenticated_cli_snapshot() {
    let j = Journey::new();
    {
        let mut state = j.server.state.lock().unwrap();
        state.current["attempts"] = json!([{"id":"run","providerId":"factory","taskId":"task-1"}]);
        state.current["artifacts"] = json!([{"id":"file","providerId":"factory","attemptId":"run","kind":"file","name":"src/main.rs","extensions":{"custom":{"diff":"-old\n+new"}}}]);
    }
    let data = j.cli(&["snapshot"], true);
    let snap = &data["snapshot"];
    assert_eq!(snap["artifacts"].as_array().unwrap().len(), 2);
    for work in snap["workItems"].as_array().unwrap() {
        let linked: Vec<_> = snap["artifacts"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| {
                work["_artifactKeys"]
                    .as_array()
                    .unwrap()
                    .contains(&a["_key"])
            })
            .collect();
        assert_eq!(linked.len(), 1);
        assert_eq!(linked[0]["providerId"], work["providerId"]);
        assert_eq!(linked[0]["_preview"], "-old\n+new");
    }
}
#[test]
fn credential_file_rotation_is_used_without_rewriting_widget_config() {
    let j = Journey::new();
    let file = j.folder.path().join("ce.token");
    fs::write(&file, "private-token").unwrap();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).unwrap();
    let mut endpoint = j.home.clone();
    endpoint["token"] = json!("");
    endpoint["tokenFile"] = json!(file);
    j.save(&[endpoint]);
    assert_eq!(j.cli(&["refresh"], true)["status"], "ready");
    fs::write(file, "revoked").unwrap();
    assert_eq!(j.cli(&["refresh"], true)["status"], "partial");
}
