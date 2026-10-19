# Omarchy integration guide

These are Miilo's required integration methods, not an upstream Omarchy approval
or security certification. Follow the current [Omarchy development guide](https://plugins.omarchy.org/develop.html)
and [publishing guide](https://plugins.omarchy.org/publish.html) for platform contracts.

## Treat observed text as data

Provider names, agent/session names, task titles, action labels, evidence, errors,
input placeholders and process output are untrusted display data. They must not
become markup, executable code or implicit resource requests merely by rendering.

For every QML `Text` or `Label` that can display these values, explicitly set:

```qml
Text {
    text: providerLabel
    textFormat: Text.PlainText
}
```

Use Miilo's existing `TypeText` component where available; it already enforces
plain text. The shared OWL runtime must apply the same rule to body labels,
buttons, placeholders and terminal output. Audit newly added text surfaces too.
Editable `TextInput` is already plain text; do not add an unsupported `textFormat`
property to it. For `TextEdit`/`TextArea`, explicitly select the supported plain-text
mode when rendering untrusted content.

Qt automatically detects formatted text unless told otherwise. HTML image tags
can therefore fetch URLs just by displaying a string. See [Qt's explicit warning
for untrusted text](https://doc.qt.io/qt-6/qml-qtquick-text.html#textFormat-prop).

Do not sanitize by rewriting stored observation data or globally HTML-escaping it.
Keep original evidence intact and enforce literal rendering at the display boundary.
A formatter for intentional rich content requires separate design review, an
explicit trust/resource policy and tests. Do not remove the plain-text protection
to support provider-supplied formatting. Explicit image nodes are a separate
resource-loading surface; this guide does not certify arbitrary image URLs.

## Validate behavior, not only source

- Normal CI must reject runtime text items that omit explicit plain-text formatting.
- Run `make check-plain-text` on a native Qt/Quickshell host for renderer changes.
  The test displays HTML/image payloads in buttons, body labels, placeholders and
  terminal output, and requires zero image requests from those surfaces.
- The request monitor must have a positive control: an explicitly rich-text test
  item fetches a loopback image. Otherwise zero requests could mean a broken test.
- Keep the HTTP canary bound to loopback. Do not use external tracking services,
  real credentials or production endpoints as security fixtures.
- Run the existing typography, configuration and source checks. Record the tested
  revision, native environment, results and limits in `.specify/evidence/`.

## Preserve the native shell boundary

Use the installed shell's plugin lifecycle. Never launch a second live Quickshell
instance from plugin code. Native test harnesses must be isolated and offscreen;
this does not authorize changing the user's active shell or configuration.
Use the repository's documented development/relaunch tools for installed testing.

Credentials remain outside widget source. Discovering local sessions does not
permit starting agent work or enrolling them without explicit selection. Rendering
an observation does not authorize an action or change its provider permissions.

## Keep shared and vendored copies aligned

The shared Qt renderer lives in `miilo-owl-widgets/desktop/runtime`. The native
plugin ships its own pinned copy under `vendor/owl`. Fix both when both are affected.
Preserve upstream notices and record local renderer changes as reproducible patches.
Update the plugin's source hashes and patch declarations in `sources.lock.json`.
Run package provenance checks and verify the patch reverses cleanly against the
fixed source. Do not overwrite newer local patches during a future renderer import.

A clean native regression run is evidence for the tested behavior, not a complete
security audit or marketplace approval. Marketplace maintainers perform their own
review. Keep the submission's dependency and installation disclosures accurate.
