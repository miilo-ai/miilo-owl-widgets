#!/usr/bin/env bash
# Shared native Linux release target. Keep ARM64 as the existing mobile default.
case "${MIILO_LINUX_ARCH:-aarch64}" in
  aarch64) linux_arch=aarch64; linux_platform=linux/arm64; linux_machine='ARM aarch64' ;;
  x86_64) linux_arch=x86_64; linux_platform=linux/amd64; linux_machine='x86-64' ;;
  *) echo 'MIILO_LINUX_ARCH must be aarch64 or x86_64' >&2; exit 1 ;;
esac
linux_target="$linux_arch-unknown-linux-musl"
linux_builder="miilo-owl-builder:linux-$linux_arch-v2"
