# Animation rendering performance

## Scope

Correct frozen deferred canvas clocks and reduce per-frame UI work while keeping
original robots, orbit geometry, glow, state indicators, task evidence and actions.
The shared renderer remains a presentation adapter. Observation/domain contracts
and agent/provider permissions do not change.

## Required behavior

- Component-local time expressions advance without a source refresh or layout rebuild.
- State/theme/size changes invalidate cached expressions and display lists.
- Static canvases reuse their output rather than repainting on every clock tick.
- Clipped/offscreen and inactive canvases skip drawing and resume when visible.
- Neighbor glances select their direction in the drawing phase, not the task layout.
- Immutable orbit samples are cached. Per-frame projection keeps the existing
  65 samples per half-ring, matched endpoints, motion paths, glow and clock speed.
- Full/Reduced/Off, pace, colors and saved connections remain intact.
- Provider strings retain explicit plain-text rendering protections.

## Evidence gates

- [x] Retained-clock, nested dependency, state/resize invalidation and static-cache tests.
- [x] Native Qt clock, stable-layout, static-paint and scroll-culling regression tests.
- [x] Existing orbit, robot, compact density, portrait and settings regressions.
- [x] Native isolated evaluator timing separates JavaScript geometry from painting.
- [x] Final installed-source hash verification and actual desktop inspection.
- [x] Final same-view whole-shell CPU comparison, with visible/hidden control.
- [x] Companion shared renderer native acceptance.

A short CPU sample is not a leak test. Evaluator timing is not GPU frame timing.
Physical Android performance remains a separate acceptance step. Do not claim a
fixed frame rate without a frame-pacing measurement.
