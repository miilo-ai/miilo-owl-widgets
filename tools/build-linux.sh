#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
docker build --platform linux/arm64 -t miilo-owl-builder:1.3 -f "$root/tools/Dockerfile.helper" "$root/tools"
docker run --rm --platform linux/arm64 --cpus 2 --memory 2g \
  -v "$root/helper:/src" -v miilo-owl-cargo:/usr/local/cargo/registry \
  -e CARGO_TARGET_DIR=/src/target-linux miilo-owl-builder:1.3 \
  cargo build --locked --release --target aarch64-unknown-linux-musl
mkdir -p "$root/helper/target/aarch64-unknown-linux-musl/release"
cp "$root/helper/target-linux/aarch64-unknown-linux-musl/release/miilo-owl" \
  "$root/helper/target/aarch64-unknown-linux-musl/release/miilo-owl"
file "$root/helper/target/aarch64-unknown-linux-musl/release/miilo-owl"
