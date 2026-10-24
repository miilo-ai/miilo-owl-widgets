# Omalauncher branding and About

Local branch: feat/omalauncher-brand-and-about. Package version: 1.7.1.

The three generated OWL6 widgets use the custom wordmark inside their screen and
settings views. The native launcher chrome is unchanged. Stock canvas paths are
projected from the project SVGs, with theme foreground ink, sampled curves,
transparent visor cutouts and no image fetches or animation clock dependencies.
Full observation robots and their existing responsive layouts are preserved.

Settings → About is isolated from task content and configuration terminals. It
remains available without the helper and displays the generated widget version,
privacy/usage disclosures and explicit public website/sponsor actions. It honors
the saved action-side preference and returns to the settings menu with Back.
The displayed version does not claim to identify an independently installed helper.

## Completed local checks

- Full `make test`: helper/Rust integration, preview toolchain, installer, release,
  architecture and source guards pass. HTTP/chat fixtures used local loopback only.
- `make check-desktop`: 36 Linux evaluation scenarios, robot/orbit/portrait/compact
  regressions, session visibility, 18 settings scenarios and helper onboarding pass.
- Additional 36 brand/About scenarios cover all three widgets, 320/393/600 widths,
  both themes and helper-present/missing states. Rendering does not install helpers,
  invoke models, change credentials or open URLs. Public links act only on selection.
- Generated canvas matches the canonical SVG source. Existing robot-count tests
  distinguish the brand canvas from observation robot canvases.
- macOS Qt 6.11.2 offscreen rendering of the About view passes with its real OWL
  renderer and a synthetic empty inventory. This is not Android acceptance.

## Before publication or marketplace PRs

The October 24 review ran `make test check-desktop` successfully and passed
264 official OWL compiler/evaluator tile and screen scenarios. Adaptive Compact
layout checks cover 60 population/viewport combinations, including 1–20 systems,
column limits and width/height bounds. Small fleets grow while dense fleets stay
compact. Generated widget copies and brand assets match their source.

The Android 14 ARM64 Omalauncher preview app loaded the exact generated fleet
widget, verified by SHA-256. Its custom wordmark, About disclosures, packaged
version 1.7.1, Back and Done were checked in the installed launcher. Existing helper,
configuration and credentials were preserved; no enrollment, model request or
provider action was performed. The configured inventory was empty, so this device
run does not establish live multi-system geometry or current provider health.
Light/dark and many-system geometry are covered by evaluator/Qt fixtures, not
claimed as physical-phone acceptance.

The actual release archive, clean-install download, physical phone, larger font
settings and full live pairing journey remain release qualification gates.
Retain existing configuration; no silent enrollment or provider actions are allowed.

After source PR approval, build and qualify the actual release archive, then
prepare the widget marketplace update with the new literal archive checksum and a
store version above 1.6.4. Native plugin changes and its marketplace submission are
separate. Do not claim an unpublished helper or pending listing is available.
