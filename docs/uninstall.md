# Remove the Miilo widget

This source version adds receipt-backed removal of the fleet widget. It does not
purge private data, stop independent observers, remove the launcher, or delete
shared miilo-owl/miilo-local binaries. Older published helpers may lack this
command; check `miilo-owl uninstall --help` and use matching installer/helper code.

## Preview, then confirm

New installs save an installation receipt in
`${XDG_STATE_HOME:-$HOME/.local/state}/miilo/installations/widgets.json`.
Run:

```sh
miilo-owl uninstall widgets
```

Read the JSON `remove`, `retained` and `plan_id` fields. Nothing is removed by this
command. Copy the returned plan_id into the confirmation command:

```sh
miilo-owl uninstall widgets --confirm PLAN_ID
```

Only the receipt-owned `70-miilo-fleet.owl` is removed. Its content must match the
installation receipt. A changed plan, edited widget, symlink or missing receipt
fails closed. Installation and removal need no API/model credentials or network.
A repeat confirmation after successful removal reports already_absent.

Backups, legacy disabled widgets, private connection/chat settings and the receipt
remain. A live model endpoint is not stopped and its key is not revoked. The
shared helper remains available for other clients and reinstall.

If a concurrent change is detected after staging removal, a
`.miilo-removal-PID` recovery file may remain beside the widget. Inspect that
file before retrying; do not delete or restore it blindly. Partial failures are
reported as failures, not success.

## Old installs and reinstall

An old install has no receipt. Automatic removal refuses it instead of guessing
ownership. Review/back up your widget edits before updating with the matching
installer; the installer records the current installed widget. Never enroll
private sources or start agent work just to test an uninstaller.

Reinstall the same reviewed package using `install.sh --no-setup` if you want to
retain settings without reopening setup. It restores the widget and refreshes
its receipt. Other widgets must remain unchanged.

## Host validation

Rust tests exercise actual helper processes, real installer removal/reinstall,
stale confirmations, modified files, symlinks, private-data retention and repeat
removal in disposable directories. ARM64 Linux and Android checks are separate
from native Omalauncher rendering. The Omarchy plugin has its own removal wrapper
and native acceptance handoff; this command does not remove that integration.
Data purge and shared binary/service removal remain explicit separate operations.
See [recorded validation and remaining native gates](uninstall-validation.md).
