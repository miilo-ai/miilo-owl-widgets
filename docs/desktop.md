# Miilo on Linux

> Native Omarchy marketplace development now lives in the separate
> [miilo-omarchy-plugin](https://github.com/miilo-ai/miilo-omarchy-plugin) repository.
> This document includes legacy prototype/install validation; for current standalone
> wiring and registration use `.specify/handoffs/omarchy-plugin-repository.md`.
> Existing desktop source/installers remain for compatibility during migration.

The application helper and developer tooling are Rust. OWL is the shared widget
source; the pinned upstream Quickshell runtime renders it on Linux. Python is not
required for building, testing, installing, or releasing this repository.

## Build and test

```sh
make test
make assemble
make check-desktop
MIILO_LOCAL_EXECUTABLE=/path/to/miilo-local make check-keyboard
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

The dashboard detects existing agent setups and offers **Review sessions**.
Detection reads filesystem metadata only. It does not enroll sessions, open
credentials or start work. Select existing Codex/Cursor sessions explicitly.
Devin/Claude setup detection does not imply live observation support.

Install `miilo-local` from miilo-cli and `miilo-owl` from this repository. Use the
candidate versions and native instructions in the separate Omarchy plugin handoff.
Confirmed local imports write version-2 `~/.config/miilo/local.json`, start the
owned collector service and connect its private snapshot cache. CE is not required.
Existing remote endpoints and version-1 configurations remain intact.

Sessions are grouped by agent and project folder. **Add group** creates named
rules such as Codex under `~/Work` (enter the absolute path), or projects matching
`vave-*` / `miilo-*` beneath that folder. `*` and `?` match project-name text.
Group headings select all supported sessions; individual selections remain
available, and the +/- control folds a group. **Edit group** changes or removes
rules. Changes are saved only when you confirm **Apply groups & import**.

**Include future matching sessions** is off by default. Explicitly opted-in rules
are checked every minute and enroll newly discovered matching sessions, up to
100 total. Existing unchecked sessions stay excluded. Disabling or removing a
rule stops future enrollment after confirmation and retains existing selections.
Cursor folder grouping requires its workspace path to resolve within the chosen
discovery directory; unresolved workspaces remain individually selectable.

Keyboard controls: **Tab / Shift+Tab** move between controls; **Up / Down** move
through groups and sessions; **Space** toggles a selection; **Enter** folds a group
or activates a control; **Left / Right** collapse/expand a group; **Escape** goes
back or closes the dashboard. **Ctrl+L** opens local agents, **Ctrl+G** adds a
group, **Ctrl+R** rescans, and **Ctrl+Enter** confirms import. Focus uses accent
markers and a tint; selection stays visible through checkboxes, with a dash for
partially selected groups. The Omarchy dashboard reads native shell interaction
tokens; portable hosts use theme-derived defaults. `make check-keyboard` sends actual key events through QtTest inside
Quickshell, using isolated metadata and the real Rust group preview command.

To stop background observation, run
`systemctl --user disable --now miilo-local-desktop.service`.
The separate `miilo-local setup` terminal workflow remains available for custom
CE connections; it uses `~/.config/miilo/local.json`.

Devin discovery uses its official directory-scoped list command. Its live
observation adapter is not implemented and is labelled discovery-only. No agent
work is started or resumed. History is not treated as current activity; robot
movement reflects recently observed public activity, not authoritative execution.

## Validation checkpoint

On this Linux x86-64 Omarchy machine: Rust suites, credential-file rotation, 36
Linux evaluator scenarios, 264 official OWL tile/screen evaluations of all three widgets,
headless Qt rendering and invitation/review/import UI automation, native manifest validation, dashboard and desktop-layer
loading, and a Rust journey through a real temporary CE process passed. The CE
journey used two synthetic sessions and checked public/private content boundaries,
robot/work association, independent disconnect, and rotated/rejected credentials.
An isolated import journey also checks selection validation, service startup,
automatic widget configuration, and preservation of existing endpoints.
Read-only discovery found eight existing local Codex sessions. No user's sessions
were automatically enrolled. Devin returned no sessions in the two projects
checked; that does not establish that none exist elsewhere.

The reload-loop fix relocates diagnostics out of the plugin source tree. This is
a prototype acceptance checkpoint, not public-release, ARM desktop, physical-phone,
Debian/Ubuntu, or live Devin qualification. Package publication remains disabled.
