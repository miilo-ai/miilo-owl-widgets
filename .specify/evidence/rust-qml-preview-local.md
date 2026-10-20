# Local Rust/QML preview qualification

Date: 2026-10-19. Environment: macOS 15.6.1 arm64, Qt 6.11.2 software/offscreen,
Node 23.10.0, Rust 1.96.1. Development branch: fix/qml-preview-runner.

## Completed checks

- Cargo formatting, six Rust unit tests and Clippy with warnings denied passed.
- Preview toolchain guard and its rejection regression passed.
- Actual QML capture matrix: 9 tests passed, zero failed/skipped.
- Matrix includes full/light-reduced/off motion, Quiet and expanded Quiet,
  inspection recording, invalid dimensions, missing runner and invalid QML.
- Inspection produced 25 deterministic frames for two seconds and verified
  task selection, file changes and verification output.
- Architecture/module-size, retained-render invalidation and no-Python guards passed.
- git diff --check passed.

## Fixture corrections

OrbitFixture now receives explicit initial properties rather than C++ context
properties. Its synthetic widget declares active=true, matching the renderer's
viewport scheduling contract. The former fixture omitted that property and
failed its original animated-frame comparison on this same host.

The capture tests sample a blink timestamp for Reduced motion rather than assuming
arbitrary timestamps differ. Off motion explicitly requires identical images.
Control text-height assertions read the actual numeric content height instead of
converting a missing QML property to zero.

## Limits and next steps

No deployment, remote command, installation or capture was performed on Omarchy
or Android. No production renderer, helper behavior or source pin changed.
The new CI workflow has been authored locally but has not run on GitHub.

Before release: reconcile current main, run the new workflow, and qualify native
Qt runner availability and behavior on the target machines after authorization.
Application versions remain unchanged because this change is development tooling.

## Subsequent Android smoke check

The existing isolated Omalauncher app on the local ARM64 Android emulator was
checked via ADB. Compact robots, selected-system task list, expanded task, both
Back transitions and Compact-to-Detail on the first tap rendered successfully.
The Detail view showed two card columns. The expanded screen was closed afterward.
No helper, widget, credential, configuration file or APK was deployed or replaced.
No explanation or provider action was invoked.

Installed widget source matched the local unchanged source after normalizing only
its pre-existing preview helper command prefix. The new desktop preview harness
was not run on Android: qmltestrunner is absent there. This is a mobile regression
smoke check, not Android qualification of the Rust/QML capture tool. No Omarchy
machine was accessed during this follow-up.

## Omarchy offscreen qualification

Date: 2026-10-19. Authorized tests ran in isolated temporary source snapshots on
Linux x86_64 with Qt 6.12.0, Rust/Cargo 1.99.1 and Node 26.12.0. Existing development
checkouts, installed plugin files and active configuration were not changed.

- Native plugin preview matrix: 10 passed, zero failed/skipped.
- Widgets preview matrix: 9 passed, zero failed/skipped.
- Each preview crate: six Rust unit tests, formatting and Clippy passed.
- Full plugin make check: 47 portable/lifecycle/uninstall fixture tests passed,
  plus package/provenance checks and shell/authentication checks.
- Widgets architecture and no-Python guards passed.
- Persistent synthetic Orbit and onboarding captures were retrieved for review.
- Exactly one live Quickshell process remained running with its original start time.

An initial manual qmltestrunner -help probe without a display platform aborted
because Qt could not connect to a display or initialize xcb. Subsequent probes
and every capture used offscreen rendering; no preview suite process crashed.
The runner already defaults to offscreen. 
macOS archive metadata files were removed from the disposable snapshots before
package checks. No runtime source or installed component change was required.
This qualifies the offscreen development runner on the tested Linux host; it is
not an installed-plugin lifecycle or physical-phone acceptance claim.

## Final implementation review

The reviewed source passed both local capture matrices and repeated Linux tests:
10 plugin preview cases, 9 widgets preview cases, 47 plugin regression cases,
and seven Rust unit tests per preview crate, formatting and Clippy. Module-size
and explicit-import guards passed. Changed files contained no private-machine
references or diary-style markers. Preview output validation now rejects stale
numbered frames from a longer recording before publishing a shorter recording.
The previous frames remain untouched. No production installation was changed.
