use miilo_session_telemetry::{Host, Outcome, carrier};
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use prost::Message;
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    thread,
    time::Duration,
};

fn receiver() -> (String, thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}/v1/traces", listener.local_addr().unwrap());
    let handle = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut header = vec![];
        while !header.ends_with(b"\r\n\r\n") {
            let mut byte = [0];
            socket.read_exact(&mut byte).unwrap();
            header.push(byte[0]);
            assert!(header.len() < 8192);
        }
        let text = String::from_utf8(header).unwrap();
        let length: usize = text
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
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .unwrap();
        body
    });
    (endpoint, handle)
}
fn config(path: &std::path::Path, endpoint: &str) {
    fs::write(
        path,
        serde_json::to_vec(
            &serde_json::json!({"enabled":true,"endpoint":endpoint,"tokenFile":null}),
        )
        .unwrap(),
    )
    .unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
}
#[test]
fn actual_otlp_links_tool_to_chat_and_suppresses_revoked_operations() {
    let (endpoint, receiver) = receiver();
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("telemetry.json");
    config(&path, &endpoint);
    let host = Host::load(&path).unwrap().unwrap();
    let parent = host.transcribe().finish(Outcome::Success).unwrap();
    host.chat(carrier::parse(&parent).unwrap())
        .finish(Outcome::Success);
    let revoked = host.transcribe();
    fs::remove_file(&path).unwrap();
    assert!(!host.enabled());
    assert!(revoked.finish(Outcome::Success).is_none());
    host.shutdown();
    let bytes = receiver.join().unwrap();
    let request = ExportTraceServiceRequest::decode(bytes.as_slice()).unwrap();
    let spans: Vec<_> = request
        .resource_spans
        .iter()
        .flat_map(|r| &r.scope_spans)
        .flat_map(|s| &s.spans)
        .collect();
    assert_eq!(spans.len(), 2);
    let tool = spans
        .iter()
        .find(|s| s.name == "execute_tool transcribe")
        .unwrap();
    let chat = spans
        .iter()
        .find(|s| s.name == "execute_tool chat_request")
        .unwrap();
    assert_eq!(chat.trace_id, tool.trace_id);
    assert_eq!(chat.parent_span_id, tool.span_id);
    assert!(
        spans
            .iter()
            .all(|s| s.end_time_unix_nano >= s.start_time_unix_nano)
    );
    assert!(!String::from_utf8_lossy(&bytes).contains(&endpoint));
    for span in spans {
        assert!(span.attributes.iter().all(|a| !a.key.contains("content")
            && !a.key.contains("model")
            && !a.key.contains("token")));
    }
}
#[test]
fn dropped_operation_reports_cancellation_without_raw_error() {
    let (endpoint, receiver) = receiver();
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("telemetry.json");
    config(&path, &endpoint);
    let host = Host::load(&path).unwrap().unwrap();
    drop(host.transcribe());
    host.shutdown();
    let request = ExportTraceServiceRequest::decode(receiver.join().unwrap().as_slice()).unwrap();
    let spans: Vec<_> = request
        .resource_spans
        .iter()
        .flat_map(|r| &r.scope_spans)
        .flat_map(|s| &s.spans)
        .collect();
    assert_eq!(spans.len(), 1);
    assert!(spans[0].attributes.iter().any(|a| a.key.contains("cancel")));
}
