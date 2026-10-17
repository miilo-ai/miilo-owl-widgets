use miilo_adapter_sdk::ChatRequest;
use miilo_owl::{
    adapters::{chat_config, chat_http::OpenAiChat},
    application::chat::ChatBackend,
    domain::chat::{ChatConfig, ChatContext},
};
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};
fn config(url: String) -> ChatConfig {
    ChatConfig {
        version: 1,
        base_url: url,
        model: "example-small".into(),
        api_key_file: None,
        consent: true,
    }
}
fn turn() -> ChatRequest {
    serde_json::from_value(
        json!({"protocolVersion":"0.1","requestId":"turn-1","text":"What needs me?"}),
    )
    .unwrap()
}
fn ctx() -> ChatContext {
    ChatContext {
        status: "partial".into(),
        observed_at: 10.0,
        work: vec![],
        omitted: 0,
    }
}
#[test]
fn endpoint_policy_rejects_credentials_queries_and_insecure_remote() {
    for url in [
        "http://example.invalid/v1",
        "https://user:secret@example.invalid/v1",
        "https://example.invalid/v1?token=x",
        "https://example.invalid/chat",
    ] {
        assert!(chat_config::validate(&config(url.into())).is_err());
    }
    assert!(chat_config::validate(&config("http://127.0.0.1:4000/v1".into())).is_ok());
}
#[test]
fn actual_http_request_streams_and_does_not_enable_tools() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/v1", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let mut raw = vec![];
        let mut buf = [0; 1024];
        let header_end = loop {
            let n = socket.read(&mut buf).unwrap();
            assert!(n > 0);
            raw.extend_from_slice(&buf[..n]);
            if let Some(i) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
                break i + 4;
            }
        };
        let headers = String::from_utf8_lossy(&raw[..header_end]);
        assert!(headers.starts_with("POST /v1/chat/completions"));
        let length = headers
            .lines()
            .find_map(|l| {
                l.to_lowercase()
                    .strip_prefix("content-length:")
                    .map(|s| s.trim().parse::<usize>().unwrap())
            })
            .unwrap();
        while raw.len() - header_end < length {
            let n = socket.read(&mut buf).unwrap();
            raw.extend_from_slice(&buf[..n]);
        }
        let request: Value = serde_json::from_slice(&raw[header_end..header_end + length]).unwrap();
        assert_eq!(request["model"], "example-small");
        assert_eq!(request["stream"], true);
        assert!(request.get("tools").is_none());
        let body = "data: {\"choices\":[{\"delta\":{\"content\":\"No current work.\"}}]}\n\ndata: [DONE]\n\n";
        write!(socket,"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",body.len(),body).unwrap();
    });
    let mut answer = String::new();
    OpenAiChat::new()
        .unwrap()
        .stream(&config(url), &turn(), &ctx(), &mut |s| {
            answer.push_str(s);
            Ok(())
        })
        .unwrap();
    assert_eq!(answer, "No current work.");
    server.join().unwrap();
}
#[test]
fn configuration_roundtrip_is_private_and_preserves_other_connections() {
    use std::os::unix::fs::PermissionsExt;
    let temp = tempfile::tempdir().unwrap();
    let store = miilo_owl::config::Store {
        directory: temp.path().join("config"),
    };
    let original = miilo_owl::config::Config::default();
    store.save(&original).unwrap();
    chat_config::save(&store, &config("https://example.invalid/v1".into())).unwrap();
    assert_eq!(chat_config::load(&store).unwrap().model, "example-small");
    assert_eq!(
        std::fs::metadata(store.directory.join("chat.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o077,
        0
    );
    assert!(store.load().unwrap().endpoints.is_empty());
}
