# ORBIT-001 local candidate evidence — 2026-10-17

> Historical validation record. Results apply only to the revision and environment recorded below; they do not describe current release readiness.


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

## Smooth-rail correction

Removed nonlinear base-path warping and electrical position jitter. Rail, glyphs
and travelling light now share an exact tilted ellipse. A regression verifies the
closed seam and the ellipse equation for every rail/highlight sample. Official OWL
syntax, existing renderer/motion/portrait checks and actual Qt render/selection
passed; the updated screenshot was inspected. Native machine gates remain open.

## Readability and toolbar follow-up

Center Bot size increased from 62 to 93 dp (same original drawing). Counts use
singular/plural task labels. Filters and view selection now share one compact
row with Quiet/Orbit right-aligned and 44 dp hit targets. Actual Qt screenshots
inspected at 900 and 360 px; native row selection passed at both widths. View
switching/persistence, task labels, renderer/motion/portrait and official syntax
checks passed. Omarchy shell-specific gates are unchanged.

## Crossed paths and state icons

The secondary ellipse now rocks through +/-45 degrees over 20 seconds at normal
pace. Alternating task glyphs follow the two exact ellipses. The primary rail and
its travelling light retain identical geometry. Reduced/Off and stale sources
freeze the secondary rail; motion never issues provider commands.

Small accent-colored icons accompany state text: attention mark, implementation
chevrons, verification magnifier, failed cross, stale slash and waiting clock.
Labels remain visible and existing work-row selection opens the same detail view.

Validated generated OWL geometry/closure/bounds, state icons, static motion modes,
36 Linux evaluator scenarios, original portrait parity, official OWL syntax and
actual Qt renders at 900px and 360px, including native detail selection.
A deterministic 241-frame Qt recording covers a full 20-second cycle at 12fps.
This is synthetic inventory with software Qt rendering, not native performance
or provider-operation evidence. Installed Omarchy acceptance remains pending.

Reproduce with `MIILO_PREVIEW_RECORD=1 MIILO_PREVIEW_HEIGHT=550 python3
tools/render-orbit.py /tmp/orbit-capture` using the documented Qt environment.

## Antenna zap and state alignment

State labels now start beside their 14dp icons with a consistent 4dp gap.
An expressive, working robot may emit a decorative 180ms antenna-to-task arc
every nine source-time seconds, staggered per system. The selected destination
is an active task without attention; the path and endpoint follow that orb.
A cool secondary theme blend separates it from the rail. The renderer draws
three short strokes and two endpoint glows, with no provider calls or messages.
Calm, Reduced/Off, unavailable, degraded, sleeping and attention-only systems
do not emit arcs. Source pace scales presentation timing.

Generated geometry tests cover burst start/end, staggering, bounds, resting and
attention suppression, static modes, label alignment and no provider commands.
Desktop evaluator, robot parity, architecture and official syntax checks pass;
actual Qt captures verify desktop/narrow layout and native detail selection.
Recordings use synthetic inventory; installed Omarchy acceptance remains pending.
