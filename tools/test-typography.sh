#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
scratch=$(mktemp -d /tmp/miilo-typography.XXXXXX)
trap 'rm -rf -- "$scratch"' EXIT
mkdir "$scratch/runtime"
chmod 700 "$scratch/runtime"
cat > "$scratch/shell.qml" <<QML
import Quickshell
import "file:$root/tests/desktop" as Tests
ShellRoot {
    FloatingWindow {
        implicitWidth: 900
        implicitHeight: 800
        Tests.TestTypography {}
    }
}
QML
# An isolated, offscreen test harness, never a second live desktop shell.
if ! env -u WAYLAND_DISPLAY XDG_RUNTIME_DIR="$scratch/runtime" \
    QT_QPA_PLATFORM=offscreen QT_QUICK_BACKEND=software \
    QT_QPA_PLATFORMTHEME=basic QT_QUICK_CONTROLS_STYLE=Basic \
    timeout 25s quickshell -p "$scratch/shell.qml" --no-color > "$scratch/result" 2>&1; then
    cat "$scratch/result"
    exit 1
fi
cat "$scratch/result"
rg -q 'MIILO_TYPOGRAPHY_OK' "$scratch/result"
! rg -q 'MIILO_TYPOGRAPHY_FAILED|TypeError|ReferenceError|Cannot assign|Binding loop|Failed to load configuration' "$scratch/result"
