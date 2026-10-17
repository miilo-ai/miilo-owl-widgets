import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell

FloatingWindow {
    id: root
    title: "Miilo"
    property real textScale: 1
    property string textFamily: Qt.application.font.family
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
            height: headerContent.implicitHeight + 16
            color: surface.widget.ink("background")
            GridLayout {
                id: headerContent
                anchors { left: parent.left; right: parent.right; top: parent.top; margins: 8 }
                columns: width < 480 * root.interactionTheme.typography.effectiveScale ? 1 : 2
                TypeText {
                typography: root.interactionTheme.typography
                role: "title"
                Layout.fillWidth: true
                text: "Miilo"
                color: surface.widget.ink("foreground")
            }
            Button {
                id: localButton
                Layout.minimumWidth: 0
                Layout.fillWidth: headerContent.columns === 1
                text: root.width < 480 ? "Local agents" : "Local agents · Ctrl+L"
                focusPolicy: Qt.StrongFocus
                Keys.onReturnPressed: clicked()
                implicitHeight: Math.max(root.interactionTheme.typography.touchTarget, contentItem.implicitHeight + 16)
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
                contentItem: TypeText {
                    typography: root.interactionTheme.typography
                    role: "label"
                    text: localButton.text
                    color: localButton.activeFocus ? root.interactionTheme.focusText : root.interactionTheme.foreground
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
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
            hostTextScale: root.textScale
            hostFontFamily: root.textFamily
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
