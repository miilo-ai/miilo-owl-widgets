import Quickshell

ShellRoot {
    Dashboard {
        visible: true
        onVisibleChanged: if (!visible) Qt.quit()
    }
}
