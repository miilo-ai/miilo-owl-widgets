# Quiet and Orbit

The expanded Miilo screen now offers Quiet and Orbit. Quiet keeps the current
work view and compact robot tiles. Orbit groups observed work around a robot
anchor for each connected system. The choice persists separately from motion.
Choose Full/Reduced/Off under robot appearance in setup.

Orbit shows at most six systems per page and eight work glyphs per system.
Each stationary work row opens the existing details, evidence, explanations and
advertised actions. The +N control switches to that system's ordinary work list;
search and Needs you still apply. Canvas glyphs are currently decorative, not
separate click targets. No inferred cross-system links or event-completion bursts.

Electrical filaments are bounded decorative activity cues, never network packets
or proof that an agent is communicating. They stop for stale/unavailable systems.
Reduced/Off freezes the orbital geometry. Existing robot motion rules still apply.
There is no new CLI command, enrollment step, model call or backend dependency.

## Local qualification

Run make assemble, make check-desktop and make test. The official check-owl target
also needs the Kotlin compiler and OWL checker classpaths described in developer
setup. Controlled-time tests execute the generated OWL, not a duplicate renderer.

Optional actual Qt layout/Canvas render (developer-only dependency):

```sh
uv run --with PySide6-Essentials python tools/render-orbit.py /tmp/miilo-orbit-preview
```

It uses synthetic inventory, the current evaluator/layout/Canvas and an unused
terminal stub. It cannot run helper commands. Screenshots and native work-row
selection are checked. It does not start Quickshell, change config or qualify the
actual Omarchy host. Use a working graphics session/backend if offscreen is
unavailable. PySide6 is not an application/install dependency.

Native agent: pull the plugin's matching asset pin and perform the lifecycle,
keyboard, theme, scaling and performance gates in ORBIT-001. Test Android separately.
Remaining follow-ups: direct glyph picking, event-identity-driven one-shots,
smooth membership transitions and measured adaptive effect density. System motion
preference detection and complete zero-frame scheduling in Off are not yet qualified.
Qt 6 ignores Canvas.FramebufferObject; do not advertise guaranteed GPU Canvas speed.

Onboarding portraits must use desktop/RobotProgram.js, generated from the exact
shared Bot and palette source by make assemble. Do not redraw a simplified QML
robot. tools/test-portrait.cjs compares exact drawing operations to the fleet Bot.

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

For the current renderer and capture commands, see [the toolchain guide](development-toolchain.md).
The current atom projection supersedes the earlier independent ellipse motion described above.

Expressive Full motion includes an occasional short antenna-to-active-task zap,
using a cool secondary color blended with the theme accent. It is decorative,
not a provider event. Calm/Reduced/Off or unavailable/resting sources suppress it.
State text is left aligned immediately beside its icon.
