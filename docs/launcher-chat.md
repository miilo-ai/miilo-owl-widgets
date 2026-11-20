# Launcher text chat

This branch adds an unreleased in-place OmaLauncher conversation panel. It
requires a matching helper and OWL 15 or later; Android acceptance uses official
OmaLauncher 1.8.0. Published 1.7.2 does not contain this panel. Model-backed Explain
and conversational chat are separate capabilities. See
[release status](release-status.md) for current validation limits.
Plan: `.specify/specs/003-launcher-chat/spec.md`.

## Configure

Install the matching helper, then open **Settings → Chat model → Configure chat model**, or run `miilo-owl chat-setup` in a terminal. Enter an OpenAI-compatible base URL ending in `/v1`, model name and
optional absolute path to a private API key file. HTTPS is required except on
loopback. The setup asks permission to send normalized observation context.
Saving config does not claim that an endpoint has been tested.

The file is `chat.json` in the existing private helper configuration directory.
Optional `maxOutputTokens` sets the completion budget from 128 through 8192;
existing configurations default to 512. Reasoning models may need a larger
budget to produce a final answer. Truncated output remains an incomplete turn
and is never committed as a successful answer.
For a server that explicitly supports `chat_template_kwargs.enable_thinking`,
optional `templateThinking` configures that boolean. It is omitted by default;
do not set it for an incompatible provider. Standard separate reasoning fields
and streamed `<think>` blocks are not rendered as answers. SSE framing overhead
has its own 8 MiB cap; individual frames remain bounded at 64 KiB.
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

## Android conversation

Tap the Miilo face beside search. Configure the model through **Model settings**;
the composer stays unavailable until a capable helper and model configuration
are present. Type a question and use keyboard Send or the circular send control.
Replies stream into the current conversation. Stop reply discards late events.
The host may keep a detached stream alive for five seconds before terminating
it; stopping display does not guarantee that provider usage stops immediately.
Recent request identifiers prevent automatic resubmission after stream restarts.

**Share context** starts off. Enabling it sends bounded current work summaries
and evidence; opening chat from a work item binds that item's current revision.
Changing sharing clears conversation carryover. With sharing off, chat does not
read the observation snapshot. The UI retains at most two recent turns in memory;
the model receives up to 1,000 characters from each of those four messages.
New conversation clears that history. No user/assistant transcript is persisted
by Miilo. Questions and history pass through the trusted launcher's transient
shell command, with each field shell-quoted and transferred to Rust on stdin;
this is not isolation from the device owner or a compromised host.

The microphone appears only when Android reports a recognizer. Tapping it asks
for Android microphone permission. Tap again to finish dictation, then review
and send the draft. Denial leaves typed chat available. Dictation uses the
Android recognizer selected by the host; this is not a claim of local-only
transcription or Superwhisper support on Android. Continuous hands-free rearming
is not supported by the widget API: microphone starts require a user gesture.

For playback, configure **Settings / Spoken replies** or run
`miilo-owl speech-setup`. Explicitly choose an HTTPS OpenAI-compatible `/v1`
endpoint, speech model, voice and optional protected credential-file path.
Loopback HTTP is permitted. Configuration requires consent to send reply text.
Enable **Spoken replies** in Ask Miilo when wanted; it starts off. Android owns
audio focus/playback through the host's `sound` API. This implementation
synthesizes the completed answer, not streaming audio or barge-in.

Speech responses must be WAV and at most 5 MB. Derived audio is kept in a mode
600 temporary file in the helper's private directory; a subsequent synthesis
removes earlier files. `miilo-owl speech-cleanup` explicitly removes this audio
cache. A crash or closing the host can leave the protected file until cleanup.
No credential is sent to a different speech endpoint without configuring it.

## Native Omarchy voice

The unused legacy `miilo-owl voice` placeholder has been removed and is no longer
an advertised or supported command. It does not forward to a microphone helper.

[Miilo One for Omarchy](https://github.com/miilo-ai/miilo-omarchy-plugin/blob/main/docs/voice.md)
provides in-place chat and hands-free conversation using its separate Rust
`miilo-voice-session` helper, Superwhisper transcription and configured speech
playback. That native UI owns consent, wake detection, cancellation and playback;
the shared `miilo-owl chat --spoken` command supplies conversational model text.
Installing the OmaLauncher widgets does not install or activate that Linux stack.

## Qualification

Tests cover actual loopback HTTP/SSE, compiled helper plus a private local
snapshot with no CE, provider-isolated evidence, stale selection, consent,
partial observations, malformed/truncated streams and cancellation callbacks.
Android 1.8.0 emulator evidence includes typed/follow-up replies, sharing disabled,
synthetic provider failure, explicit cancellation, the Android permission prompt
and denial, and silent WAV playback. Real microphone transcription, audible
voice quality, device latency and physical-phone acceptance remain separate.

The helper imports the SDK at the exact revision in helper/Cargo.toml. Public
checkouts require no private dependency token. Never embed runtime model credentials
in CI, images, widgets or source control.
