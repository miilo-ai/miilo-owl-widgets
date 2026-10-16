Vendored from https://github.com/SimonSchubert/omarchy-widgets
Revision: 0741f7ff03d0ef357b4f69af8c0ec2586669cd8d
Path: runtimes/quickshell
License: MIT (see LICENSE).

Local patch: OwlWidget exposes errorDir (default unchanged). Desktop hosts put
diagnostics under the state directory, preventing plugin hot-reload loops. Local hosts and integration live one directory up.
