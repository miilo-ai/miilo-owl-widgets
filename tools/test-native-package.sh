#!/usr/bin/env bash
# Execute the shipped binary in an empty, non-root container with no network.
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
source "$root/tools/linux-target.sh"
src="${MIILO_OWL_SOURCE_ROOT:-$root}"
out="${MIILO_NATIVE_OUTPUT:-$root/dist-native}"
version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$src/helper/Cargo.toml" | head -1)
(cd "$out" && shasum -a 256 -c "SHA256SUMS-native-$linux_arch")
stage=$(mktemp -d)
image="miilo-native-smoke:$linux_arch"
trap 'rm -rf -- "$stage"' EXIT
tar -xzf "$out/miilo-owl-$version-linux-$linux_arch.tar.gz" -C "$stage"
grep -Fx "source_commit=$(git -C "$src" rev-parse HEAD)" "$stage/BUILD-INFO"
cat > "$stage/Dockerfile" <<'DOCKER'
FROM scratch
COPY miilo-owl /miilo-owl
USER 65532:65532
ENV HOME=/tmp
ENTRYPOINT ["/miilo-owl"]
DOCKER
docker build --platform "$linux_platform" -t "$image" "$stage" >/dev/null
result=$(docker run --rm --platform "$linux_platform" --network none --read-only --cap-drop ALL --security-opt no-new-privileges:true "$image" --version)
test "$result" = "miilo-owl $version"
docker run --rm --platform "$linux_platform" --network none --read-only --cap-drop ALL --security-opt no-new-privileges:true "$image" --help >/dev/null
printf '%s\n' 'Native package runs without a system loader, extra libraries, root or network.'
