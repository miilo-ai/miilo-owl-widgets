# Omarchy plugin repository handoff

The independent native plugin lives in https://github.com/miilo-ai/miilo-omarchy-plugin
with clean history and its own root manifest. See its `.specify/handoffs/omarchy-agent.md`
for native validation and `docs/marketplace.md` for owner-authorized registration.

Official references:
- https://plugins.omarchy.org/develop.html
- https://plugins.omarchy.org/publish.html
- https://github.com/omacom/omarchy/blob/quattro/docs/omarchy-shell.md

The plugin owns Omarchy presentation. Widgets retains shared OWL and miilo-owl;
CLI retains miilo-local, SDK retains the domain core. The plugin needs no private
systems source. Source snapshot/provenance is explicit; native integration edits
belong in the plugin and shared renderer fixes must be coordinated upstream.

Existing desktop installers and local checkouts are preserved for compatibility.
Do not enable two io.miilo.bar implementations or overwrite user-owned folders.
The canonical new marketplace installation is the plugin repository, not widgets.
Legacy dashboard/desktop entry points need deliberate migration if still desired.

Widgets remains under EdyVision until its owner transfers it to miilo-ai. Do not
pretend the transfer happened or change remotes before confirming it. All intended
open-source repos can be opened together; private-application must remain private.
No release has been made, no visibility changed and no marketplace issue submitted.
