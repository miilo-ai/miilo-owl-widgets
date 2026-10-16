import QtQuick
import QtQuick.Controls
import Quickshell
import Quickshell.Io

FloatingWindow {
    id: root
    title: "Miilo"
    implicitWidth: 720
    implicitHeight: 760
    property alias surface: surface
    property var localAgents: []
    readonly property string detectedNames: localAgents.filter(a => a.found).map(a => a.name).join(" · ")
    Process {
        command: ["miilo-owl", "local-agents"]
        running: true
        stdout: StdioCollector {
            onStreamFinished: {
                try {
                    root.localAgents = JSON.parse(text).agents || [];
                } catch (error) {
                    root.localAgents = [];
                }
            }
        }
    }
    Item {
        anchors.fill: parent
        Rectangle {
            id: header
            anchors {
                top: parent.top
                left: parent.left
                right: parent.right
            }
            height: 76
            color: surface.widget.ink("background")
            Text {
                anchors {
                    left: parent.left
                    leftMargin: 16
                    verticalCenter: parent.verticalCenter
                }
                text: "Miilo"
                color: surface.widget.ink("foreground")
                font.pixelSize: 18
            }
            Button {
                id: connectButton
                anchors {
                    right: parent.right
                    rightMargin: 12
                    verticalCenter: parent.verticalCenter
                }
                text: "Connect local agents"
                implicitHeight: 36
                horizontalPadding: 12
                onClicked: Quickshell.execDetached(["xdg-terminal-exec", "miilo-local", "setup"])
                flat: true
                background: Rectangle {
                    color: connectButton.hovered ? surface.widget.ink("lighter_background") : "transparent"
                    border.color: surface.widget.ink("muted")
                    border.width: 1
                    radius: 0
                }
                contentItem: Text {
                    text: connectButton.text
                    color: surface.widget.ink("foreground")
                    font.pixelSize: 13
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }
            Text {
                anchors {
                    left: parent.left
                    right: connectButton.left
                    bottom: parent.bottom
                    leftMargin: 16
                    rightMargin: 12
                    bottomMargin: 8
                }
                text: root.detectedNames ? "Found locally: " + root.detectedNames : "Connect existing sessions on this computer"
                color: surface.widget.ink("foreground")
                font.pixelSize: 11
                elide: Text.ElideRight
            }
        }
        MiiloSurface {
            id: surface
            anchors {
                top: header.bottom
                bottom: parent.bottom
                left: parent.left
                right: parent.right
            }
            active: root.visible
            expanded: true
            focus: true
            Keys.onEscapePressed: root.visible = false
        }
    }
    onVisibleChanged: if (visible && surface.widget.program)
        surface.widget.openScreen()
}
