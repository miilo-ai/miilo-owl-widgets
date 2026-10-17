# Atom Orbit presentation checkpoint

The two rails now use one deterministic projection and one clock for both rails,
their soft neon strokes, and task glyphs. There is no independent particle
simulation or lane switching. Stable provider offsets survive system filtering.
The shared original robot is anchored at its head center only in Orbit; other
portraits retain their existing positioning and expressions.

## Rendering contract

- Field height 192 dp; radii 122 / 98 dp, projected rise 52–64 dp.
- Plane angles -0.44 / +0.44 radians with a shared +/-0.17-radian sweep.
- Golden-angle provider offsets; 18-second rise and 30-second orientation periods.
- Front/rear split at depth zero, equal stroke width, rear intensity 0.84.
- Layered low-alpha strokes approximate the browser's soft neon without relying
  on a platform blur. Glow and core use exactly the same polyline.
- No masks or rails through the nucleus. The 112 dp original Bot is centered and
  stops body bobbing in Orbit; blinking, reactions and sleep letters remain.
- Resting systems without active tasks have stationary muted rings without glow.
- Full/Reduced/Off, stale/offline cancellation and Calm electrical suppression
  remain honored. Zaps are decorative, not action or provider receipts.
- Six systems per page, at most eight task glyphs per system. Selection and
  existing evidence/action flows are unchanged. No I/O in frame rendering.

## Portable evidence

`tests/orbit-projection.json` contains golden frames exported from the approved
browser motion study, not a second native scene implementation. Native generated
OWL matches sampled rail positions and task centers/scales for 18 time/provider
combinations. Glow follows the core exactly; rails remain >=52 dp from center.
Tests also exercise theme/motion/stale states, stable filtered offsets, bounded
pagination, state-label alignment, task selection and absence of frame commands.

- `make check-desktop`: 36 Linux evaluator scenarios, Orbit geometry/interaction,
  robot motion and exact shared portrait parity (54 comparisons).
- Official OWL compiler accepts the generated fleet program.
- Real Qt 6.12 controlled-time fixture: desktop and 390 dp light layout rendered;
  work-row click opens the existing details, no runtime errors or commands.
- A 20-second controlled-time Qt recording includes two active and one dormant
  system. This is a synthetic fixture, not a frame-rate benchmark or live feed.
- Architecture and whitespace checks pass; authored geometry remains below 200 lines.

## Native handoff

The installed Omarchy shell still must qualify live snapshots, lifecycle/focus,
keyboard, fractional scaling, theme changes and frame cost with many systems.
Android/Omalauncher must separately qualify rendering, touch and motion settings.
Do not call isolated Qt captures installed-shell or phone acceptance.

A useful community technique is deriving every animated part from the same time
and parameters, so a frame can be reproduced and compared:
[procedural web motion](https://dev.to/bimotalk/building-procedural-motion-for-the-web-geometry-particles-and-deterministic-canvas-animation-1d66).
The native implementation keeps that principle and the existing OWL renderer.

Final official validation: `cargo xtask check-owl` passed all 264 tile/screen
compiler/evaluator cases across the three generated widgets. Existing approval,
confirmation, evidence, search and responsive collapsed-tile assertions passed.

## Attention consistency follow-up

Orbit now passes the shared robot's attention/error mark explicitly, aggregating
all system items before the eight-orb display cap. Needs-you work shows ?!, failed
work takes priority with !!, and stale/offline sources suppress actionable marks.
The shared robot also uses a supplied mark for its blocked/error expression.
Regression draw tests cover attention, failures outside the visible cap, cleared
work and stale sources. Existing motion, geometry, portrait parity and portable
interaction checks pass. Official syntax checks pass for all three widgets.

Controlled-time recording now uses the same sample task/state distribution as
the browser: one factory review, coding tasks implementing, one dormant system.
The toolbar order is Settings, Refresh, then the system selector. Plugin source
provenance includes the local presentation-consistency patch until a new source
revision is committed and pinned. Installed machine acceptance remains separate.
