import QtQuick
import Quickshell
import "." as Miilo

ShellRoot {
    Miilo.Dashboard {
        id: window
        visible: true
        property int importStage: 0
        Timer {
            interval: 100
            repeat: true
            running: Quickshell.env("MIILO_IMPORT_UI_TEST") === "1"
            onTriggered: {
                if (window.importStage === 0 && window.importer.mode === "invite") {
                    if (window.importedCount !== 0) { console.error("MIILO_IMPORT_UI_FAILED: imported without consent"); Qt.quit(); return }
                    window.importStage = 1
                    window.importer.review()
                } else if (window.importStage === 1 && window.importer.mode === "review") {
                    if (window.importer.sessions.length !== 1 || window.importer.selected.length !== 0) { console.error("MIILO_IMPORT_UI_FAILED: invalid review state"); Qt.quit(); return }
                    window.importer.toggle(window.importer.sessions[0])
                    window.importStage = 2
                    window.importer.importSelected()
                } else if (window.importStage === 2 && window.importedCount === 1 && window.importer.mode === "") {
                    console.log("MIILO_IMPORT_UI_OK")
                    Qt.quit()
                }
            }
        }
        Timer {
            interval: 2500
            running: Quickshell.env("MIILO_IMPORT_UI_TEST") !== "1"
            onTriggered: {
                var widget = window.surface.widget
                if (widget.error || widget.problems.length || !widget.screenTree) {
                    console.error("MIILO_SMOKE_FAILED", widget.error, JSON.stringify(widget.problems))
                    Qt.quit()
                    return
                }
                window.surface.parent.grabToImage(function(result) {
                    var saved = result.saveToFile(Quickshell.env("MIILO_SMOKE_PNG") || "/tmp/miilo-desktop.png")
                    console.log(saved ? "MIILO_SMOKE_OK" : "MIILO_SMOKE_FAILED")
                    Qt.quit()
                })
            }
        }
    }
}
