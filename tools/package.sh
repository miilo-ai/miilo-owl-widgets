#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$root/dist"
tar --exclude='__pycache__' --exclude='*.pyc' -czf "$root/dist/miilo-owl-widgets-1.2.tar.gz" -C "$root" \
  70-miilo-fleet.owl 71-miilo-attention.owl 72-miilo-board.owl install.sh helper widgets tools tests README.md Makefile
printf '%s\n' "$root/dist/miilo-owl-widgets-1.2.tar.gz"
