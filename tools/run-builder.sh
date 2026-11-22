#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
source "$root/tools/linux-target.sh"
source_root="${MIILO_OWL_SOURCE_ROOT:-$root}"
access=()
if [[ -n "${GIT_CONFIG_GLOBAL:-}" ]]; then
  [[ -f "$GIT_CONFIG_GLOBAL" ]] || { echo 'SDK access configuration is missing' >&2; exit 1; }
  access+=(--mount "type=bind,source=$GIT_CONFIG_GLOBAL,target=/run/sdk.gitconfig,readonly"
    -e GIT_CONFIG_GLOBAL=/run/sdk.gitconfig -e CARGO_NET_GIT_FETCH_WITH_CLI=true)
fi
docker run --rm --platform "$linux_platform" --cpus 2 --memory 2g \
  -v "$source_root:/repo" -w /repo -v "miilo-owl-cargo-$linux_arch":/usr/local/cargo/registry \
  -e CARGO_TARGET_DIR=/repo/helper/target-linux ${access[@]+"${access[@]}"} "$linux_builder" "$@"
