# Release scope and validation

Current published package: 1.7.2 (checked November 20, 2026).
For available assets and subsequent versions, use GitHub Releases and the package manifests.
Documentation-only changes do not require a package version bump.

## Supported scope

Local observation supports Codex, Claude Code, Cursor, OpenCode, Devin CLI,
Grok CLI and Hermes through the CLI. Consult the CLI compatibility matrix for
storage formats, tested tool versions and the distinction between import and live updates.
Local observation is read-only; provider actions require a separately connected provider.

## Outstanding work

- This branch adds unreleased Android chat, native host dictation and opt-in
  completed-answer playback. Emulator fixtures do not establish real microphone,
  audible voice quality or physical-device acceptance. Continuous hands-free
  conversation still needs a supported host integration. See
  [Android conversation](launcher-chat.md#android-conversation).
- The native Omarchy capture/playback implementation belongs to its separate
  plugin and Rust `miilo-voice-session` helper; Android does not use PipeWire.
- Keep platform-specific acceptance explicit: physical Android, Windows local observation,
  fractional scaling, large fleets and hidden/reduced-motion performance need their own evidence.
- Marketplace approval and AUR availability require external confirmation; a public
  repository or GitHub release does not establish a store listing.
- The protected Homebrew tap needs a PR-compatible publishing flow. Formula CI and
  an existing published formula do not establish automatic future tap updates.

## Documentation authority

README and docs describe supported installation and operation. `.specify/specs`
retains design requirements; `.specify/evidence` records dated test results.
Old candidate plans and unchecked prelaunch checklists are historical, not current
release blockers or proof of missing implementation. Do not erase recorded test
limitations or treat screenshots as exhaustive platform certification.
