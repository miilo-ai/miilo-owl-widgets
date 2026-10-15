#!/usr/bin/env bash
# Run inside Omalauncher's Linux environment; no network install or sudo.
set -euo pipefail
command -v python3 >/dev/null || { echo 'Install Python first: sudo pacman -S python' >&2; exit 1; }
repo="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
base="${MIILO_OWL_INSTALL_ROOT:-$HOME/.local}"
widgets="${MIILO_OWL_WIDGET_DIR:-$HOME/.config/omalauncher/widgets}"
mkdir -p "$base/bin" "$base/share/miilo-owl" "$widgets"
cp -R "$repo/helper/miilo_owl" "$base/share/miilo-owl/"
python3 - "$base" <<'PY'
from pathlib import Path
import shlex
import sys
base = Path(sys.argv[1]).resolve()
launcher = base/'bin/miilo-owl'
launcher.write_text('#!/bin/sh\nPYTHONPATH='+shlex.quote(str(base/'share/miilo-owl'))+' exec python3 -m miilo_owl "$@"\n')
launcher.chmod(0o755)
PY
for file in "$repo"/*.owl; do
  name="$(basename -- "$file")"
  if [ -f "$widgets/$name" ]; then
    backup="$widgets/$name.before-miilo-1.2"
    [ -e "$backup" ] || cp "$widgets/$name" "$backup"
  fi
  cp "$file" "$widgets/$name"
done
printf 'Installed widgets and helper in %s\n' "$base"
printf 'Ensure %s/bin is in your launcher Linux PATH.\n' "$base"
if [ "${1:-}" != "--no-setup" ]; then
  "$base/bin/miilo-owl" setup
fi
