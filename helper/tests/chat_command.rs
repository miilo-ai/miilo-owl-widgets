mod common;
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::TcpListener,
    process::{Command, Stdio},
    thread,
};
#[test]
fn compiled_helper_streams_a_fresh_local_workflow_without_ce() {
    let dir = tempfile::tempdir().unwrap();
    let mut raw = common::fixture();
    raw["observedAt"] = json!(chrono::Utc::now().to_rfc3339());
    let snapshot_file = dir.path().join("snapshot.json");
    std::fs::write(&snapshot_file, serde_json::to_vec(&raw).unwrap()).unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&snapshot_file, std::fs::Permissions::from_mode(0o600)).unwrap();
    let mut endpoint = common::endpoint();
    endpoint.local_snapshot_file = Some(snapshot_file);
    endpoint.url = "local://desktop".into();
    endpoint.token = String::new();
    endpoint.allow_actions = false;
    let store = common::store(dir.path(), vec![endpoint]);
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/v1", listener.local_addr().unwrap());
    miilo_owl::adapters::chat_config::save(
        &store,
        &miilo_owl::domain::chat::ChatConfig {
            version: 1,
            base_url: url,
            model: "fixture-small".into(),
            api_key_file: None,
            consent: true,
        },
    )
    .unwrap();
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let mut raw = vec![];
        let mut buf = [0; 1024];
        let end = loop {
            let n = socket.read(&mut buf).unwrap();
            assert!(n > 0);
            raw.extend_from_slice(&buf[..n]);
            if let Some(i) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
                break i + 4;
            }
        };
        let headers = String::from_utf8_lossy(&raw[..end]);
        let length = headers
            .lines()
            .find_map(|l| {
                l.to_lowercase()
                    .strip_prefix("content-length:")
                    .map(|s| s.trim().parse::<usize>().unwrap())
            })
            .unwrap();
        while raw.len() - end < length {
            let n = socket.read(&mut buf).unwrap();
            raw.extend_from_slice(&buf[..n]);
        }
        let body: Value = serde_json::from_slice(&raw[end..end + length]).unwrap();
        assert_eq!(body["model"], "fixture-small");
        assert!(
            body["messages"][1]["content"]
                .as_str()
                .unwrap()
                .contains("Review recovery")
        );
        let response = "data: {\"choices\":[{\"delta\":{\"content\":\"This workflow needs review.\"}}]}\n\ndata: [DONE]\n\n";
        write!(socket,"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",response.len(),response).unwrap();
    });
    let mut process = Command::new(env!("CARGO_BIN_EXE_miilo-owl"))
        .arg("chat")
        .env("MIILO_OWL_CONFIG_DIR", &store.directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(br#"{"protocolVersion":"0.1","requestId":"journey","text":"What needs me?"}"#)
        .unwrap();
    let output = process.wait_with_output().unwrap();
    assert!(output.status.success());
    let events: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(events[0]["type"], "context");
    assert_eq!(events[0]["references"][0]["title"], "Review recovery");
    assert_eq!(events[1]["text"], "This workflow needs review.");
    assert_eq!(events[2]["type"], "done");
    assert!(events.iter().all(|e| e["requestId"] == "journey"));
    server.join().unwrap();
}
