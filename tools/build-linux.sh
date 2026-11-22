#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
source "$root/tools/linux-target.sh"
source_root="${MIILO_OWL_SOURCE_ROOT:-$root}"
docker build --platform "$linux_platform" --build-arg "RUST_TARGET=$linux_target" \
  -t "$linux_builder" -f "$root/tools/Dockerfile.helper" "$root/tools"
bash "$root/tools/run-builder.sh" cargo build --locked --manifest-path helper/Cargo.toml --release --target "$linux_target"
mkdir -p "$source_root/helper/target/$linux_target/release"
cp "$source_root/helper/target-linux/$linux_target/release/miilo-owl" \
  "$source_root/helper/target/$linux_target/release/miilo-owl"
file "$source_root/helper/target/$linux_target/release/miilo-owl"
