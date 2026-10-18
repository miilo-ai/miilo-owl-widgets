# Compact and Detail

Compact shows a space-efficient robot fleet with up to four columns and additional
rows as the viewport permits. Select a robot to inspect its system's tasks.
Detail shows task rows beside the same robots and orbital state cues. The saved
values remain `quiet` and `orbit` for compatibility. View selection is independent
of Full/Reduced/Off motion and Calm/Expressive personality settings.

Each system uses two crossed elliptical tracks centered on the original robot.
Tracks and task glyphs share the same projection and phase, so moving glyphs stay
on their tracks. Per-system offsets prevent synchronized motion. Dormant systems
use muted rails; stale/unavailable systems freeze. Layered theme-colored strokes
provide a subtle glow without a platform-specific blur dependency.

The nucleus stays clear of the robot face. In Expressive Full motion, active
systems may show a brief antenna-to-task electrical filament. This is decoration,
not a network packet, approval, model call or proof of agent communication.
Reduced/Off freezes orbital geometry; the robot's separate motion rules apply.

Select a system tile or task row to open ordinary inspection. Needs you and search
filter observed work; overflow opens that system's full task list. Glyphs on the
tracks are decorative rather than separate click targets. Back restores the
inspection entry mode. Failed work uses asymmetric round eyes with `!!`; pending
attention uses `?!`, including detail portraits. Crossed eyes denote unavailable
observation, rather than a task failure. Colors come from the current theme.

## Validation and capture

Run `make assemble`, `make test` and `make check-desktop`. The `check-owl` target
also requires the official OWL checker and Kotlin compiler classpaths. Controlled
time tests execute the generated widget, compare original portrait drawing
operations and check orbit bounds, motion, selection and state isolation.

Use the native Qt/Node capture workflow in
[the toolchain guide](development-toolchain.md). It renders synthetic inventory
through the actual evaluator, layout and Canvas; it cannot issue provider actions.
It does not establish installed Omarchy lifecycle, native performance or physical
Android acceptance. Native acceptance must independently cover keyboard/focus,
themes, scaling, touch scrolling and many-system performance.

Onboarding portraits consume `desktop/RobotProgram.js`, generated from the shared
Bot and palette by `make assemble`. They must not duplicate robot geometry in QML.
Event-driven one-shots, glyph picking, automatic OS motion preference detection
and complete zero-frame scheduling in Off remain outside the qualified behavior.
