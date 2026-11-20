# Launcher speech follow-up (VOICE-001)

> Retained design record. Unchecked historical milestones are not a current release checklist. See docs/release-status.md for current scope and outstanding validation.


Status: superseded. Native Omarchy voice is owned by the separate Miilo One plugin
and its Rust `miilo-voice-session` helper. OmaLauncher widgets remain text-based.
Existing web/mobile/provider speech contracts and implementations stay intact.

The original unused shared-helper voice placeholder has been removed. No launcher
widget called it, and it must not be advertised as a microphone entry point.
See `docs/launcher-chat.md` for current ownership and installation boundaries.
The remaining host-adapter requirements below are retained design context, not
a claim that OmaLauncher has native microphone support.

Host adapters own permission, capture readiness, VAD/endpointing and playback.
Core owns explicit turn state and cancellation. Provider adapters support
OpenAI-compatible transcription (/v1/audio/transcriptions) and synthesis
(/v1/audio/speech), model/voice/format discovery and connection tests.
Device dictation/TTS are optional host-specific alternatives, not assumed parity.

Acceptance: no clipped initial speech, automatic endpointing, explicit cancel,
interruption drops pending audio, no actions from partial transcripts, text fallback,
denied permission/offline recovery, latency measurements, ephemeral audio cleanup.
Never copy saved clip libraries or public environment API keys from other apps.
Each host's microphone and interruption support requires native proof.
