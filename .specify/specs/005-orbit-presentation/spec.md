# ORBIT-001 — Quiet / Orbit presentation

Status: initial candidate implemented; native Omarchy qualification pending.
Decision: 2026-10-17. See ../../evidence/orbit-local.md for measured scope.
Native OWL/Omarchy first; Android parity next; web is a later adapter.

## Visual contract

Quiet preserves recognizable robots, compact layouts and modest state animation.
Orbit expands the same observations into stable system clusters with shallow
elliptical work paths, original robot anchors and restrained electrical filaments.
No globe, enclosing sphere, starfield, central authority hub or all-to-all mesh.
Labels and controls stay still. System neighborhoods retain stable positions.
Selecting work freezes that cluster's moving targets and opens existing details.
Do not encode progress as orbit angle, speed, particle count or a percentage.

View choice (quiet/orbit) and motion (full/reduced/off) are independent persisted
preferences. Existing installations retain Quiet. Orbit never changes enrollment,
provider configuration, credentials, discovery, action authority or polling cadence.

## Ownership and proposed modules

- SDK retains observations/actions; no new graphics engine or scene schema there.
- Rust helper supplies existing bounded normalized inventory and stable scoped IDs.
  Add an additive semantic scene projection only if current data is insufficient;
  do not create a required new CLI command just to animate existing observations.
- UI projection selects clusters, visible work and grouped remainder; pure geometry
  maps these to coordinates. It must run with fixtures and a supplied clock.
- OWL owns drawing, time, palette, selection and motion preferences. Suggested split:
  orbit-state.inc, orbit-layout.inc, orbit-draw.inc and orbit-controls.inc.
- Plugin owns native hosting/lifecycle and consumes the reviewed shared output.
- No per-frame subprocess, HTTP fetch, model call, generated OWL code or new listener.
  No lazy imports; new authored modules under 200 lines. No copied SecMesh source.

## Geometry and interaction

1. Key every system/work item by connection plus object ID. Derive deterministic
   seed/slot from that identity; never derive position from status or random() per frame.
2. Place clusters in a responsive stable grid; filtering preserves retained IDs.
   Reserve label/header space and clip decorative strokes inside the scene bounds.
3. Ellipse at phase a: x=rx*cos(a), y=ry*sin(a), z=rz*sin(a+tilt).
   Rotate with a fixed matrix; project with s=d/(d+z), keeping d+z>=d/2.
   Projected position is anchor+(x*s,y*s); clamp apparent size/opacity.
4. Draw far objects before near objects; labels remain a separate fixed layer.
   Behind-robot trails are drawn before the robot; front trails after it.
5. Stable ID sets initial phase; elapsed seconds sets motion. Start at 12-24 s per
   orbit, tilt 12-24 degrees. These are tuning proposals, not timing measurements.
6. Focus pins the selected work to a stationary inspection position; use a 180-260 ms
   eased transition, no overshoot. Reduced/off switches immediately. Preserve focus
   and selected ID across updates; removal produces an explicit unavailable state.
7. Initial budget: 6 clusters/page, 8 work glyphs/cluster, <=48 visible work glyphs.
   Group the remainder with a selectable count. Search includes undisplayed items.
   Narrow view uses one focused cluster plus searchable system selection.
8. Native Button/accessible list provides equivalent keyboard actions. Canvas hit
   tests use the same projected geometry as drawing, >=44 dp effective targets;
   expand spacing or use the list where targets overlap. Decorative paths take no input.

## Electrical animation mechanics

Use original procedural polylines; do not require shaders, bloom or particle modules.
Separate ambient ornament from observed events in names, policy and tests.

- Ambient filament: a short open arc within an active cluster's orbital ribbon.
  It conveys recent observed activity only, never packet flow or agent communication.
- Relationship pulse: available only on a published relationship. Explain its meaning;
  do not invent direction, throughput, dependency or a cross-provider edge.
- Event flash: one small travelling highlight for a new accepted observation revision,
  deduplicated by source epoch+event/revision. Initial load/replay does not celebrate.
  If the adapter lacks event identity, omit this effect rather than infer an event.
- Receipt, execution and completion remain distinct; an accepted approval is not success.
- Idle/resting has no electrical effect. Stale/unreachable cancels effects immediately.

For a path P(u), sample 16 segments and compute cumulative distance once per geometry
change. Interpolate the lookup table for constant-speed pulse travel. For a filament,
use the local perpendicular N(u): Q(u)=P(u)+N(u)*A*sin(pi*u)*noise(seed,k,bucket).
This envelope keeps both endpoints on the path. Sample 12 segments; blend seeded
noise between 100 ms buckets instead of choosing fresh offsets every frame.
Start A at 1.5-3 dp. No large forks. Optional one short branch in Full only after profiling.
Draw identical points in three passes: 4 dp at alpha .06, 2 dp at .16, .9 dp at .85.
This simulates a halo without blur. Fade in/out over 120/220 ms; lifetime 450-700 ms.
Initial global cap: 3 simultaneous filaments, one/cluster, >=4 s ambient cooldown;
rotate eligibility so many systems do not flash together. Never flash the whole scene.
These numbers are initial design budgets, not performance or accessibility certification.

