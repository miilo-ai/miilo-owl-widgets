# Optional operation telemetry

Miilo does not export telemetry by default. This option is separate from consent
to send questions or observed context to a chat model.

Explicitly configure a collector using the full OTLP/HTTP protobuf traces URL:

```sh
miilo-owl telemetry configure --consent --endpoint https://collector.example/v1/traces
miilo-owl telemetry status
miilo-owl telemetry disable
```

For a collector requiring authentication, add `--token-file /absolute/private/file`.
The file must be owned by the current user with no group/other permissions. Its
credential is separate from the model credential. HTTPS is required except for
numeric loopback addresses used in local testing. Credentials, query parameters
and fragments are forbidden in the collector URL.

Configuration is stored privately beside the helper's connection configuration.
Restart an active hands-free session after enabling telemetry. Disabling export
suppresses future and still-pending records, including active voice operations.
Already queued or delivered spans cannot be recalled.

Records describe actual chat requests and completed Superwhisper transcription
operations. Wake listening and snapshot polling are not exported. Unknown provider,
model and token information is omitted; an OpenAI-compatible endpoint does not
establish provider identity, so these requests use tool-operation spans. Speech
content, questions, replies, observed context, work identifiers, model credentials
and endpoint URLs are excluded. The voice host passes only a validated trace-parent
identifier to the chat host, with no baggage or automatic downstream propagation.

A bounded background exporter isolates collector failures from successful work.
Final shutdown is best effort and bounded; abrupt process termination can lose
records. Trace delivery is not a guarantee that a remote backend retained them.
