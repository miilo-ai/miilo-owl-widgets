# Rust helper validation — October 14, 2026

Port based on main `db8a957` (dismissible in-widget setup), fetched before editing.

## Implemented boundary

The deployed widget helper is Rust. OWL still describes the native UI; Python remains only in developer assembly/inspection/black-box test tooling. There is no Python package declaration in the widgets and no interpreter/runtime download in the installer. The bundle includes a static, stripped ARM64 Linux executable (~2.2 MiB), the three generated widgets, installer, and checksums.

`Transport` abstracts provider HTTP; `Console` abstracts onboarding input. Configuration/storage, snapshot aggregation/cache, provider projection, graph links, and operations are separate small modules. Runtime commands and version-1 config remain compatible. Public trust roots are bundled through Rustls; custom private CA configuration is not yet implemented.

## Evidence

- 13 Rust tests pass on macOS ARM64 and ARM64 Linux/musl.
- 8 black-box tests pass against the compiled CLI on both platforms: authenticated HTTP observation/explanation/action, stale revision rejection, one-shot HTTP conflicts, redirect refusal, revoked credentials, response bounds, old opaque-key compatibility including Unicode, concurrent cache reuse, and installer behavior.
- 20 factories/1,000 tasks exercise isolated IDs and linked evidence/agents.
- All three widgets compile in the official OWL compiler; 123 runtime tile/screen evaluations pass across sizes, themes, filtered/multiple/empty/partial data and confirmation handlers.
- Formatter, Clippy with warnings denied, and shell syntax checks pass locally.
- GitHub Actions configured for these tests and a static ARM64 bundle; remote CI execution is not claimed here.

## Actual Android emulator

Pixel 3a AVD, ARM64 Android 14/API 34; official Omalauncher 0.7.2 release APK. Arch Linux ARM bootstrapped successfully through PRoot. No Python package was installed by this work.

Installed the release bundle through a localhost-only ADB reverse connection. `miilo-owl --version` reports 1.3.0; `miilo-owl snapshot` returns a valid unconfigured snapshot. Tapping Set up in the actual expanded Miilo widget opens the Rust onboarding prompt. The official `owl-check` reports that all three files compile; it skips automatic previews because the widget definitions contain an embedded Terminal. Do not interpret that skip as generated native PNGs.

Evidence screenshots: [native helper execution](../previews/native-rust-helper.png), [native onboarding](../previews/native-rust-onboarding.png).

## Still to prove

- Reachability and authenticated registration against the user's tower Miilo API.
- Live provider snapshots and provider-supported actions from the emulator/phone.
- Physical-phone layouts, scrolling, theme changes, and full setup/confirmation interactions.

No real provider, synthetic approval, or production token was added to the emulator. An empty fresh install is expected until the user selects a reachable backend and its registered providers.

## October 15 interaction and evidence validation

Current suite: 19 Rust tests, 9 authenticated CLI/installer tests, and 255 OWL evaluations. Approve, Reject, and Retry handlers each verify propose-only first tap, Cancel without dispatch, explicit Confirm with one dispatch, pending-request suppression, and stale-proposal rejection. SDK privileged/explicit approvals with empty-object schemas are accepted; destructive/strong/form-input actions remain delegated to the provider. Native emulator rendering separately verified the three buttons and Confirm/Cancel on a clearly labeled local fixture; no real Factory action was executed. The fixture was removed and real observation restored.

Live tower observation preserved 93 artifacts with 38 nonempty previews, linked to 18 tasks. Current live cards advertise no available actions, and the connection remains observation-only; details now explain both conditions. Local action opt-in cannot invent an upstream action or grant provider authorization. Native inline-header changes and their reproducible OWL8 generator are documented in `launcher-header.md`; the default release remains stock-compatible OWL6.

## Stock-compatible search toolbar

Cog and refresh now share the search row using OWL6, with centered icon hit targets. Native checks used the exact default install file, not an OWL8 header variant: tap expands the focused input, typing filters the real feed, and the finish control restores the toolbar while retaining the query. Runtime tests cover opening/finishing/clearing search for all widget sources; action and evidence tests remain included. The archived launcher patch only controls native host UI and is no longer a prerequisite for toolbar placement.

Mobile density follow-up: Detail clears the prior system/actor inspection when
selected, so Compact drill-down does not survive as the Detail overview. Phone
widths of 320–440 logical pixels display two Detail cards, with a 92-pixel orbit
band, smaller labels and 28-pixel task rows. Tabs and explicit buttons are 28
pixels high; search/toolbar controls are 32. These are visual layout sizes, not a
claim of a minimum accessible touch target. Density uses viewport width because
Android PRoot also reports Linux. No provider observation or action policy changes.
