import QtQuick
import qs.Commons
import ".." as Miilo

Item {
    id: root
    property bool opened: false
    function open() { opened = true }
    function close() { opened = false }
    function toggle() { opened = !opened }
    Miilo.Dashboard {
        id: dashboard
        textScale: Style.fontScale
        textFamily: Style.font.family
        interactionTheme: Miilo.InteractionTheme {
            widget: dashboard.surface.widget
            foreground: Color.foreground
            accent: Color.accent
            border: Style.normalBorderColor
            hoverFill: Style.hoverFill
            focusFill: Color.menu.selectedBackground
            focusText: Color.menu.selectedText
            selectedFill: Style.selectedFill
            pressedFill: Style.pressedFill
            selectionFill: Style.selectionFillFor(Color.foreground, Color.accent)
        }
        visible: root.opened
        onVisibleChanged: if (!visible) root.opened = false
    }
}
