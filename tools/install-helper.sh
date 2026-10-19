#!/usr/bin/env bash
# User-invoked bootstrap; never enrolls sessions or changes credentials.
set -euo pipefail
version='1.6.1'
[ "$(uname -s)" = Linux ] && [ "$(uname -m)" = aarch64 ] || {
  echo 'This installer supports ARM64 Linux inside Omalauncher. See the installation guide for other systems.' >&2
  exit 1
}
for program in curl sha256sum tar mktemp install; do
  command -v "$program" >/dev/null || { echo "Required program missing: $program" >&2; exit 1; }
done
base="$HOME/.local/bin"
if [ -e "$base/miilo-owl" ]; then
  echo 'An existing helper was found; it has not been replaced. Use the documented upgrade workflow.'
  exit 0
fi
stage=$(mktemp -d)
trap 'rm -rf -- "$stage"' EXIT
asset="miilo-owl-widgets-$version-linux-aarch64.tar.gz"
url="https://github.com/miilo-ai/miilo-owl-widgets/releases/download/v$version"
curl --fail --silent --show-error --location --proto '=https' --proto-redir '=https' --connect-timeout 15 --max-time 180 "$url/$asset" -o "$stage/$asset" || { echo "Miilo $version could not be downloaded. Check your connection and whether this release is published; no helper was installed." >&2; exit 1; }
curl --fail --silent --show-error --location --proto '=https' --proto-redir '=https' --connect-timeout 15 --max-time 30 "$url/SHA256SUMS" -o "$stage/SHA256SUMS" || { echo "Release checksums are unavailable; no helper was installed." >&2; exit 1; }
expected=$(awk -v name="$asset" '$2 == name { print $1 }' "$stage/SHA256SUMS")
[[ "$expected" =~ ^[a-fA-F0-9]{64}$ ]] || { echo 'Missing or ambiguous release checksum.' >&2; exit 1; }
printf '%s  %s\n' "$expected" "$asset" > "$stage/verified.sha256"
(cd "$stage" && sha256sum --check verified.sha256)
# Extract one regular member to stdout; never unpack archive paths onto disk.
member='./bin/linux-aarch64/miilo-owl'
[ "$(tar -tzf "$stage/$asset" | grep -Fxc "$member")" = 1 ] || { echo 'Unexpected release layout.' >&2; exit 1; }
tar -xOzf "$stage/$asset" "$member" > "$stage/miilo-owl"
chmod 755 "$stage/miilo-owl"
[ "$("$stage/miilo-owl" --version)" = "miilo-owl $version" ] || { echo 'Unexpected helper version.' >&2; exit 1; }
mkdir -p "$base"
# Hard link publishes the staged copy without overwriting a concurrent install.
pending=$(mktemp "$base/.miilo-owl.XXXXXX")
trap 'rm -rf -- "$stage"; rm -f -- "$pending"' EXIT
install -m 755 "$stage/miilo-owl" "$pending"
ln "$pending" "$base/miilo-owl"
printf 'Installed miilo-owl %s in %s. Return to Miilo and refresh to continue.\n' "$version" "$base"
