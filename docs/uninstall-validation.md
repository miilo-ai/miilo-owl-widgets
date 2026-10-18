# Uninstall validation

Validated on 2026-10-18 for the source implementation in this branch. These
checks do not establish availability in an older published release.

| Environment | Evidence | Boundary |
| --- | --- | --- |
| macOS | Full helper and build-tool tests; strict Clippy; installer/removal round trips | Disposable fixture directories |
| ARM64 Linux (musl) | Full helper suite; install/removal tests rerun after path portability fix | Container with ARM64 execution |
| Android ARM64 emulator | Native helper installed, removed, repeated and reinstalled in two cycles; modified widget refused; private fixture, unrelated widget and shared helper retained | ADB shell, isolated temporary directory; live launcher unchanged |
| Desktop presentation | 36 generated OWL scenarios plus robot, orbit, portrait, compact and visibility checks | Render contracts, not native unload acceptance |

The Android run used the source installer with `--no-setup`, a locally built
static ARM64 helper and isolated HOME, XDG_STATE_HOME, install-root and widget
paths. Each cycle verified a read-only preview, rejected a wrong plan ID,
confirmed removal, repeated removal and checked retained fixture contents.
It required no provider credentials or network calls.

Android exposed execute-only ancestor directories. Linux directory traversal
uses O_PATH with O_DIRECTORY/O_NOFOLLOW so those directories can be traversed
without granting read access or following symlinks. The installer is POSIX sh.

Still required before marketplace acceptance: native Omalauncher widget unload,
process/request cessation, launcher restart and reinstall in its actual Linux
environment. The separate Omarchy plugin handoff covers native shell disable,
manager removal, backups and shared-resource retention. Data purge, credential
revocation and automatic shared binary/service removal are outside this change.
