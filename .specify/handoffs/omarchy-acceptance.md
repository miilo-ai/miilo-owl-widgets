# Handoff: native Omarchy acceptance and presentation

Target: actual the designated Omarchy test host. Systems baseline 891d278, widgets e92b895.
Core extraction is in progress; do not infer standalone acceptance from the old CE-backed path.

## Ownership

You may edit widget desktop/ QML, tests/desktop/, tools/test-desktop-keyboard.sh,
and native theme/keyboard/layout documentation. Coordinate installer modifications
with core agent. Do not edit SDK runtime, CLI local engine, CE, widget helper
configuration/transport/local_import, or release workflows without agreement.
No publication, AUR retries, global shell changes, agent task launches or production approvals.

## Target wiring

Keep miilo-local discover/groups/import command protocol. Public CLI will provide
miilo-local as a second binary; local import creates version-2 mode=local config,
snapshotFile and explicit sessions/groups. Local observer uses the shared runtime
and atomic private cache; CE is absent. Widget helper connect-local handles the
cache, not a URL/token. Existing remote connections and version-1 configurations
remain intact. Use installed candidate binaries, not a private systems build.

## Validate and record

1. Record OS, Omarchy shell, Quickshell/Qt versions, git revisions and binary hashes.
2. Fresh disposable XDG config/data/state; install native surfaces without silently
   enabling plugins or editing global shell configuration. Confirm manifests valid.
3. Existing-session invitation: Not now persists; new setup invites again. Select
   individual Codex/Cursor sessions and groups; unchecked existing sessions stay out.
4. Future group opt-in remains off by default. Folder scope/pattern and symlink escape
   exclusions hold; removing a group does not delete existing selections.
5. Stop CE / make its executable unavailable in the disposable test environment.
   Local import and natural transcript updates still reach robots. Record process
   tree/listeners proving no CE or Miilo HTTP listener. Never stop unrelated services.
6. Tab/Shift+Tab, arrows, Space, Enter, Escape, Ctrl+L/G/R/Enter work. No focus trap,
   clipped selection, off-center icons, unreadable text or unintended shell launch.
7. Light/dark and at least two accents; reduced motion; empty/one/many systems;
   working/resting/unreachable/attention distinct; expanded evidence scrolls.
8. Disconnect one source, stop/restart observer, remove one selection; preserve
   other local/remote systems. Stale cache must be explicit, not healthy sleep.
9. Native bar/dashboard/desktop surface uses existing shell; no second shell for
   plugins. Portable dashboard is a separate explicitly launched host.
10. Reinstall/upgrade/removal preserve user data and unrelated plugin/config files.

Use make check-desktop and make check-keyboard with the new public miilo-local
candidate; run native UI inspection as well. Qt fixtures alone do not prove live
harness behavior. Save a short evidence report (revision, command, outcome,
redacted screenshots, limitations) in .specify/evidence/omarchy-acceptance.md.
Do not commit credentials, private transcripts or raw screenshots with secrets.

## Sunday recording

Capture expressive compact robots, native local import, clear groups, real natural
updates and progressive detail. Voice/GenUI/action scenes are conditional on their
separate qualified paths; do not simulate unsupported behavior. Use a sanitized
demo workspace. Report blockers immediately with exact redacted error and scope.

## Candidate source and commands

Fetch `codex/standalone-runtime` in SDK, CLI, CE, widgets and systems. Preserve
your current uncommitted work; do not reset it. CLI/CE resolve the full SDK commit
from GitHub, not a private systems path. Until public, use your existing authorized
SSH identity or the documented contents-read credential; never paste secrets.

Build CLI: `cargo build --workspace --locked -j2` in miilo-cli.
Build helper: `cargo build --manifest-path helper/Cargo.toml --locked -j2` in widgets.
Set `MIILO_LOCAL_BINARY` to the resulting public `miilo-local` binary and
`MIILO_OWL_BINARY` to the resulting helper when running `tools/install-desktop.sh omarchy`.
Use native import; it should now write `~/.config/miilo/local.json` with version 2.
Old desktop-local.json/CE configurations are preserved, not silently repurposed.

Test repeat import and group edits while the owned observer is already running.
Only that service may be paused; failed import restores configuration/cache/unit.
Success requires a fresh snapshot from the restarted service, not just a successful
systemctl invocation. Confirm the saved absolute Codex executable works in the
service environment. File updates use native notifications plus a 5-second cache
heartbeat; stale cutoff is 15 seconds. Test future groups through the 60-second
discovery cycle, including an excluded pre-existing session.

Do not install the old private systems `miilo-local` binary over the public one.
Do not stop production CE/services just to test absence; use isolated XDG paths
and make the CE executable unavailable for the test. Record any service readiness
failure exactly (redacted). SDK/CLI/CE source candidates are 0.2.0; helper is 1.4.0.

## Future-enrollment consent regression

The discovery window is bounded. Prove an older session omitted from the initial
window cannot later enroll simply by becoming recently updated. Future matches
must have a verified creation timestamp after the server-bound enrolledAfter
cutoff; missing, older or implausibly future timestamps are excluded. Editing an
unchanged opted-in rule preserves its cutoff; enabling/changing a rule starts a
new one. Existing selected sessions remain selected when future enrollment is off.
Legacy remote rules without a cutoff pause future enrollment explicitly.

If repositories are still private and HTTPS Cargo access is unavailable, an
existing authorized GitHub SSH identity can be used without token values:

```sh
CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 \
GIT_CONFIG_KEY_0='url.ssh://git@github.com/miilo-ai/miilo-sdk.insteadOf' \
GIT_CONFIG_VALUE_0='https://github.com/miilo-ai/miilo-sdk' \
cargo build --workspace --locked -j2
```

This is a per-command rewrite, not a global Git configuration change. Once public,
the ordinary pinned HTTPS dependency needs no cross-repository token.

## Separate plugin repository

Native plugin work now targets `miilo-ai/miilo-omarchy-plugin`. Read
`omarchy-plugin-repository.md` before modifying or installing native entry points.
Its own handoff includes the official development and publishing guides.
