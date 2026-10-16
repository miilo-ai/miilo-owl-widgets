# Miilo on Linux

The application helper and developer tooling are Rust. OWL is the shared widget
source; the pinned upstream Quickshell runtime renders it on Linux. Python is not
required for building, testing, installing, or releasing this repository.

## Build and test

```sh
make test
make assemble
make check-desktop
make build
```

`make test` covers Rust boundaries, real localhost HTTP/CLI journeys, Android and
desktop installers, credential files, and release policy. `make check-desktop`
uses Rust-generated fixtures with the actual upstream JavaScript evaluator: Node
is a development dependency for this check only. `make check-owl` runs the official
Java/Kotlin compiler/evaluator when `OWL_CHECKER_CLASSPATH` and
`KOTLIN_COMPILER_CLASSPATH` are supplied. The official compiler remains the syntax
authority; no custom Rust language implementation is substituted.

## Install a portable dashboard

Install Quickshell through your distribution, then:

```sh
MIILO_OWL_BINARY="$PWD/helper/target/release/miilo-owl" tools/install-desktop.sh portable
miilo-desktop dashboard
```

The installer is a thin shell entry point into the Rust helper. The dashboard is
installed under `$XDG_DATA_HOME/miilo/desktop` (default `~/.local/share`), with a
Miilo application entry and executables in `~/.local/bin`. Debian/Ubuntu packaging
is not qualified yet; the standalone host avoids Omarchy imports, but a working
Quickshell installation is still required. Tray and GNOME/KDE desktop adapters
remain separate platform work.

## Native Omarchy surfaces

```sh
tools/install-desktop.sh omarchy
omarchy-shell shell rescanPlugins
omarchy plugin enable io.miilo.bar
omarchy plugin enable io.miilo.dashboard
omarchy plugin enable io.miilo.desktop
miilo-desktop omarchy-dashboard
miilo-desktop omarchy-desktop
```

Each plugin is independently optional. The installer stages files without enabling
plugins or changing shell.json. The bar opens a dashboard with current attention
and connectivity. The desktop tile uses a bottom Wayland layer and expands on
interaction. The dashboard uses a regular application window hosted inside the
existing Omarchy shell. Native plugins never launch a second Quickshell process.
Escape closes the dashboard or returns an expanded tile to its compact state.

Both hosts consume the exact generated `70-miilo-fleet.owl` and the same native
helper/cache. Credentials stay in protected helper configuration, never QML state.
Theme changes are watched; the desktop adapter lifts muted text toward foreground
where needed for readable contrast. Diagnostic files live under
`$XDG_STATE_HOME/miilo/widgets/errors`, outside plugin source directories, so they
cannot trigger Omarchy's plugin hot-reload watcher.

Use `omarchy plugin disable io.miilo.bar` (or the other surface ID) to turn a
surface off. User-owned plugin folders and the portable application entry can be
removed independently; keep `~/.config/miilo-owl` if you want to retain connections.

## Connect existing local agents

The dashboard's **Connect local agents** control runs the native `miilo-local`
setup. Its source belongs in the sibling `private-application/bridges/desktop` project,
with the existing Codex and Cursor observation adapters.

Opening the dashboard automatically detects known Codex, Cursor, Devin, and
Claude Code folders and executables. This reads filesystem metadata only and
displays **Found locally**; it does not open credentials, read transcripts, start
agent commands, or register sessions. The settings cog uses a drawn monochrome
outline and the desktop controls use square, flat theme colors.

Build that binary with `cargo build -p miilo-local -j 2` from private-application. Build
`miilo-ce` from its repository. Supply both binaries when installing:

```sh
MIILO_LOCAL_BINARY=/path/to/miilo-local MIILO_CE_BINARY=/path/to/miilo-ce \
  tools/install-desktop.sh omarchy
miilo-local setup /path/to/project
miilo-local run
miilo-owl setup
```

Setup discovers existing Codex sessions and Cursor transcripts, asks which to
observe, and stores their explicit selection in `~/.config/miilo/local.json`.
It can own a loopback-only CE process, or use an existing CE. In widget setup,
add the CE endpoint, enter `@/absolute/path/to/ce.token` instead of copying the
credential, then select its registered providers. Actions remain disabled for CE.
Credential-file references are reread on each HTTP request so rotation works.

Devin discovery uses its official directory-scoped list command. Its live
observation adapter is not implemented and is labelled discovery-only. No agent
work is started or resumed. History is not treated as current activity; robot
movement reflects recently observed public activity, not authoritative execution.

## Validation checkpoint

On this Linux x86-64 Omarchy machine: Rust suites, credential-file rotation, 36
Linux evaluator scenarios, 264 official OWL tile/screen evaluations of all three widgets,
headless Qt rendering, native manifest validation, dashboard and desktop-layer
loading, and a Rust journey through a real temporary CE process passed. The CE
journey used two synthetic sessions and checked public/private content boundaries,
robot/work association, independent disconnect, and rotated/rejected credentials.
Read-only discovery found eight existing local Codex sessions. No user's sessions
were automatically enrolled. Devin returned no sessions in the two projects
checked; that does not establish that none exist elsewhere.

The reload-loop fix relocates diagnostics out of the plugin source tree. This is
a prototype acceptance checkpoint, not public-release, ARM desktop, physical-phone,
Debian/Ubuntu, or live Devin qualification. Package publication remains disabled.
