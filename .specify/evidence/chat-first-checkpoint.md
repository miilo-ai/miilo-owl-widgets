# Chat-first source checkpoint — 2026-10-17

Implemented: SDK text contracts and shared helper chat-setup/chat-context/chat.
Voice returns an explicit VOICE-001 not_implemented result; no microphone starts.
No graphical chat, native host acceptance or actual Gemma inference is claimed.

Passed: helper 53 Rust tests (13 new chat cases), 5 release-tool tests, 2 SDK
access-policy tests, architecture gate plus 3 negative tests, warnings-denied
helper/tooling lint, workflow actionlint and shell syntax. Existing desktop
OWL evaluator: 36 scenarios. SDK: 12 Rust tests, 7 Python tests, 18 protocol
schema tests and TypeScript type checks. Compiled helper fixture proves local
snapshot → chat context → loopback model SSE → NDJSON without CE.

Native follow-up: plugin .specify/handoffs/chat-native.md. Pending: shared/native
UI stream bridge, context retention and cancellation on real hosts, live configured
model proof, installed Linux artifacts and existing pre-publication release gates.
Docker credential-mount wiring was source/lint checked, not executed locally.
No release has been made.
