#!/usr/bin/env bash
# Update the historical flat desktop plugin layout, preserving an undo copy.
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
plugins="${1:-${XDG_CONFIG_HOME:-$HOME/.config}/omarchy/plugins}"
backup_root="${XDG_STATE_HOME:-$HOME/.local/state}/miilo/plugin-backups"
mkdir -p -- "$backup_root"
backup=$(mktemp -d "$backup_root/typography.XXXXXX")
chmod 700 "$backup"
updated=0
for plugin in io.miilo.bar io.miilo.dashboard io.miilo.desktop; do
    target="$plugins/$plugin"
    [[ -d "$target" ]] || continue
    if [[ -L "$target" || ! -f "$target/LocalImport.qml" || ! -f "$target/runtime/OwlWidget.qml" ]]; then
        echo "Unsupported layout: $plugin. Use its native package installer." >&2
        exit 1
    fi
    cp -a -- "$target" "$backup/$plugin"
    for file in AvailabilityIcon.qml SetupListing.qml TypeText.qml InteractionTheme.qml MiiloSurface.qml Dashboard.qml LocalImport.qml \
        runtime/MiiloTypeScale.js runtime/MiiloTypography.qml runtime/OwlHost.qml \
        runtime/OwlWidget.qml runtime/OwlNode.qml runtime/UPSTREAM.md omarchy/Dashboard.qml; do
        cp -- "$root/desktop/$file" "$target/$file"
    done
    echo "Updated $plugin"
    updated=$((updated + 1))
done
if [[ "$updated" == 0 ]]; then
    rmdir -- "$backup"
    echo "No legacy Miilo plugins found." >&2
    exit 1
fi
echo "Backups: $backup"
echo "Run: omarchy restart shell"
