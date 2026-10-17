# ORBIT-001 local candidate evidence — 2026-10-17

Candidate implementation, not release or native Omarchy acceptance.

- Shared modular OWL: orbit-geometry.inc and orbit.inc. Quiet remains default;
  expanded view choice persists. Existing widget tile and action paths retained.
- Official Kotlin checker accepted all three generated widgets; 264 existing
  tile/screen evaluations passed. New Orbit-specific scenarios run in the Linux
  JavaScript evaluator; those do not yet extend the Kotlin fixture matrix.
- 36 Linux evaluator theme/size/detail/confirmation scenarios passed.
- Robot controlled-clock and new Orbit checks passed: themes, finite geometry,
  full/reduced/off, stale stop, six-cluster bounds, page clamping, stationary
  work selection, fallback preference and no frame I/O.
- Rust helper and xtask test suites passed. Rust formatting and architecture checks
  passed. The xtask lock was stale against the helper's existing SDK pin; updated
  it to that already-selected pin and its compression transport dependencies.
- Actual Qt 6.12.0 QML renderer, software/offscreen on macOS: synthetic fixture
  rendered and controlled-frame screenshots were inspected; native row click opened
  current details. No evaluator or QML warnings in that fixture. Existing palette
  override and missing-font diagnostics are host warnings, not performance evidence.

Reproduce actual layout with tools/render-orbit.py (developer-only PySide6).
The harness copies current renderer files into an isolated folder and stubs only
unused terminal plumbing. No source command executes. Images are local artifacts,
not fabricated live-provider or native Omarchy screenshots.

Native Omarchy install/shell lifecycle, platform theme/fonts, keyboard focus,
fractional scaling, native frame/CPU/RSS measurements and Android remain pending.
Canvas glyph picking is deferred; accessible stationary work rows are functional.
No event flashes or dependency edges are invented when source data lacks identity.
Do not mark all ORBIT-001 tasks complete or claim a measured percentage finished.

## Original robot identity correction

A generated portrait program now embeds the exact shared Bot, appearance controls
and palette from authored OWL. No second character geometry is maintained.
Official Kotlin checker accepted the standalone program. Drawing parity passed
18 state/theme/motion combinations at 3 controlled frames (54 comparisons), with
no external sources or I/O. Existing renderer/Orbit/motion and xtask tests passed.
