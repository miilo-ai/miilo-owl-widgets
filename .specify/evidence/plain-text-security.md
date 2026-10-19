# Plain-text renderer security acceptance

Date: 2026-10-18. Marketplace finding:
https://github.com/omacom/omarchy-plugin-marketplace/issues/12123

## Fix

Explicit Text.PlainText on runtime buttons, input placeholders and terminal
output; existing body-text protection remains. Static runtime labels are also
explicitly plain. Editable TextInput remains unchanged: it does not interpret
HTML. Native app controls use the existing plain-text TypeText component.

No HTML escaping is applied to the underlying observation data. Characters stay
literal at presentation, avoiding double escaping and preserving stored evidence.
Explicit image nodes retain their existing behavior; this fix concerns implicit
image loading through text. Upstream notices, patch files and source hashes are
retained.

## Native Omarchy evidence

All tests ran in disposable source copies and isolated offscreen Quickshell
instances, not the installed desktop shell. Original user configuration was not
modified.

The exact unpatched plugin renderer was tested using the same harness. It failed
the plain-text assertion and sent three unintended loopback image requests from
button, placeholder and terminal payloads. The existing body label did not fetch.
Both patched renderers passed: zero requests from these four surfaces. A separate
explicit RichText control successfully fetched an image, proving that network
observation was active rather than disabled. The fixture never contacts an
external server.

Plugin: make check-native check-typography passed, including 47 source tests,
QML lint, typography, configuration and the native canary. Existing offscreen
platform/import-scanner warnings remain; this is not a warning-free claim.
Widgets: native check-typography, robot-motion, orbit, portrait (54 comparisons),
compact, session-visibility and 18 mobile settings scenarios passed. Local source
guards, installer tests, architecture/no-Python gates and official OWL compilation
passed. The Omarchy test host has no cargo on its noninteractive PATH, so the
combined make check-desktop target was not run there; its Node renderer tests
were run directly. No new helper protocol or observation behavior was introduced.

## Reproduction commands

Plugin: make check-native check-typography
Widgets: make check-typography
The check-plain-text target independently runs the real Qt network canary.
Normal CI runs source guards to prevent implicit text-format regressions.

Qt guidance: https://doc.qt.io/qt-6/qml-qtquick-text.html#textFormat-prop

No release publication, marketplace reply, installation into the live shell or
pull request creation was performed during this acceptance.
