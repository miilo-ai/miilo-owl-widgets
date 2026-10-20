# Development toolchain

Application/helper behavior remains Rust. Presentation remains OWL/QML and its
shared JavaScript renderer. No Python source, installation, CI, check, test or
preview driver is required in this repository.

Portable development tools use Node.js 22+ built-ins (no npm install). The native
preview coordinator is a standalone Rust development crate. It invokes the installed
Qt 6.8+ qmltestrunner; QML tests exercise the actual renderer and capture PNGs.
There is no authored C/C++, CMake project or Rust-to-Qt binding build.

## Preview capture

Install Qt development tools with your normal package manager. On Arch/Omarchy,
use qt6-base and qt6-declarative. On macOS, use Homebrew qtdeclarative
(which includes qtbase as a dependency). Install stable Rust and Cargo as well.
Put qmltestrunner on PATH, or set MIILO_QMLTESTRUNNER to its executable path.
The Rust crate uses its committed Cargo.lock and builds under tools/qt-preview/target.
Fixture inputs and captures use temporary directories and are cleaned after the run.
Only validated captures from a successful run are copied to the requested output.
Qt remains an upstream dependency; this removes Miilo-maintained C++ tooling,
not Qt itself. The system Qt packages do not ship with Miilo.

Run:

```sh
node tools/render-orbit.cjs /tmp/miilo-orbit-preview
MIILO_PREVIEW_WIDTH=720 MIILO_PREVIEW_HEIGHT=680 MIILO_PREVIEW_RECORD=1 \
  node tools/render-orbit.cjs /tmp/miilo-orbit-recording
```

The recorder writes PNGs and 241 deterministic frames. Encode them with ffmpeg
if a video is wanted. MIILO_PREVIEW_THEME=light selects the synthetic light QA
palette; the default dark fixture uses Omarchy Tokyo Night colors. Full, reduced
and off motion are selected by MIILO_PREVIEW_MOTION.

This is an isolated Qt fixture with synthetic tasks and a terminal stub. It
checks real drawing and row selection, not Omarchy lifecycle, live providers,
Android rendering, accessibility acceptance or measured frame rate.

## Regression protection

`node tools/check-no-python.cjs` rejects language source and interpreter/binding
dependencies in active tooling. The constitution requires this boundary. Dated
historical evidence is retained honestly; its older commands are superseded by
this guide. Runtime installation does not start agents, enroll sources, or
change provider configuration without explicit consent.

## Short theme previews

Use the native Qt fixture with synthetic inventory. Example:

```sh
MIILO_PREVIEW_WIDTH=720 MIILO_PREVIEW_HEIGHT=450 \
  MIILO_PREVIEW_RECORD=1 MIILO_PREVIEW_SECONDS=6 \
  node tools/render-orbit.cjs /tmp/miilo-orbit-short
```

For the compact Quiet fleet, set MIILO_PREVIEW_VIEW=quiet, height=230, and
MIILO_PREVIEW_PALETTE=tests/themes/osaka-jade.toml. For the actual task-click
sequence, set MIILO_PREVIEW_VIEW=inspect and height=760. These are fixture
controls, not application preferences. The duration is bounded to 2–20 seconds;
the legacy recording default remains 20 seconds. Frames are emitted at 12 fps.

The fixture uses the native host's existing 4.5:1 muted-text contrast floor.
Theme hues come from the original theme palette; lifting secondary text matches
the installed host policy. Quiet captures the compact robot fleet, not the
expanded grouped workflow view. Inspect clicks a real task row after two seconds
and asserts that evidence, the file diff and verification output render. It
never executes Explain, approvals, enrollment or a real provider command.

These captures prove renderer behavior only. Actual Omarchy shell lifecycle,
font/scaling and physical Android interaction remain separate acceptance checks.

The shared Qt host defaults to a 30 FPS animation paint budget. It advances using
elapsed time, so orbit speed is independent of skipped paint ticks. Hosts can set
`animationFramesPerSecond` between 1 and 60. Source refresh and input invalidation
remain immediate. The Android launcher has its own frame clock.

## Preview regression gates

Run `make check-preview-toolchain` for the language boundary, Cargo formatting,
unit tests and Clippy. Run `make check-preview` for actual offscreen captures,
interaction assertions and a matrix of motion/view modes. The independent QML
preview workflow runs these checks without SDK access or provider credentials.

The tool preserves the orbit.png, orbit-next-frame.png, orbit-detail.png and
frames/NNNN.png output names. Native onboarding additionally preserves crew-dark.png,
crew-expanded.png and crew-light-narrow.png. Recording includes both endpoint frames:
two seconds produces 25 PNGs; the 20-second default produces 241.

The coordinator validates dimensions (100–4096), recording duration (2–20 seconds),
mode and motion settings. Missing runner, QML compilation errors, QML warnings,
failed interactions, missing/invalid captures and a 90-second runner timeout fail
with a nonzero exit. Qt platform diagnostics such as font-alias timing remain visible;
they are not QML-engine warnings. These tests use no live observation data.

When shortening a recording, use a new output directory. The coordinator rejects
leftover numbered frames from a longer recording before publishing captures; it
does not silently combine recordings or delete the earlier frames.
