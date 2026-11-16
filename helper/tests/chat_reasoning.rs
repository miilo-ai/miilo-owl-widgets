use miilo_owl::adapters::chat_sse;

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
