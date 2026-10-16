#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
binary="${MIILO_OWL_BINARY:-$root/helper/target/debug/miilo-owl}"
[[ -x "$binary" ]] || { echo 'Build miilo-owl first, or set MIILO_OWL_BINARY.' >&2; exit 1; }
case "${1:-portable}" in portable|omarchy) mode="${1:-portable}";; *) echo 'Usage: install-desktop.sh [portable|omarchy]' >&2; exit 2;; esac
exec "$binary" install-desktop "$root" "$mode"
