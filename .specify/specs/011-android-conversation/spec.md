# Android conversation

Status: implementation candidate; not a published release.

## Contract

- OWL 15 host, matching helper capability `chat-status.widgetApi = 1`.
- Miilo face opens in-place conversation; no coding-agent tool authority.
- Context sharing and spoken replies initially off. Changing sharing clears history.
- No snapshot access without sharing. Selected work uses fresh revision validation.
- SDK chat policy remains authoritative; UI history is bounded to two turns, and
  outgoing history excerpts to 1,000 characters per message. Typed input is at
  most 4,000 characters; stdin and streamed answers are bounded separately.
- NUL-delimited, shell-quoted stdin; prompts/credentials never used as file names.
- Cumulative NDJSON is correlated by request identifier. Stop discards late events.
- A private, locked receipt contains at most 128 recent IDs, without transcripts,
  to reject model resubmission when the host restarts the same stream.
- Android owns permissions, dictation and audio focus. Dictation produces a draft;
  it does not implicitly submit or automatically rearm the microphone.
- Speech endpoint/model/voice and credential file are separately consented to.
  HTTPS except loopback, no redirects, bounded WAV, protected temporary audio.
- Completed-answer synthesis only; streaming audio, barge-in, wake words and
  continuous mobile conversation are not implemented by this candidate.

## Verification

- Compiled Rust helper against loopback HTTP/SSE; cumulative deltas and terminal
  errors, no observation access when sharing is off, replay/concurrency guard.
- OWL handler tests: legacy-helper/configuration gating, actual shell stdin
  round-trip with hostile quotes, stale request IDs, keyboard Send, dictation
  draft and cancellation, non-persistence.
- Speech fixtures: endpoint/consent checks, WAV bounds, private file permissions,
  scoped cleanup and provider rejection/non-audio failure.
- Official pinned compiler and emulator visual/interaction checks.
- Physical microphone and audible playback qualification remain release gates.

## Host limits

Official OmaLauncher 1.8.0 permits `speech` start/toggle only in user handlers.
Its widget dictation holds across pauses. Its separate agent voice sessions are
not a Miilo model adapter. A future hands-free integration must be explicit
upstream work, not simulated gestures or silently granting an agent tools.
Detached command streams have a five-second host linger; UI cancellation is not
a promise of immediately stopping model-provider billing.
