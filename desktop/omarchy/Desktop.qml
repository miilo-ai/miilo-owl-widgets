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
        visible: root.opened
        anchors { bottom: true; right: true }
        margins { bottom: 24; right: 24 }
        implicitWidth: surface.expanded ? 620 : 260
        implicitHeight: surface.expanded ? 720 : 180
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
