# Miilo typography and setup availability

Checkpoint: 2026-10-17. Source and offscreen Qt verification, not installed-host
or Android acceptance.

## Shared roles and host preferences

`desktop/runtime/MiiloTypeScale.js` defines the role sizes. `MiiloTypography.qml`
exposes them to native QML and the bundled OWL renderer. `desktop/TypeText.qml`
applies the family, role, leading, wrapping and literal text format consistently.

| Role | Desktop | Phone profile |
| --- | ---: | ---: |
| title | 16 | 20 |
| body / muted | 13 | 16 |
| small / label | 12 | 14 |
| huge (display only) | 32 | 34 |

Values are logical units before the host text preference. Qt handles display
density; do not multiply fonts by devicePixelRatio or widget/grid dp. Body text
uses 1.45 leading, titles use 1.25. OWL text boxes use the measured text height
instead of the previous forced 1.17-em boxes. Authored OWL `fontSize` overrides
remain supported and follow text scale; this does not rewrite their semantic role.

Omarchy's dashboard supplies `Style.fontScale` and `Style.font.family`. Portable
hosts can bind `hostTextScale` and `hostFontFamily` on MiiloSurface. The renderer
uses the system monospace alias instead of a hardcoded JetBrains font. No new
process-wide Qt/DPI environment changes are needed.

For controlled previews, `MIILO_TEXT_PROFILE=phone` selects the phone role table
and `MIILO_TEXT_SCALE=2` multiplies the host preference. Invalid/nonpositive scale
values fall back to 1. These are host inputs, not automatic Android accessibility
integration. Android Compose/Omalauncher is a separate renderer: it still needs
its own platform text-scale adapter and actual-device qualification.

## Layout and availability

Onboarding scrolls as a whole. Controls and session rows measure their text;
action grids and the dashboard header reflow on narrow or enlarged layouts.
Long session names and paths wrap instead of disappearing behind an ellipsis.
Native labels, fields, dropdowns, states and errors use explicit type roles.

SetupListing puts an accent-colored setup path beside the name when space
permits, otherwise below it. A circled check means import support, not that an
agent is currently running or enrolled. A blocked icon, muted text and a short
explanation identify unsupported setups. Icons expose accessible names and do
not depend on an installed icon font. Home shortening only replaces a real home
prefix. Paths remain literal text, not markup or an implicit open action.

Unsupported session delegates are disabled. Existing controller guards still
exclude them from individual and group selection. Groups with no supported
sessions keep their expand control but have no selection checkbox. When every
detected setup is unsupported, Review sessions is disabled and the explanation
states why. No discovery, enrollment or provider capabilities are added here.

## Verify and apply

Run `make check-typography` in either repository. The offscreen tests exercise
narrow layouts, 200% text, scroll reachability, wrapped rows, independent geometry
scaling, path placement and disabled unsupported sessions. The native repository
also runs `make check` and `make check-native` for package and lint checks.
An offscreen pass does not prove compositor focus, touch, contrast in every theme,
screen-reader behavior or actual phone rendering.

For the existing flat desktop plugin layout, run:

```sh
bash tools/update-installed-typography.sh
omarchy restart shell
```

The updater backs up each existing Miilo plugin under the user's XDG state
directory, prints the backup location, and copies the required QML/runtime files.
It preserves connection data, enablement and other plugins. It refuses the newer
native plugin layout, which must use its own package installation workflow.
An optional first argument selects a disposable plugin directory for testing.

## References

- [Qt Quick scalability](https://doc.qt.io/qt-6/scalability.html)
- [Material typography](https://m3.material.io/styles/typography/applying-type)
- [Android large-text support](https://developer.android.com/about/versions/14/features)
- [Apple automatic font scaling](https://developer.apple.com/documentation/uikit/scaling-fonts-automatically)
- [Large-font UI tests by Eeva-Jonna Panula](https://dev.to/eevajonnapanula/writing-tests-for-larger-font-sizes-with-compose-scrolling-and-text-truncation-1l0)
