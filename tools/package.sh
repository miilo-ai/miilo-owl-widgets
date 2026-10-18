#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$root/helper/Cargo.toml" | head -1)
binary="${MIILO_OWL_BINARY:-$root/helper/target/aarch64-unknown-linux-musl/release/miilo-owl}"
[ -f "$binary" ] || { echo 'Build the ARM64 Linux helper first: make build-linux' >&2; exit 1; }
file "$binary" | grep -q 'ELF 64-bit.*ARM aarch64.*statically linked' || {
  echo 'Release packaging requires a static ARM64 Linux binary.' >&2; exit 1;
}
stage="$(mktemp -d)"
trap 'rm -rf -- "$stage"' EXIT
mkdir -p "$stage/bin/linux-aarch64" "$stage/tools" "$stage/docs" "$stage/previews" "$root/dist"
cp "$binary" "$stage/bin/linux-aarch64/miilo-owl"
chmod 755 "$stage/bin/linux-aarch64/miilo-owl"
cp "$root"/*.owl "$root/install.sh" "$root/README.md" "$root/LICENSE" "$stage/"
cp "$root/tools/check-device.sh" "$stage/tools/"
cp "$root/docs/native-helper-validation.md" "$root/docs/uninstall.md" "$root/docs/uninstall-validation.md" "$stage/docs/"
cp "$root/previews/native-rust-helper.png" "$root/previews/native-rust-onboarding.png" "$stage/previews/"
COPYFILE_DISABLE=1 tar --format=ustar -czf "$root/dist/miilo-owl-widgets-${version}-linux-aarch64.tar.gz" -C "$stage" .
(cd "$root/dist" && shasum -a 256 miilo-owl-widgets-${version}-linux-aarch64.tar.gz > SHA256SUMS)
printf '%s\n' "$root/dist/miilo-owl-widgets-${version}-linux-aarch64.tar.gz"
