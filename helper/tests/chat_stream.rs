use miilo_owl::adapters::chat_sse;
fn parse(body: &[u8]) -> (miilo_owl::Result<()>, String) {
    let mut answer = String::new();
    let result = chat_sse::read(body, &mut |s| {
        answer.push_str(s);
        Ok(())
    });
    (result, answer)
}
#[test]
fn sse_handles_crlf_comments_metadata_and_utf8() {
    let bytes = ":keepalive\r\ndata: {\"choices\":[{\"delta\":{\"role\":\"assistant\"}}]}\r\n\r\ndata: {\"choices\":[{\"delta\":{\"content\":\"Héllo\"}}]}\r\n\r\ndata: [DONE]\r\n\r\n";
    let (result, answer) = parse(bytes.as_bytes());
    result.unwrap();
    assert_eq!(answer, "Héllo");
}
#[test]
fn truncated_malformed_and_oversized_streams_fail() {
    for body in [
        "data: {}\n\n",
        "data: broken\n\n",
        "data: {\"error\":{\"message\":\"secret\"}}\n\n",
    ] {
        assert!(parse(body.as_bytes()).0.is_err());
    }
    assert!(
        parse(format!("data: {}\n\n", "x".repeat(65_537)).as_bytes())
            .0
            .is_err()
    );
}
#[test]
fn tool_calls_and_truncation_cannot_become_success() {
    for body in [
        r#"{"choices":[{"delta":{"tool_calls":[]}}]}"#,
        r#"{"choices":[{"delta":{},"finish_reason":"length"}]}"#,
    ] {
        assert!(
            parse(format!("data: {body}\n\ndata: [DONE]\n\n").as_bytes())
                .0
                .is_err()
        );
    }
}
#[test]
fn callback_failure_cancels_parsing() {
    let body = b"data: {\"choices\":[{\"delta\":{\"content\":\"Hello\"}}]}\n\ndata: [DONE]\n\n";
    assert!(chat_sse::read(body.as_slice(), &mut |_| Err("cancelled".into())).is_err());
}
