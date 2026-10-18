# Widget release validation — 2026-10-17

## Implemented behavior

- Compact/Detail navigation preserves configured systems and inspection origin.
  Compact has up to four columns, bounded rails and additional rows.
- Toolbar icons share vector size, stroke, theme foreground and alignment.
  Status labels use centered containers; attention notes and evidence stay visible.
- Failed tasks have asymmetric round eyes; unavailable observations have crossed
  eyes. Attention marks remain present during inspection.
- Shared settings expose the existing consent-based chat-model setup. Voice is
  explicitly not implemented in this launcher.
- Read-only explanation requests allow twenty seconds; observation/actions retain
  eight-second deadlines. Provider actions are not retried.

## Automated evidence

Rust helper tests, including real-socket slow-explanation/action deadlines, passed.
Rust formatting and Clippy passed. Source architecture, no-language-dependency and
release-policy checks passed. Generated desktop execution passed 36 scenarios;
portrait parity passed 54 exact comparisons. Compact viewport, orbit geometry,
robot motion, session visibility and inspection-navigation checks passed.

## Android walkthrough

The actual Omalauncher emulator completed setup of four isolated provider fixtures,
Compact/Detail and attention navigation, notes/evidence/file-diff inspection,
a live OpenAI-compatible model explanation, and a revision-bound confirmed fixture
approval. Observation configuration was isolated and real provider records were
unchanged. This proves the exercised emulator path, not physical-phone or native
Omarchy acceptance. Voice was not exercised.

The companion native plugin consumes matching generated assets with source
hashes and a reproducible provenance patch. Installed Omarchy lifecycle, keyboard,
scaling and performance remain host-specific checks. This checkpoint creates no
release tag, package publication or marketplace submission.
