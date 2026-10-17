#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
bash "$root/tools/run-builder.sh" cargo test --locked --manifest-path helper/Cargo.toml --target aarch64-unknown-linux-musl -j 2
