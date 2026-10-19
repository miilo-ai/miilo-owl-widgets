# Paired computers in launcher widgets

Use the shared `miilo-pairing` client from the CLI repository, pinned to an exact
reviewed revision. Do not duplicate TLS or handshake policy. Pairing must remain
optional and explicit, with a dedicated settings page and deferred terminal.
Existing endpoints, selected systems, chat and local observation remain intact.

The helper receives a private invitation, requests host approval, displays a
comparison code, then lets the user select returned providers. Configuration is
saved atomically under the existing private store. A paired endpoint cannot mix
ordinary credentials, snapshot files or action authority. Its HTTPS origin and
certificate are pinned. Errors use fixed safe diagnostics, never raw payloads.

Explicitly selected paired sessions stay visible while idle, using their reported
sleeping state. Do not change historical-session filtering for local discovery.
Host names must be human-readable rather than generated connection identifiers.

Validate headless pairing, native Android execution inside the launcher Linux
environment, actual widget rendering, deferred settings navigation, separate
credentials, revocation, host restart and stale-state recovery. Preserve original
installed files/preferences during acceptance and record their restoration.

Explain uses the explanation already present in a fresh observation and preserves
its revision and limitations. Paired clients never POST to the host for it.
