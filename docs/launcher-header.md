# Search toolbar and native launcher header

The current Miilo layout is stock-compatible OWL6. Cog and refresh share the search row; the system filter also appears when multiple systems are connected. Tapping search replaces the compact preview with a focused input and hides the adjacent toolbar controls so editing has more room. Submit or the checkmark restores the compact row without losing the query. Task/agent detail views retain a separate contextual back/settings/refresh row because those views have no search field.

Stock OWL6 supports requesting Input focus but not focus/blur callbacks. This is an explicit editing state: dismissing the Android keyboard alone does not end editing; use submit or the checkmark. No launcher patch is required for this layout. The default installer and emulator use the same generated source. `cargo xtask header-variant` is retained as a compatibility export; it no longer injects an OWL8 native header. Its optional preview-robots switch changes only the clearly labeled decorative examples.

## Native close icon and historical patch

The launcher's own close icon is outside widget control. Stock Omalauncher controls its color. The archived `patches/omalauncher-header-actions.patch` records the earlier local header integration against upstream commit `ca99985b3536d26dfd8935391bbfa5c0d215488a`; that patch includes a theme-accent close icon and side-by-side debug app packaging. It is not applied by the widget installer, and installing a separately packaged launcher does not migrate the original Linux guest/configuration.

The previous emulator used that patched host. The current widget toolbar no longer depends on its OWL8 extension. Do not describe the native close-color patch as a widget update. No APK or native runtime libraries are committed here. The patch modifies upstream code and remains subject to that project's license.
