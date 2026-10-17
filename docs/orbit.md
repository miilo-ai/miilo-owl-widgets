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
