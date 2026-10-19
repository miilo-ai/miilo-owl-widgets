# Omalauncher store installation

## Required helper

Miilo requires `miilo-owl`, a Rust executable. It reads explicitly selected local
agent sessions, retrieves work and evidence from user-configured systems, and
submits supported actions only when requested. Observation does not start agents.
Optional explanations send the selected context to the user's configured model.
Remote factories and models require network access and their own credentials.
Credentials belong in the helper's supported credential storage, never OWL source.

On first use, choose **Install helper & continue**. The visible terminal downloads
the pinned ARM64 Linux release from `miilo-ai/miilo-owl-widgets`, verifies SHA-256,
and installs only `~/.local/bin/miilo-owl`. No root, session enrollment, shell
profile edits, or credential changes occur. Existing helpers are not overwritten.
The checksum detects corrupted or mismatched assets; it is not an independent
signature because the archive and checksum come from the same release account.
Return and refresh after success. Ensure `~/.local/bin` is on the launcher PATH.
Version 1.5.0 must be published with its archive and SHA256SUMS before this workflow
can install it. Missing release assets fail without installing anything.

To remove the helper, delete `~/.local/bin/miilo-owl` after stopping its use.
Remove the widget through the launcher. Configuration and credentials are retained;
use the repository removal guide before deleting any saved connections.

## Instructions for agents opened through Edit widget

1. Read the missing-helper or terminal error before changing anything.
2. Check `command -v miilo-owl`, `~/.local/bin/miilo-owl --version`, Linux architecture,
   and PATH. ARM64 Linux is the currently published mobile bundle target.
3. Explain the dependency above and get the user's consent before installing it.
   Use the widget's installer or the official release-bundle installation guide.
4. On HTTP 404, check whether the pinned release exists. Do not substitute a moving
   branch, an unofficial binary, or a downloaded shell script. Never bypass checksums.
5. Do not print secrets or enroll sessions without the user's selection. Diagnose
   connectivity separately from authorization; a VPN route grants neither credentials
   nor permission to execute remote work.
6. Preserve configuration and existing binaries. Never use root, reset the launcher,
   or replace user settings to fix a missing helper. Report the exact sanitized error.

## Store reviewer disclosure

The widget requires an external Rust helper. A user-triggered first-run terminal
installs a version-pinned GitHub release after SHA-256 verification. The executable
is not bundled in the store. Packages: bash, curl, coreutils, tar, gawk, grep, gzip.
No automatic download occurs on rendering. The helper uses local selected session
files and explicitly configured network endpoints. Its download/execution must be
reviewed under the store's flagged-command policy before listing.
