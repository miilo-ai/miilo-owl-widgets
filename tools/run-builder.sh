#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
access=()
if [[ -n "${GIT_CONFIG_GLOBAL:-}" ]]; then
  [[ -f "$GIT_CONFIG_GLOBAL" ]] || { echo 'SDK access configuration is missing' >&2; exit 1; }
  access+=(--mount "type=bind,source=$GIT_CONFIG_GLOBAL,target=/run/sdk.gitconfig,readonly"
    -e GIT_CONFIG_GLOBAL=/run/sdk.gitconfig -e CARGO_NET_GIT_FETCH_WITH_CLI=true)
fi
docker run --rm --platform linux/arm64 --cpus 2 --memory 2g \
  -v "$root:/repo" -w /repo -v miilo-owl-cargo:/usr/local/cargo/registry \
  -e CARGO_TARGET_DIR=/repo/helper/target-linux "${access[@]}" miilo-owl-builder:1.3 "$@"
