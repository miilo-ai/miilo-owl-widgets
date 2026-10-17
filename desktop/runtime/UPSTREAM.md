Vendored from https://github.com/SimonSchubert/omarchy-widgets
Revision: 0741f7ff03d0ef357b4f69af8c0ec2586669cd8d
Path: runtimes/quickshell
License: MIT (see LICENSE).

Local patches: OwlWidget exposes errorDir (default unchanged). Desktop hosts put
diagnostics under the state directory, preventing plugin hot-reload loops.
OwlWidget also accepts a configureLocal callback; OwlNode routes Miilo's setup
terminal to native onboarding when supplied, retaining the terminal otherwise.
Interactive OwlNode elements support Tab focus, Enter/Space activation, and
compact focus markers; text inputs participate in Tab navigation.
Local hosts and integration live one directory up.

Miilo typography patch: shared semantic desktop/phone roles and a host text-scale
port, independent of grid dp and device-pixel ratio. OwlNode measures multiline
text height, gives body text additional leading and renders observation text
literally. Explicit OWL fontSize remains an authored logical-size override; it
still follows the host text scale. New MiiloTypography.qml/MiiloTypeScale.js are
Miilo-authored additions. Android Compose/Omalauncher is a separate renderer and
requires its own host integration and on-device acceptance.
Canvas explicitly selects Canvas.Image: Qt 6 ignores FramebufferObject. This
corrects the previous GPU-path comment; native performance still needs measurement.
