#!/usr/bin/env bash
# Helper-only archive for native desktops; never bundles a legacy mobile installer.
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
source "$root/tools/linux-target.sh"
src="${MIILO_OWL_SOURCE_ROOT:-$root}"
version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$src/helper/Cargo.toml" | head -1)
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([.-][A-Za-z0-9.-]+)?$ ]] || exit 1
binary="$src/helper/target/$linux_target/release/miilo-owl"
file "$binary" | grep -Eq "ELF 64-bit.*$linux_machine.*(statically linked|static-pie linked)" || {
  echo 'Expected matching static native Linux helper' >&2; exit 1;
}
commit=$(git -C "$src" rev-parse HEAD)
stage=$(mktemp -d)
trap 'rm -rf -- "$stage"' EXIT
out="${MIILO_NATIVE_OUTPUT:-$root/dist-native}"
mkdir -p "$out"
cp "$binary" "$stage/miilo-owl"
cp "$src/LICENSE" "$stage/LICENSE"
printf 'version=%s\nsource_commit=%s\ntarget=%s\n' "$version" "$commit" "$linux_target" > "$stage/BUILD-INFO"
cat > "$stage/INSTALL.md" <<'INSTALL'
# Native Linux helper

Verify this archive using its accompanying architecture-specific SHA256SUMS file.
Extract it, then install the helper only:

    install -Dm755 miilo-owl "$HOME/.local/bin/miilo-owl"
    "$HOME/.local/bin/miilo-owl" --version

Ensure ~/.local/bin is on the Omarchy shell PATH. This installs no widgets,
collectors, credentials or connections. The Miilo One plugin is installed separately.
INSTALL
name="miilo-owl-$version-linux-$linux_arch.tar.gz"
COPYFILE_DISABLE=1 tar --format=ustar -czf "$out/$name" -C "$stage" .
(cd "$out" && shasum -a 256 "$name" > "SHA256SUMS-native-$linux_arch")
printf '%s\n' "$out/$name"
