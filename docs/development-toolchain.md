# Development toolchain

Application/helper behavior remains Rust. Presentation remains OWL/QML and its
shared JavaScript renderer. No Python source, installation, CI, check, test or
preview driver is required in this repository.

Portable development tools use Node.js 22+ built-ins (no npm install). The native
preview driver uses C++17, CMake 3.21+ and Qt 6.8+ Core/Gui/Qml/Quick/Test, matching
the actual renderer rather than recreating it in a web screenshot.

## Preview capture

Install Qt development tools with your normal package manager. On Arch/Omarchy,
use qt6-base, qt6-declarative and cmake. On macOS, use Homebrew qtbase,
qtdeclarative and cmake. Standard Qt SDKs also work through CMAKE_PREFIX_PATH.
The capture wrapper builds in a temporary directory with two build jobs and
removes that directory afterward. The system Qt packages do not ship with Miilo.

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
