use miilo_owl::{
    adapters::{chat_config, widget_chat, widget_chat_guard},
    config::Store,
    domain::chat::ChatConfig,
};
use serde_json::Value;
use std::{
    io::{Read, Write},
    net::TcpListener,
    process::{Command, Stdio},
    thread,
};

#[test]
fn nul_wire_preserves_quotes_multiline_and_bounds_history() {
    let (request, share) = widget_chat::request(
        b"test-1\0What is 'this'?\n$(false)\0false\0\0\0user\0Hi\0assistant\0Hello\0",
    )
    .unwrap();
    assert!(!share);
    assert!(request.focus.is_none());
    assert_eq!(request.text, "What is 'this'?\n$(false)");
    assert_eq!(request.history.len(), 2);
    for input in [
        b"no delimiter".as_slice(),
        b"id\0text\0yes\0\0\0",
        b"id\0text\0true\0work\0\0",
    ] {
        assert!(widget_chat::request(input).is_err());
    }
    assert!(widget_chat::request(&vec![0; 64_001]).is_err());
}

#[test]
fn receipt_prevents_automatic_rebilling_without_retaining_transcripts() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store {
        directory: dir.path().join("config"),
    };
    let guard = widget_chat_guard::claim(&store, "one").unwrap();
    assert!(widget_chat_guard::claim(&store, "two").is_err());
    drop(guard);
    assert!(widget_chat_guard::claim(&store, "one").is_err());
    assert!(widget_chat_guard::claim(&store, "two").is_ok());
    let receipt: Value = serde_json::from_reader(
        std::fs::File::open(store.directory.join("widget-chat-request.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(receipt, serde_json::json!({"requestIds":["one","two"]}));
}

#[test]
fn compiled_widget_chat_streams_cumulative_text_without_observation_consent() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store {
        directory: dir.path().join("config"),
    };
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    chat_config::save(
        &store,
        &ChatConfig {
            version: 1,
            base_url: format!("http://{}/v1", listener.local_addr().unwrap()),
            model: "fixture".into(),
            api_key_file: None,
            consent: true,
            max_output_tokens: 128,
            template_thinking: None,
        },
    )
    .unwrap();
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        let mut raw = Vec::new();
        let (end, size) = loop {
            let mut buf = [0; 4096];
            let n = socket.read(&mut buf).unwrap();
            assert!(n > 0);
            raw.extend_from_slice(&buf[..n]);
            if let Some(end) = raw.windows(4).position(|s| s == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&raw[..end]);
                let size: usize = headers
                    .lines()
                    .find_map(|l| {
                        l.to_lowercase()
                            .strip_prefix("content-length:")
                            .map(|s| s.trim().parse().unwrap())
                    })
                    .unwrap();
                if raw.len() >= end + 4 + size {
                    break (end + 4, size);
                }
            }
        };
        let body: Value = serde_json::from_slice(&raw[end..end + size]).unwrap();
        let context = body["messages"][1]["content"].as_str().unwrap();
        assert!(context.contains("context_not_shared"));
        assert!(context.contains("\"work\":[]"));
        let events = "data: {\"choices\":[{\"delta\":{\"content\":\"Hello\"}}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\" there\"}}]}\n\ndata: [DONE]\n\n";
        write!(socket,"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",events.len(),events).unwrap();
    });
    let mut child = Command::new(env!("CARGO_BIN_EXE_miilo-owl"))
        .arg("widget-chat")
        .env("MIILO_OWL_CONFIG_DIR", &store.directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"test-stream\0Hello\0false\0\0\0")
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    let events: Vec<Value> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(events.len(), 4);
    assert_eq!(events[1]["text"], "Hello");
    assert_eq!(events[2]["text"], "Hello there");
    assert_eq!(events[3]["done"], true);
    assert!(events.iter().all(|e| e["error"].is_null()));
    server.join().unwrap();
}
