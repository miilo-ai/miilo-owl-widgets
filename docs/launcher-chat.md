# Launcher text chat

Chat contracts and the native helper are implemented. Model-backed Explain and
full conversational chat-panel acceptance are separate capabilities. See
[release status](release-status.md) for current validation limits.
Plan: `.specify/specs/003-launcher-chat/spec.md`.

## Configure

Install the matching helper, then open **Settings → Chat model → Configure chat model**, or run `miilo-owl chat-setup` in a terminal. Enter an OpenAI-compatible base URL ending in `/v1`, model name and
optional absolute path to a private API key file. HTTPS is required except on
loopback. The setup asks permission to send normalized observation context.
Saving config does not claim that an endpoint has been tested.

The file is `chat.json` in the existing private helper configuration directory.
It is mode 600 and separate from observation connections. Tokens are read from
the referenced private file on each turn; never put them in widgets or Git.
Normalized summaries may still contain sensitive information. Inspect context
before enabling a remote model; this feature is not an automatic PII scrubber.

## Exercise the helper

With enrolled observations updating, send a bounded JSON turn on stdin:

```sh
printf '%s\n' '{"protocolVersion":"0.1","requestId":"try-1","text":"What needs my attention?"}' | miilo-owl chat
```

Output is NDJSON: `context`, ordered `text_delta`, then `done` or `error`.
An error after deltas means the partial answer is incomplete. Consumers must
not treat exit zero as success: inspect the terminal event. Invalid command/input
may use the existing generic failure output and nonzero exit.
`chat-context` accepts the same stdin and prints the exact bounded context
without contacting the model. Optional `focus` binds `workKey` and `revisionKey`
from the observed workflow; optional `history` carries at most eight user/assistant
messages. The caller owns history in memory and clears it on target change.
No transcript enrollment, tool execution or historical search occurs in chat.
Current observations are forcibly refreshed each turn. Closing the helper process
cancels its request; native UI must terminate the owned process on cancel/close
and discard events from an earlier requestId.

## Voice placeholder

`miilo-owl voice` returns `status: not_implemented`, `specId: VOICE-001`.
See `.specify/specs/004-launcher-voice/spec.md`. It does not request microphone
permission or record audio. Existing provider/mobile speech is not removed.

## Qualification

Tests cover actual loopback HTTP/SSE, compiled helper plus a private local
snapshot with no CE, provider-isolated evidence, stale selection, consent,
partial observations, malformed/truncated streams and cancellation callbacks.
Real Gemma latency, graphical rendering and native microphone acceptance are
not established by these fixtures.

The helper imports the SDK at the exact revision in helper/Cargo.toml. Public
checkouts require no private dependency token. Never embed runtime model credentials
in CI, images, widgets or source control.
