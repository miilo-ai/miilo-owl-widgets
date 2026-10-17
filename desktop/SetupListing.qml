pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell

ColumnLayout {
    id: root
    required property var agents
    required property var widget
    required property var theme
    spacing: 12
    function displayPaths(locations) {
        var home = Quickshell.env("HOME") || "";
        return (locations || []).map(function(path) {
            return home && (path === home || path.indexOf(home + "/") === 0)
                ? "~" + path.slice(home.length) : path;
        }).join(" · ") || "Installed command found";
    }
    Repeater {
        model: root.agents
        delegate: GridLayout {
            id: setup
            required property var modelData
            readonly property bool available: modelData.observationSupported === true
            readonly property bool inlinePath: width >= 480 * root.theme.typography.effectiveScale
            Layout.fillWidth: true
            Layout.minimumWidth: 0
            columns: inlinePath ? 3 : 2
            columnSpacing: 8
            rowSpacing: 4
            AvailabilityIcon {
                objectName: "setupStatus"
                Layout.column: 0; Layout.row: 0
                Layout.rowSpan: setup.inlinePath ? 1 : 2
                Layout.alignment: Qt.AlignTop
                available: setup.available
                ink: setup.available ? root.theme.accent : root.widget.ink("muted")
                size: Math.max(24, root.theme.typography.size("body") + 8)
            }
            TypeText {
                typography: root.theme.typography
                Layout.column: 1; Layout.row: 0
                Layout.fillWidth: !setup.inlinePath
                Layout.minimumWidth: 0
                text: setup.modelData.name + (setup.available ? "" : " · Not available yet")
                color: setup.available ? root.theme.foreground : root.widget.ink("muted")
            }
            TypeText {
                typography: root.theme.typography
                objectName: "setupPath"
                Layout.column: setup.inlinePath ? 2 : 1
                Layout.row: setup.inlinePath ? 0 : 1
                Layout.fillWidth: true
                Layout.minimumWidth: 0
                text: root.displayPaths(setup.modelData.locations)
                role: "small"
                color: setup.available ? root.theme.pathText : root.widget.ink("muted")
                wrapMode: Text.WrapAnywhere
            }
        }
    }
}
