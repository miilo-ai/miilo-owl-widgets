#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
report=$(mktemp /tmp/miilo-keyboard.XXXXXX)
config=$(mktemp /tmp/miilo-keyboard-shell.XXXXXX.qml)
trap 'rm -f "$report" "$config"' EXIT
cat > "$config" <<QML
import Quickshell
import "file:$root/tests/desktop" as Tests
ShellRoot {
    FloatingWindow {
        implicitWidth: 900
        implicitHeight: 800
        Tests.TestKeyboard { anchors.fill: parent }
    }
}
QML
if ! QT_QPA_PLATFORM=offscreen QT_QUICK_BACKEND=software QT_QPA_PLATFORMTHEME=basic \
  timeout 20s quickshell -p "$config" --no-color > "$report" 2>&1; then
  cat "$report"
  exit 1
fi
cat "$report"
rg -q 'MIILO_KEYBOARD_OK' "$report"
! rg -q 'MIILO_KEYBOARD_FAILED|TypeError|ReferenceError|Cannot assign' "$report"
