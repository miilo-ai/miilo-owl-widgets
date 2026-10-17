import QtQuick
import "MiiloTypeScale.js" as TypeScale

QtObject {
    property string profile: "desktop"
    property real textScale: 1
    readonly property real effectiveScale: TypeScale.validScale(textScale)
    property string family: Qt.application.font.family
    readonly property int touchTarget: profile === "phone" ? 48 : 40
    // Dense desktop controls; phone controls retain their interaction minimum.
    function controlHeight(contentHeight, verticalSpace) {
        return Math.ceil(Math.max(profile === "phone" ? touchTarget : 0, contentHeight + verticalSpace))
    }
    function size(role) { return TypeScale.size(role, profile, textScale) }
    function leading(role) { return TypeScale.leading(role) }
}
