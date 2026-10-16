import QtQuick
import Quickshell
import Quickshell.Io
import qs.Ui as Ui

Ui.BarWidget {
    id: root
    moduleName: "io.miilo.bar"
    readonly property bool opened: dashboard.opened
    readonly property bool popoutSwitchClosing: false
    property var observation: ({})
    readonly property int attentionCount: observation.snapshot && observation.snapshot.attention ? observation.snapshot.attention.length : 0
    readonly property bool unavailable: observation.status === "partial" || observation.status === "failed"
    function open() { dashboard.open() }
    function close() { dashboard.close() }
    function toggle() { dashboard.toggle() }
    function closeForPopoutSwitch() { close() }
    implicitWidth: button.implicitWidth
    implicitHeight: button.implicitHeight
    Dashboard { id: dashboard }
    Ui.WidgetButton {
        id: button
        anchors.fill: parent
        bar: root.bar
        text: root.unavailable ? "Miilo !" : root.attentionCount ? "Miilo · " + root.attentionCount : "Miilo"
        tooltipText: root.unavailable ? "Miilo: check connection" : "Open Miilo"
        onPressed: (code) => { if (code === Qt.LeftButton) root.toggle() }
    }
    Process {
        id: fetch
        command: ["miilo-owl", "snapshot"]
        running: true
        stdout: StdioCollector {
            onStreamFinished: {
                try { root.observation = JSON.parse(text) }
                catch (error) { root.observation = { status: "failed" } }
            }
        }
        onExited: (code) => { if (code !== 0) root.observation = { status: "failed" } }
    }
    Timer { interval: 15000; running: root.visible; repeat: true; onTriggered: if (!fetch.running) fetch.running = true }
}
