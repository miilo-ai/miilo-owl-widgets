#!/usr/bin/env bash
# Run inside Omalauncher Linux, with launcher visible and installed widgets.
set -euo pipefail
command -v owl-check >/dev/null || { echo 'Run inside Omalauncher Linux with owl-check available.' >&2; exit 1; }
folder="$HOME/.config/omalauncher/widgets"
owl-check "$folder/70-miilo-fleet.owl" "$folder/71-miilo-attention.owl" "$folder/72-miilo-board.owl"
printf '\nReview available PNGs under %s/.preview in each chosen theme.\n' "$folder"
printf 'Omalauncher may skip previews for widgets containing Terminal; inspect those on screen or capture with ADB.\n'
