use opentelemetry::{
    Context,
    trace::{SpanContext, SpanId, TraceContextExt, TraceFlags, TraceId, TraceState},
};
pub fn parse(value: &str) -> Option<Context> {
    let parts: Vec<_> = value.split('-').collect();
    if parts.len() != 4
        || parts[0] != "00"
        || parts[1].len() != 32
        || parts[2].len() != 16
        || parts[3] != "01"
        || !parts[1..3].iter().all(|part| {
            part.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
    {
        return None;
    }
    let span = SpanContext::new(
        TraceId::from_hex(parts[1]).ok()?,
        SpanId::from_hex(parts[2]).ok()?,
        TraceFlags::SAMPLED,
        true,
        TraceState::default(),
    );
    span.is_valid()
        .then(|| Context::new().with_remote_span_context(span))
}
pub fn encode(context: &Context) -> Option<String> {
    let span = context.span();
    let value = span.span_context();
    value
        .is_valid()
        .then(|| format!("00-{}-{}-01", value.trace_id(), value.span_id()))
}
