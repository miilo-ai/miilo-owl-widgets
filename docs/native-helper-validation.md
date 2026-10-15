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
