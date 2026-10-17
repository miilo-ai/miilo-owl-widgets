import QtQuick
import "MiiloTypeScale.js" as TypeScale

QtObject {
    property string profile: "desktop"
    property real textScale: 1
    readonly property real effectiveScale: TypeScale.validScale(textScale)
    property string family: Qt.application.font.family
    readonly property int touchTarget: profile === "phone" ? 48 : 40
    function size(role) { return TypeScale.size(role, profile, textScale) }
    function leading(role) { return TypeScale.leading(role) }
}
