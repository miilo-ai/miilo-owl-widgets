#!/usr/bin/env bash
# Install the prebuilt helper inside Omalauncher's Linux environment.
set -euo pipefail
repo="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
base="${MIILO_OWL_INSTALL_ROOT:-$HOME/.local}"
widgets="${MIILO_OWL_WIDGET_DIR:-$HOME/.config/omalauncher/widgets}"
binary="${MIILO_OWL_BINARY:-$repo/bin/linux-aarch64/miilo-owl}"
if [ ! -x "$binary" ] || ! "$binary" --version >/dev/null 2>&1; then
  echo 'A compatible native miilo-owl binary is required. Use the ARM64 Linux release bundle, or set MIILO_OWL_BINARY to a locally built binary.' >&2
  exit 1
fi
mkdir -p "$base/bin" "$widgets"
temp="$(mktemp "$base/bin/.miilo-owl.XXXXXX")"
trap 'rm -f -- "$temp"' EXIT
cp -- "$binary" "$temp"
chmod 755 "$temp"
mv -f -- "$temp" "$base/bin/miilo-owl"
for file in "$repo"/*.owl; do
  name="$(basename -- "$file")"
  if [ -f "$widgets/$name" ]; then
    backup="$widgets/$name.before-miilo-1.3"
    [ -e "$backup" ] || cp -- "$widgets/$name" "$backup"
  fi
  cp -- "$file" "$widgets/$name"
done
printf 'Installed miilo-owl and three widgets.\n'
printf 'Ensure %s/bin is in your launcher Linux PATH.\n' "$base"
if [ "${1:-}" != "--no-setup" ]; then
  "$base/bin/miilo-owl" setup
fi
