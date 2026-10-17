#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
docker build --platform linux/arm64 -t miilo-owl-builder:1.3 -f "$root/tools/Dockerfile.helper" "$root/tools"
bash "$root/tools/run-builder.sh" cargo build --locked --manifest-path helper/Cargo.toml --release --target aarch64-unknown-linux-musl
mkdir -p "$root/helper/target/aarch64-unknown-linux-musl/release"
cp "$root/helper/target-linux/aarch64-unknown-linux-musl/release/miilo-owl" \
  "$root/helper/target/aarch64-unknown-linux-musl/release/miilo-owl"
file "$root/helper/target/aarch64-unknown-linux-musl/release/miilo-owl"
