#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
source "$root/tools/linux-target.sh"
bash "$root/tools/run-builder.sh" cargo test --locked --manifest-path helper/Cargo.toml --target "$linux_target" -j 2