## Theme and motion policy

Use host accent, foreground, muted, background and semantic error tokens. Respect
assigned robot colors. Electrical trails use the selected accent, not robot colors.
A lightened core is mix(accent,foreground,.2); tune on light themes rather than force white.
No hard-coded cyan, pure-black backing disk, rainbow scheme or yellow attention default.
Normal text targets 4.5:1 contrast; necessary state/focus graphics target 3:1.
Decorative low-alpha halos never carry status alone. Retain text/shape alternatives.
Quiet: existing robots and status transitions, no electrical paths or orbital movement.
Orbit/Full: bounded orbit, robot personality, filaments and observed event effects.
Orbit/Reduced: static arrangement, no travel/jitter/tilt, immediate state updates;
retain only static state shapes and optional existing gentle blink preference.
Off: every decorative channel and robot animation static; no continuous repaint.
Honor system reduced motion where exposed; otherwise provide explicit controls and
record that automatic system integration is unavailable. Do not overwrite saved choices.
Hidden/inactive/minimized surfaces stop decorative scheduling; resume from current
state without replay bursts. Lifecycle forwarding requires real host qualification.

## Rendering feasibility gate

Current OWL supports Canvas, time, sin/cos, paths, alpha/mix and coordinate taps.
A temporary 12-point projection compiled/evaluated with the pinned evaluator;
this does not prove official Kotlin compatibility, native pixels or frame rate.
Qt 6 documentation says Canvas.FramebufferObject is ignored and warns that large
frequently updated canvases can incur texture-upload costs. The vendored comment
claiming a guaranteed GPU Canvas path is not evidence; do not repeat it in claims.
Keep surfaces/operation counts bounded and static geometry cached where practical.
If native budget fails, reduce ornament first; consider a separately qualified QML
scene-graph adapter later without forking semantics or silently changing the runtime.

## Validation / release gates

- [ ] Compile with official OWL checker and evaluate actual generated widget.
- [ ] Controlled-clock frames: stable IDs, finite coordinates, depth order, frozen
      selection, bounded geometry, exact endpoints, seeded noise and pulse speed.
- [ ] Full/reduced/off, hidden/resume, stale sources, event replay/epoch reset,
      duplicate events, theme changes and preference migration have regressions.
- [ ] No I/O during frame generation; action checks still use fresh authority.
- [ ] Native keyboard/pointer hit targets match drawing; details/evidence scroll.
- [ ] Native screenshots: empty/one/many, long labels, 360/720/1280 dp, fractional
      scaling, light/dark, multiple accents, custom robot colors and increased text.
- [ ] Record 30 s native frame timing + CPU/RSS with 1/6/24/50 systems (pagination),
      controls responsive. Target <=16.7 ms p95 frame interval at 60 Hz demo setting;
      report misses honestly and offer a measured lower-effects mode.
- [ ] Hidden 30 s measurement verifies frame callbacks cease; provider lifecycle
      remains governed separately. No extra shell process or privileged operation.
- [ ] Android native compilation/rendering/interaction/thermal check separately;
      Omarchy evidence does not qualify phone parity. Record versions and clips.

## Research references

- https://github.com/SimonSchubert/omarchy-widgets/pull/141 — OWL globe/swarm example;
  open submission, not proof of acceptance or performance on Miilo's runtime.
- https://doc.qt.io/qt-6/qml-qtquick-canvas.html — Qt 6 Canvas limits/render targets.
- https://doc.qt.io/qt-6/qtquick-performance.html — native frame profiling guidance.
- https://dev.to/bimotalk/building-procedural-motion-for-the-web-geometry-particles-and-deterministic-canvas-animation-1d66
  — procedural geometry, seeded time and arc-length flow; web practice, not a Qt benchmark.
- https://github.com/BoogerCheeseOnRye/SecMesh — visual reference only; no project
  license found at aff7dc6. Do not copy source/artwork absent an applicable grant.

## Visual correction: soft travel rail

Use an unwarped tilted ellipse for the rail, work positions and travelling light.
The initial displacement-noise proposal above is superseded for the main orbit: it
made the highlighted segment look kinked. Use 128 closed rail segments, 32 short
highlight segments and round joins. Electricity comes from the layered light/fade;
do not displace its core away from the rail. Preserve the separate depth cues.
