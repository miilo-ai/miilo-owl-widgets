import QtQuick

Text {
    required property var typography
    property string role: "body"
    font.family: typography.family
    font.pixelSize: typography.size(role)
    lineHeightMode: Text.ProportionalHeight
    lineHeight: typography.leading(role)
    textFormat: Text.PlainText
    wrapMode: Text.Wrap
}
