use miilo_owl::adapters::chat_sse;
#[test]
fn metadata_overhead_is_bounded_separately_from_answer_text() {
    let mut stream =
        String::from("data: {\"choices\":[{\"delta\":{\"content\":\"<think>\"}}]}\n\n");
    for _ in 0..1000 {
        stream.push_str(&format!("data: {{\"padding\":\"{}\",\"choices\":[{{\"delta\":{{\"content\":\"private\"}}}}]}}\n\n","x".repeat(300)));
    }
    stream.push_str(
        "data: {\"choices\":[{\"delta\":{\"content\":\"</think>Answer.\"}}]}\n\ndata: [DONE]\n\n",
    );
    let mut answer = String::new();
    chat_sse::read(stream.as_bytes(), &mut |text| {
        answer.push_str(text);
        Ok(())
    })
    .unwrap();
    assert_eq!(answer, "Answer.");
    let excessive = format!("data: {{\"padding\":\"{}\"}}\n\n", "x".repeat(32000)).repeat(270);
    assert!(chat_sse::read(excessive.as_bytes(), &mut |_| Ok(())).is_err());
}

#[test]
fn null_or_empty_tool_fields_are_not_calls_and_thinking_is_not_rendered() {
    for tools in ["null", "[]"] {
        let stream = format!(
            "data: {{\"choices\":[{{\"delta\":{{\"content\":\"<thi\",\"tool_calls\":{tools},\"function_call\":null}}}}]}}\n\ndata: {{\"choices\":[{{\"delta\":{{\"content\":\"nk>private reasoning</think>Final answer.\",\"tool_calls\":{tools}}},\"finish_reason\":\"stop\"}}]}}\n\ndata: [DONE]\n\n"
        );
        let mut answer = String::new();
        chat_sse::read(stream.as_bytes(), &mut |text| {
            answer.push_str(text);
            Ok(())
        })
        .unwrap();
        assert_eq!(answer, "Final answer.");
    }
}
#[test]
fn real_tool_calls_and_incomplete_thoughts_still_fail_closed() {
    for data in [
        "{\"choices\":[{\"delta\":{\"tool_calls\":[{\"function\":{\"name\":\"execute\"}}]}}]}",
        "{\"choices\":[{\"delta\":{\"content\":\"<think>private reasoning\"}}]}",
    ] {
        let stream = format!("data: {data}\n\ndata: [DONE]\n\n");
        let mut answer = String::new();
        assert!(
            chat_sse::read(stream.as_bytes(), &mut |text| {
                answer.push_str(text);
                Ok(())
            })
            .is_err()
        );
        assert!(answer.is_empty());
    }
}
