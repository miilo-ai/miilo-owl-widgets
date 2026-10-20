# Rust-coordinated native QML previews

## Scope

Replace the custom C++ capture executable with a standalone Rust development crate
and the installed Qt qmltestrunner. Keep production components, helper behavior,
renderer source pins, synthetic scenario inputs and existing preview commands.
This is development tooling, with no application version bump.

## Responsibilities

- Node wrappers prepare the existing synthetic fixtures and invoke Cargo.
- Rust validates options, writes JSON-serialized QML configuration, owns temporary
  capture directories, launches one bounded Qt test process and validates artifacts.
- QML tests load the actual components, simulate input and assert visible evidence,
  compact control geometry, expansion, motion and saved image differences.
- Upstream Qt owns native rendering. No Rust Qt bindings or custom C++ compilation.

## Acceptance

- Existing Orbit filenames and 12 fps deterministic recording sequences are retained.
- Onboarding dark/expanded/narrow captures remain pixel-identical on the same Qt host.
- Full motion differs across selected timestamps. Reduced motion tests the retained
  blink window. Off motion produces equal frames.
- Inspect clicks a task and verifies file changes, filename and verification output.
- Quiet and expanded Quiet retain their real navigation paths and theme inputs.
- Missing runner, malformed input, invalid bounds, QML errors/warnings, failed checks,
  missing PNGs and timeout return failure without publishing partial captures.
- Source checks prohibit preview C/C++, CMake and bindings that generate C++.
- CI executes the native fixture matrix independently of private SDK access.

## Remaining qualification

Current development evidence is macOS offscreen Qt only. Before release, reconcile
with current main in both repositories, run the independent preview workflow, then
qualify the installed Qt version and paths on Omarchy with owner authorization.
Live shell lifecycle, Android rendering, fonts/scaling, performance and keyboard
acceptance remain separate gates. No remote deployment is part of this change.
