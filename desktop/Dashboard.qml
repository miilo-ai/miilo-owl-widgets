import QtQuick
import QtQuick.Controls
import Quickshell

FloatingWindow {
    id: root
    title: "Miilo"
    // Initial floating size only; the compositor owns the actual viewport.
    implicitWidth: screen ? Math.round(screen.width * 0.65) : 0
    implicitHeight: screen ? Math.round(screen.height * 0.8) : 0
    property alias surface: surface
    property alias importer: importer
    property int importedCount: 0
    property QtObject interactionTheme: InteractionTheme { widget: surface.widget }
    Shortcut { sequence: "Ctrl+L"; enabled: root.visible && !importer.busy; onActivated: importer.review() }
    Shortcut { sequence: "Escape"; enabled: root.visible && !importer.visible; onActivated: root.visible = false }
    Item {
        anchors.fill: parent
        Rectangle {
            id: header
            anchors {
                top: parent.top
                left: parent.left
                right: parent.right
            }
            height: 56
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
                id: localButton
                anchors {
                    right: parent.right
                    rightMargin: 12
                    verticalCenter: parent.verticalCenter
                }
                text: "Local agents · Ctrl+L"
                focusPolicy: Qt.StrongFocus
                Keys.onReturnPressed: clicked()
                implicitHeight: 36
                horizontalPadding: 12
                onClicked: importer.review()
                enabled: !importer.busy
                flat: true
                background: Rectangle {
                    color: localButton.down ? root.interactionTheme.pressedFill : localButton.activeFocus ? root.interactionTheme.focusFill : localButton.hovered ? root.interactionTheme.hoverFill : "transparent"
                    border.color: root.interactionTheme.border
                    border.width: 1
                    radius: 0
                    Rectangle { anchors.bottom: parent.bottom; anchors.horizontalCenter: parent.horizontalCenter; width: parent.width - 16; height: 2; visible: localButton.activeFocus; color: root.interactionTheme.accent }
                }
                contentItem: Text {
                    text: localButton.text
                    color: localButton.activeFocus ? root.interactionTheme.focusText : root.interactionTheme.foreground
                    font.pixelSize: 13
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
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
            active: root.visible && !importer.visible
            visible: !importer.visible
            expanded: true
            configureLocal: () => importer.review()
            interactionTheme: root.interactionTheme
            focus: true
            Keys.onEscapePressed: root.visible = false
        }
        LocalImport {
            id: importer
            anchors.fill: surface
            widget: surface.widget
            theme: root.interactionTheme
            active: root.visible
            onVisibleChanged: if (!visible && root.visible) localButton.forceActiveFocus()
            onConnected: count => {
                root.importedCount = count;
                surface.widget.refresh();
            }
        }
    }
    onVisibleChanged: if (visible && surface.widget.program)
        surface.widget.openScreen()
}
