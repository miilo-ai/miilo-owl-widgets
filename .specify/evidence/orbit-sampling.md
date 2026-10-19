# Orbit sampling budget

The projected ellipse now uses 32 segments per half rather than 64. Unit-circle
samples remain immutable. Shared endpoints, retained samples, task orb positions,
plane clocks, colors, glow strokes, nucleus clearance and robot expressions are
unchanged. The largest built-in radius is 122 × 0.85 dp; its chord error bound is
radius × (1 − cos(pi / 64)), below 0.125 dp. No frame-rate limit is introduced.

## Validation

- 287 generated canvas comparisons across Compact/Detail, tile/expanded,
  active/error/resting/blocked states and seven clock phases. Retained values
  match within 1e-9; removed samples differ from the replacement chords by at
  most 0.1248 dp. All task orb and robot operations remain unchanged.
- Shared orbit, motion, portrait, compact, session visibility, mobile settings and
  helper-onboarding checks pass.
- On the same Linux machine, a synthetic four-active-robot native Qt fixture
  measured evaluator cost at 23.47 ms/frame with 64 segments per half, then
  13.50 ms/frame with 32. Median offscreen frame interval changed from 26.52 ms
  to 19.59 ms; p95 from 36.79 ms to 28.85 ms. These short runs measure the fixture,
  not a guaranteed real-device frame rate or whole-shell CPU improvement.

Run the explicit comparison using a reviewed baseline widget source:

```sh
node tools/compare-rendering.cjs desktop/runtime/Owl.js BASELINE_WIDGET --subpixel-rails
```

Without the flag, comparison remains strict, including all path points.

## Qt paint cadence

Geometry alone left a visible single-system live shell at 74.6–74.9% of one core,
compared with approximately 87% before. The Qt runtime therefore defaults to a
30 FPS animation paint budget. Its FrameAnimation still supplies elapsed time;
skipped paint ticks do not slow orbit travel, accumulate catch-up frames or delay
source invalidation and input handling. Hosts may choose a budget from 1–60 FPS.

The native clock fixture verifies both the paint ceiling and elapsed-time speed,
alongside retained layout, offscreen/static suppression, scrolling and resize.
Android uses its own renderer; this Qt cadence setting does not alter Android's
host frame clock. Shared orbit geometry still applies to both distributions.

## Installed native result

With the same real single-system dashboard visible, two settled ten-second
samples after the geometry and paint-budget changes measured shell CPU at 67.8%
and 67.3% of one core, versus approximately 87% before. Earlier 48.5%/68.6%
samples overlapped startup and are not steady-state evidence. Whole-shell RSS
was approximately 523–529 MiB; this is not evidence of a memory leak or its cure.
Only one live Omarchy shell was present. Native package/lint, typography,
configuration, plain-text positive-control canary and canvas-clock tests passed.
The optimized matching helper and original collector were backed up before
replacement. All thirteen explicitly selected sessions remained current.

Rendering remains a significant CPU cost. These changes improve it; they do not
establish a low-power budget for large fleets or certify physical Android behavior.
The separate observer wakeup correction reduced its idle CPU below 1% of a core.

## Android emulator acceptance

Validated the generated 1.6.3 widget in the Android 14 ARM64 preview launcher.
A private on-device backup preserved its original widget, preferences and helper
configuration/credentials. The test used four explicitly labeled read-only sample
systems; no agent work, real provider actions or model requests were started.

- Four robots rendered side by side in the home widget; expanded Compact used
  the existing responsive 3+1 arrangement and Detail used its phone card layout.
- Opened the review system and task, inspected evidence and colored file changes,
  returned through both in-widget Back steps, and verified Needs you filtering.
- Opened isolated Settings and Appearance; Full/Reduced/Off, personality, pace
  and per-robot color controls remained available. No saved choice was changed.
- A silent eight-second Detail capture yielded 32 unique cropped orbit frames at
  four samples per second, with no adjacent duplicate samples. This demonstrates
  motion across the sampled interval, not 60 FPS or physical-phone performance.
- Settings Done and Android Back returned to the fleet and launcher respectively.
- Restored the original widget/preferences byte for byte and verified helper
  configuration/credentials against the private backup before restarting.

The Qt-specific paint budget is covered by native Qt tests, not this Android run.
