# Session operation telemetry

Optional metadata-only recording for the Miilo chat and voice hosts. Reuses the
bounded exporter from the CLI and semantic mapping from the SDK. No prompt,
transcript, audio, credential, endpoint URL or observed work identifier is exported.
Provider/model/token metadata is omitted when unknown. An OpenAI-compatible API
shape does not identify a model provider, so chat is recorded as a `chat_request`
tool operation; Superwhisper transcription is a `transcribe` tool operation.

Export is disabled without an explicit private configuration. Revocation prevents
new records, including pending operations; already queued/delivered spans cannot
be recalled. Capture and wake listening are not inference spans. The voice host
passes only a validated trace-parent identifier to the chat host, with no baggage.

The host must stop capture before dropping its last telemetry owner. Export uses
a bounded background queue; final process shutdown is best effort and capped by
the CLI exporter. Abrupt process termination may lose pending records.
