import QtQuick
import ".." as Miilo

Item {
    id: root
    property bool opened: false
    function open() { opened = true }
    function close() { opened = false }
    function toggle() { opened = !opened }
    Miilo.Dashboard {
        visible: root.opened
        onVisibleChanged: if (!visible) root.opened = false
    }
}
