import QtQuick
import Quickshell
import Quickshell.Wayland
import ".." as Miilo

Item {
    id: root
    property bool opened: false
    function open() { opened = true }
    function close() { opened = false }
    function toggle() { opened = !opened }
    PanelWindow {
        id: panel
        visible: root.opened
        anchors { bottom: true; right: true }
        margins { bottom: 24; right: 24 }
        implicitWidth: screen ? Math.max(1, Math.round((screen.width - margins.right) * (surface.expanded ? 0.45 : 0.18))) : 0
        implicitHeight: screen ? Math.max(1, Math.round((screen.height - margins.bottom) * (surface.expanded ? 0.8 : 0.2))) : 0
        exclusionMode: ExclusionMode.Ignore
        WlrLayershell.layer: WlrLayer.Bottom
        WlrLayershell.keyboardFocus: surface.expanded ? WlrKeyboardFocus.OnDemand : WlrKeyboardFocus.None
        Miilo.MiiloSurface {
            id: surface
            anchors.fill: parent
            active: root.opened
            focus: true
            Keys.onEscapePressed: closeDetails()
        }
    }
}
