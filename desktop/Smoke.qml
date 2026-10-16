import QtQuick
import Quickshell
import "." as Miilo

ShellRoot {
    Miilo.Dashboard {
        id: window
        visible: true
        Timer {
            interval: 2500
            running: true
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
