use miilo_owl::{
    adapters::widget_speech::{self, SpeechConfig},
    config::Store,
};
use std::{
    io::{Read, Write},
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    thread,
};
fn config(url: String) -> SpeechConfig {
    SpeechConfig {
        version: 1,
        base_url: url,
        model: "fixture-speech".into(),
        voice: "fixture".into(),
        api_key_file: None,
        consent: true,
    }
}
#[test]
fn speech_requires_explicit_consent_and_safe_endpoint() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store {
        directory: dir.path().join("config"),
    };
    for url in [
        "http://example.com/v1",
        "https://u:p@example.com/v1",
        "https://example.com/v1?token=x",
    ] {
        assert!(widget_speech::save(&store, &config(url.into())).is_err());
    }
    let mut denied = config("https://example.com/v1".into());
    denied.consent = false;
    assert!(widget_speech::save(&store, &denied).is_err());
    assert!(widget_speech::synthesize(&store, "").is_err());
    assert!(widget_speech::synthesize(&store, &"x".repeat(16001)).is_err());
}
fn server(body: Vec<u8>, status: &str) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/v1", listener.local_addr().unwrap());
    let status = status.to_owned();
    let handle = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        let mut raw = Vec::new();
        loop {
            let mut buf = [0; 4096];
            let n = socket.read(&mut buf).unwrap();
            assert!(n > 0);
            raw.extend_from_slice(&buf[..n]);
            if let Some(end) = raw.windows(4).position(|s| s == b"\r\n\r\n") {
                let header = String::from_utf8_lossy(&raw[..end]);
                assert!(header.starts_with("POST /v1/audio/speech "));
                let size: usize = header
                    .lines()
                    .find_map(|s| {
                        s.to_lowercase()
                            .strip_prefix("content-length:")
                            .map(|v| v.trim().parse().unwrap())
                    })
                    .unwrap();
                if raw.len() >= end + 4 + size {
                    let request: serde_json::Value =
                        serde_json::from_slice(&raw[end + 4..end + 4 + size]).unwrap();
                    assert_eq!(request["input"], "Synthetic audio test");
                    assert_eq!(request["response_format"], "wav");
                    break;
                }
            }
        }
        write!(
            socket,
            "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        )
        .unwrap();
        socket.write_all(&body).unwrap();
    });
    (url, handle)
}
#[test]
fn synthesized_audio_is_private_bounded_and_cleanup_is_scoped() {
    let mut wav = vec![0; 44];
    wav[..4].copy_from_slice(b"RIFF");
    wav[8..12].copy_from_slice(b"WAVE");
    let (url, server) = server(wav.clone(), "200 OK");
    let dir = tempfile::tempdir().unwrap();
    let store = Store {
        directory: dir.path().join("config"),
    };
    widget_speech::save(&store, &config(url)).unwrap();
    let unrelated = store.directory.join("keep.wav");
    std::fs::write(&unrelated, b"keep").unwrap();
    let file = widget_speech::synthesize(&store, "Synthetic audio test").unwrap();
    server.join().unwrap();
    assert_eq!(std::fs::read(&file).unwrap(), wav);
    assert_eq!(
        std::fs::metadata(&file).unwrap().permissions().mode() & 0o777,
        0o600
    );
    widget_speech::cleanup(&store).unwrap();
    assert!(!std::path::Path::new(&file).exists());
    assert!(unrelated.exists());
}
#[test]
fn rejected_or_non_audio_responses_cannot_reach_playback() {
    for (body, status) in [(b"not audio".to_vec(), "200 OK"), (vec![], "403 Forbidden")] {
        let (url, server) = server(body, status);
        let dir = tempfile::tempdir().unwrap();
        let store = Store {
            directory: dir.path().join("config"),
        };
        widget_speech::save(&store, &config(url)).unwrap();
        assert!(widget_speech::synthesize(&store, "Synthetic audio test").is_err());
        server.join().unwrap();
        assert!(!std::fs::read_dir(&store.directory).unwrap().any(|e| {
            e.unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("widget-speech-")
        }));
    }
}
