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
