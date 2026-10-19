# Rendering performance validation

## Implemented

- Local expression caches track data and clock dependencies separately. A retained
  deferred Canvas refreshes its clock without rebuilding the complete widget tree.
- Canvas display lists cache static output and invalidate on data, theme or size
  changes. Offscreen/clipped/inactive canvases skip drawing and resume on visibility.
- Neighbor glances calculate their direction during drawing rather than layout.
- Immutable orbit samples and per-plane projection axes are reused. Existing
  65-point half-rings, shared endpoints, paths, colors and glow are retained.
- Shared and plugin renderer changes include reproducible patches and provenance.

## Functional validation

Native Omarchy host: package/lint checks, all 47 plugin source tests, plain-text
network canary with positive control, clock/static-layout/paint-culling tests,
15 typography cases, toolbar/settings/groups and configuration checks passed.
The companion shared renderer passed its native clock, security and typography
checks plus orbit, compact, portrait and 18 mobile-settings scenarios.

Portable evaluator tests passed for retained clocks, nested time dependencies,
static-list caching, state/resize invalidation and absence of provider actions.
287 canvas comparisons matched the pre-optimization renderer across modes,
robot states and seven clock phases, using a 1e-9 numeric tolerance. Existing
54 shared portrait comparisons also passed. Generated widget/portrait assets
were assembled through the Rust xtask. All 63 helper Rust tests and five xtask
tests passed. The helper chat integration required loopback-listener permission
outside the local filesystem sandbox.

## Native A/B observations

The same isolated offscreen Qt scene used four active robots, a 580 by 620 logical
viewport, synthetic tasks and a three-second frame interval sample. The baseline
includes the clock-freeze correction so this compares moving scenes.

| Measurement | Baseline | Optimized |
| --- | ---: | ---: |
| Observed frames in sample | 44 | 102 |
| Median frame interval | 69.37 ms | 29.25 ms |
| 95th-percentile frame interval | 84.26 ms | 39.54 ms |
| Intervals above 33.4 ms | 44 | 17 |
| Mean evaluator time over 60 frames | 47.63 ms | 25.42 ms |

These are short measurements on one Intel integrated-graphics laptop, using the
same native test harness. Offscreen software rendering is not a physical-device
GPU benchmark. Do not extrapolate a guaranteed frame rate to other machines.

The actual desktop was updated through the maintained backed-up workflow and
visually inspected. A visible/hidden control showed whole-shell CPU falling to
approximately zero with Miilo hidden. Visible whole-shell ten-second samples
remained around one core: 113% before and 122–123% afterward. Thus smoothness
improved, but reduced total CPU consumption has **not** been demonstrated. Memory
remained around 539–552 MiB with approximately 11.3 GiB available. Short samples
cannot establish or exclude memory leaks.

## Limits

Further native profiling is needed for CPU headroom and a consistent 60 FPS goal.
No frame-rate cap was added. Physical Android frame pacing and a GPU trace are
not qualified by these tests. Existing live helper/collector binaries were kept
unchanged; this validates UI rendering, not a newly installed helper release.
