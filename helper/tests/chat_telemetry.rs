use miilo_adapter_sdk::ChatRequest;
use miilo_owl::{
    adapters::{chat_config, chat_http::OpenAiChat, telemetry},
    application::chat::ChatBackend,
    config::Store,
    domain::chat::{ChatConfig, ChatContext},
};
use miilo_session_telemetry::Context;
use serde_json::json;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};
fn server(response: &str, content_type: &str) -> (String, thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let response = response.to_owned();
    let content_type = content_type.to_owned();
    let handle = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut headers = vec![];
        while !headers.ends_with(b"\r\n\r\n") {
            let mut byte = [0];
            socket.read_exact(&mut byte).unwrap();
            headers.push(byte[0]);
            assert!(headers.len() < 8192);
        }
        let headers = String::from_utf8(headers).unwrap();
        let length: usize = headers
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse().unwrap())
            })
            .unwrap();
        assert!(length < 100_000);
        let mut body = vec![0; length];
        socket.read_exact(&mut body).unwrap();
        write!(socket,"HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}",response.len()).unwrap();
        body
    });
    (url, handle)
}
#[test]
fn actual_chat_stream_exports_metadata_without_question_or_reply() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store {
        directory: dir.path().into(),
    };
    let (collector, export) = server("", "application/x-protobuf");
    telemetry::dispatch(
        &store,
        &[
            "configure",
            "--consent",
            "--endpoint",
            &format!("{collector}/v1/traces"),
        ],
    )
    .unwrap();
    let host = telemetry::load(&store).unwrap();
    let reply = "private-reply-fixture";
    let (endpoint, model) = server(
        &format!(
            "data: {{\"choices\":[{{\"delta\":{{\"content\":\"{reply}\"}}}}]}}\n\ndata: [DONE]\n\n"
        ),
        "text/event-stream",
    );
    let request: ChatRequest = serde_json::from_value(json!({"protocolVersion":"0.1","requestId":"private-id-fixture","text":"private-question-fixture"})).unwrap();
    let config = ChatConfig {
        version: 1,
        base_url: format!("{endpoint}/v1"),
        model: "private-model-fixture".into(),
        api_key_file: None,
        consent: true,
    };
    let context = ChatContext {
        status: "ok".into(),
        observed_at: 0.,
        work: vec![],
        omitted: 0,
    };
    let mut output = String::new();
    OpenAiChat::new()
        .unwrap()
        .traced(Some(host.clone()), Context::new())
        .stream(&config, &request, &context, &mut |text| {
            output.push_str(text);
            Ok(())
        })
        .unwrap();
    assert_eq!(output, reply);
    host.shutdown();
    let exported = export.join().unwrap();
    let text = String::from_utf8_lossy(&exported);
    assert!(text.contains("execute_tool chat_request"));
    for marker in [
        "private-question-fixture",
        "private-reply-fixture",
        "private-id-fixture",
        "private-model-fixture",
        &endpoint,
        &collector,
    ] {
        assert!(!text.contains(marker));
    }
    let sent = model.join().unwrap();
    assert!(String::from_utf8_lossy(&sent).contains("private-question-fixture"));
    telemetry::dispatch(&store, &["disable"]).unwrap();
    assert!(telemetry::load(&store).is_none());
    assert!(chat_config::validate(&config).is_ok());
}
