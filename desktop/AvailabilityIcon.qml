import QtQuick

Canvas {
    id: root
    required property bool available
    required property color ink
    property real size: 24
    implicitWidth: size
    implicitHeight: size
    Accessible.role: Accessible.Graphic
    Accessible.name: available ? "Import supported" : "Import not available yet"
    onAvailableChanged: requestPaint()
    onInkChanged: requestPaint()
    onWidthChanged: requestPaint()
    onHeightChanged: requestPaint()
    onPaint: {
        var ctx = getContext("2d");
        ctx.clearRect(0, 0, width, height);
        ctx.strokeStyle = root.ink;
        ctx.lineWidth = Math.max(1.5, width / 14);
        ctx.lineCap = "round";
        ctx.lineJoin = "round";
        var cx = width / 2, cy = height / 2, radius = Math.min(width, height) * 0.36;
        ctx.beginPath(); ctx.arc(cx, cy, radius, 0, Math.PI * 2); ctx.stroke();
        ctx.beginPath();
        if (root.available) {
            ctx.moveTo(cx - radius * 0.5, cy);
            ctx.lineTo(cx - radius * 0.1, cy + radius * 0.4);
            ctx.lineTo(cx + radius * 0.55, cy - radius * 0.4);
        } else {
            ctx.moveTo(cx - radius * 0.7, cy + radius * 0.7);
            ctx.lineTo(cx + radius * 0.7, cy - radius * 0.7);
        }
        ctx.stroke();
    }
}
