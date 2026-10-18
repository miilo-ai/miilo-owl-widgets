#!/bin/sh
# Install the prebuilt helper inside Omalauncher's Linux environment.
set -eu
repo=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
base="${MIILO_OWL_INSTALL_ROOT:-$HOME/.local}"
widgets="${MIILO_OWL_WIDGET_DIR:-$HOME/.config/omalauncher/widgets}"
binary="${MIILO_OWL_BINARY:-$repo/bin/linux-aarch64/miilo-owl}"
if [ ! -x "$binary" ] || ! "$binary" --version >/dev/null 2>&1; then
  echo 'A compatible native miilo-owl binary is required. Use the ARM64 Linux release bundle, or set MIILO_OWL_BINARY to a locally built binary.' >&2
  exit 1
fi
"$binary" uninstall --help >/dev/null
mkdir -p "$base/bin" "$widgets"
temp="$(mktemp "$base/bin/.miilo-owl.XXXXXX")"
trap 'rm -f -- "$temp"' EXIT
cp -- "$binary" "$temp"
chmod 755 "$temp"
mv -f -- "$temp" "$base/bin/miilo-owl"
for file in "$repo"/70-miilo-fleet.owl; do
  name="$(basename -- "$file")"
  if [ -f "$widgets/$name" ]; then
    backup="$widgets/$name.before-miilo-1.3"
    [ -e "$backup" ] || cp -- "$widgets/$name" "$backup"
  fi
  cp -- "$file" "$widgets/$name"
done
# Preserve prior focused widgets outside the launcher's .owl discovery pattern.
for name in 71-miilo-attention.owl 72-miilo-board.owl; do
  if [ -f "$widgets/$name" ]; then
    mv -f -- "$widgets/$name" "$widgets/$name.disabled"
  fi
done
"$base/bin/miilo-owl" record-widget-install "$widgets" "$base/bin/miilo-owl" >/dev/null
printf 'Installed miilo-owl and the Miilo widget.\n'
printf 'Ensure %s/bin is in your launcher Linux PATH.\n' "$base"
if [ "${1:-}" != "--no-setup" ]; then
  "$base/bin/miilo-owl" setup
fi
