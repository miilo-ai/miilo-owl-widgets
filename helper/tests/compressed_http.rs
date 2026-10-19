use miilo_adapter_sdk::compression::{Codec, Limits, codecs};
use miilo_owl::{
    config::Endpoint,
    transport::{Http, Transport},
};
use std::{
    io::{Read, Write},
    net::TcpListener,
};
#[test]
fn widget_decodes_negotiated_json_and_bounds_decoded_output() {
    for (name, codec) in [
        ("gzip", Codec::Gzip),
        ("br", Codec::Brotli),
        ("zstd", Codec::Zstd),
    ] {
        for oversized in [false, true] {
            let raw = if oversized {
                vec![b' '; 2_000_001]
            } else {
                br#"{"fixture":"decoded","protocolVersion":"0.1"}"#.to_vec()
            };
            let encoded = codecs::compress(codec, &raw, Limits::default()).unwrap();
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let url = format!("http://{}", listener.local_addr().unwrap());
            let worker = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                    .unwrap();
                let mut request = Vec::new();
                let mut buf = [0u8; 1024];
                while !request.windows(4).any(|w| w == b"\r\n\r\n") {
                    let n = stream.read(&mut buf).unwrap();
                    if n == 0 {
                        break;
                    }
                    request.extend_from_slice(&buf[..n]);
                }
                assert!(
                    String::from_utf8_lossy(&request)
                        .to_lowercase()
                        .contains("accept-encoding:")
                );
                write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Encoding: {name}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",encoded.len()).unwrap();
                stream.write_all(&encoded).unwrap();
            });
            let endpoint = Endpoint {
                paired_host: None,
                id: "fixture".into(),
                url,
                token: String::new(),
                token_file: None,
                providers: vec![],
                allow_actions: false,
                local_snapshot_file: None,
                actor_names: Default::default(),
            };
            let result = Http::new()
                .unwrap()
                .request(&endpoint, "/v1/snapshot", None);
            if oversized {
                assert!(result.is_err());
            } else {
                assert_eq!(result.unwrap()["fixture"], "decoded");
            }
            worker.join().unwrap();
        }
    }
}
