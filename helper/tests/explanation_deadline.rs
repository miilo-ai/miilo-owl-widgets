//! Exercise slow responses through real HTTP; action deadlines remain unchanged.
use miilo_owl::{
    config::Endpoint,
    transport::{Http, Transport},
};
use serde_json::json;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

fn delayed(path: &str) -> miilo_owl::Result<serde_json::Value> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let worker = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let mut request = Vec::new();
        let mut buf = [0; 1024];
        while !request.windows(4).any(|w| w == b"\r\n\r\n") {
            let n = stream.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            request.extend_from_slice(&buf[..n]);
        }
        thread::sleep(Duration::from_secs(9));
        let body = r#"{"status":"completed"}"#;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        // Action client is expected to have disconnected before this write.
        let _ = stream.write_all(response.as_bytes());
    });
    let endpoint = Endpoint {
        id: "slow".into(),
        url,
        token: String::new(),
        token_file: None,
        providers: vec![],
        allow_actions: true,
        local_snapshot_file: None,
        actor_names: Default::default(),
    };
    let result = Http::new()
        .unwrap()
        .request(&endpoint, path, Some(&json!({})));
    worker.join().unwrap();
    result
}

#[test]
fn explanation_accepts_a_response_beyond_the_old_deadline() {
    assert_eq!(delayed("/v1/explanations").unwrap()["status"], "completed");
}

#[test]
fn slow_action_still_times_out_without_retrying() {
    assert!(delayed("/v1/actions").is_err());
}
