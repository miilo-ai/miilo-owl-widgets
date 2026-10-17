import QtQuick

// Portable defaults; the Omarchy host supplies its live shell tokens.
QtObject {
    id: root
    property var widget
    function tint(name, opacity) {
        var color = Qt.color(widget.ink(name));
        return Qt.rgba(color.r, color.g, color.b, opacity);
    }
    property color foreground: widget.ink("foreground")
    property color accent: widget.ink("accent")
    property color border: tint("foreground", 0.22)
    property color hoverFill: tint("foreground", 0.06)
    property color focusFill: tint("foreground", 0.1)
    property color focusText: accent
    property color selectedFill: tint("accent", 0.12)
    property color pressedFill: tint("accent", 0.2)
    property color selectionFill: widget.ink("selection")
}
