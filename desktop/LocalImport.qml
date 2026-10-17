pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell
import Quickshell.Io

FocusScope {
    id: root
    property var widget
    property QtObject theme: InteractionTheme { widget: root.widget }
    property bool active: true
    property string mode: ""
    property var agents: []
    property var sessions: []
    property var selected: []
    property var groups: []
    property var grouped: []
    property var rows: []
    property var collapsed: ({})
    property bool groupsLoaded: false
    property bool groupsDirty: false
    property int editIndex: -1
    property string fingerprint: ""
    property string project: Quickshell.env("HOME") || "/"
    property string error: ""
    property var reply: ({})
    readonly property bool busy: mode === "loading" || mode === "importing"
    signal connected(int count)
    visible: mode !== ""
    onModeChanged: Qt.callLater(() => {
        if (mode === "review") sessionList.forceActiveFocus();
        else if (mode === "group") groupName.forceActiveFocus();
        else if (mode === "invite") reviewButton.forceActiveFocus();
    })
    Keys.onEscapePressed: event => {
        if (!busy) { if (mode === "group") mode = "review"; else dismiss(); }
        event.accepted = true;
    }
    Shortcut { sequence: "Ctrl+G"; enabled: root.visible && root.mode === "review"; onActivated: root.editGroup(-1) }
    Shortcut { sequence: "Ctrl+R"; enabled: root.visible && root.mode === "review"; onActivated: root.review() }
    Shortcut { sequence: "Ctrl+Return"; enabled: root.visible && root.mode === "review" && (root.selected.length > 0 || root.groupsDirty); onActivated: root.importSelected() }

    function rebuildRows() {
        var result = [];
        grouped.forEach((g, index) => {
            result.push({header:true, group:index, name:g.name, count:g.sessions.length});
            if (!collapsed[g.name]) g.sessions.forEach(s => result.push({header:false, group:index, session:s}));
        });
        rows = result;
    }
    function regroup() { if (!grouping.running) grouping.running = true; }
    function collapseGroup(index, value) {
        var next = Object.assign({}, collapsed);
        var name = grouped[index].name;
        next[name] = value === undefined ? !next[name] : value;
        collapsed = next;
        rebuildRows();
    }
    function toggleGroup(index) {
        var items = grouped[index].sessions.filter(s => s.observable);
        var all = items.length && items.every(s => selected.indexOf(s.kind + ":" + s.id) >= 0);
        var next = selected.slice();
        items.forEach(s => { var key = s.kind + ":" + s.id; var at = next.indexOf(key);
            if (all && at >= 0) next.splice(at,1); else if (!all && at < 0) next.push(key);
        });
        selected = next;
    }
    function activateRow() {
        var row = rows[sessionList.currentIndex];
        if (!row) return;
        if (row.header) toggleGroup(row.group);
        else if (row.session.observable) toggle(row.session);
    }
    function editGroup(index) {
        editIndex = index;
        var rule = index >= 0 ? groups[index] : {name:"",folder:project,pattern:"",kind:"all",future:false};
        groupName.text = rule.name; groupFolder.text = rule.folder;
        groupPattern.text = rule.pattern; groupKind.currentIndex = ["all","codex","cursor"].indexOf(rule.kind);
        futureMatches.checked = rule.future;
        error = ""; mode = "group";
    }
    function saveGroup() {
        var rule = {name:groupName.text.trim(),folder:groupFolder.text.trim(),pattern:groupPattern.text.trim(),kind:["all","codex","cursor"][groupKind.currentIndex],future:futureMatches.checked};
        var next = groups.slice();
        if (editIndex >= 0) next[editIndex] = rule; else next.push(rule);
        pendingGroups = next;
        regroup();
    }
    property var pendingGroups: null

    function review() {
        error = "";
        mode = "loading";
        if (!discovery.running)
            discovery.running = true;
    }
    function toggle(session) {
        var key = session.kind + ":" + session.id;
        selected = selected.indexOf(key) >= 0 ? selected.filter(k => k !== key) : selected.concat([key]);
    }
    function dismiss() {
        mode = "";
        groupsLoaded = false;
        groupsDirty = false;
        if (fingerprint && !dismissal.running)
            dismissal.running = true;
    }
    function importSelected() {
        if ((!selected.length && !groupsDirty) || importing.running)
            return;
        error = "";
        reply = ({});
        mode = "importing";
        importing.running = true;
    }
    onActiveChanged: if (active && !detection.running && !busy)
        detection.running = true
    Process {
        id: detection
        command: ["miilo-owl", "local-agents"]
        running: root.active
        stdout: StdioCollector {
            onStreamFinished: {
                try {
                    var result = JSON.parse(text);
                    root.agents = (result.agents || []).filter(a => a.found);
                    root.fingerprint = result.fingerprint || "";
                    if (root.agents.length && !result.dismissed && !root.busy)
                        root.mode = "invite";
                } catch (e) { /* An unavailable helper must not masquerade as detected agents. */ }
            }
        }
    }
    Process {
        id: dismissal
        command: ["miilo-owl", "dismiss-local", root.fingerprint]
    }
    Process {
        id: discovery
        command: [Quickshell.env("MIILO_LOCAL_EXECUTABLE") || "miilo-local", "discover", root.project]
        stdout: StdioCollector {
            onStreamFinished: {
                if (root.mode === "")
                    return;
                try {
                    var result = JSON.parse(text);
                    root.sessions = result.sessions || [];
                    if (!root.groupsLoaded) { root.groups = result.savedGroups || []; root.groupsLoaded = true; }
                    root.selected = [];
                    root.error = (result.errors || []).join(". ");
                    root.mode = "review";
                    root.regroup();
                } catch (e) {
                    root.mode = "review";
                    root.error = "Local session discovery is unavailable. Check that the Miilo local connector is installed.";
                }
            }
        }
        onExited: code => {
            if (code !== 0 && root.mode !== "") {
                root.mode = "review";
                root.error = "Could not list local sessions. Check the project directory and try again.";
            }
        }
    }
    Process {
        id: grouping
        command: [Quickshell.env("MIILO_LOCAL_EXECUTABLE") || "miilo-local", "groups"]
        stdinEnabled: true
        onStarted: write(JSON.stringify({sessions:root.sessions,groups:root.pendingGroups || root.groups}) + "\n")
        stdout: StdioCollector {
            onStreamFinished: {
                try {
                    var result = JSON.parse(text);
                    root.grouped = result.groups || [];
                    if (root.pendingGroups) { root.groups = root.pendingGroups; root.groupsDirty = true; root.mode = "review"; }
                    root.pendingGroups = null; root.error = ""; root.rebuildRows();
                } catch (e) { root.pendingGroups = null; root.error = "Could not preview these groups."; }
            }
        }
        stderr: StdioCollector { onStreamFinished: if (text.trim()) root.error = text.trim() }
        onExited: code => { if (code !== 0) { root.pendingGroups = null; if (!root.error) root.error = "Check the group name, folder, and pattern."; } }
    }
    Process {
        id: importing
        command: [Quickshell.env("MIILO_LOCAL_EXECUTABLE") || "miilo-local", "import"]
        stdinEnabled: true
        onStarted: {
            var sessions = root.sessions.filter(s => root.selected.indexOf(s.kind + ":" + s.id) >= 0).map(s => ({
                        kind: s.kind,
                        id: s.id
                    }));
            write(JSON.stringify({
                sessions: sessions,
                project: root.project
                ,groups: root.groups
            }) + "\n");
        }
        stdout: StdioCollector {
            onStreamFinished: {
                try {
                    root.reply = JSON.parse(text);
                } catch (e) {
                    root.reply = {
                        status: "failed",
                        message: "The local connector did not complete the import."
                    };
                }
            }
        }
        onExited: code => {
            if (code === 0 && root.reply.status === "connected") {
                root.dismiss();
                root.connected(root.reply.sessions || 0);
            } else {
                root.mode = "review";
                root.error = root.reply.message || "Import could not be completed. Your other Miilo connections were preserved.";
            }
        }
    }

    Rectangle {
        anchors.fill: parent
        color: root.widget.ink("background")
    }
    component Action: Button {
        id: control
        implicitHeight: 40
        horizontalPadding: 14
        flat: true
        focusPolicy: Qt.StrongFocus
        Keys.onReturnPressed: clicked()
        Keys.onEnterPressed: clicked()
        background: Rectangle {
            color: control.down ? root.theme.pressedFill : control.activeFocus ? root.theme.focusFill : control.hovered && control.enabled ? root.theme.hoverFill : "transparent"
            border.color: root.theme.border
            border.width: 1
            radius: 0
            opacity: control.enabled ? 1 : 0.5
            Rectangle { anchors.bottom: parent.bottom; anchors.horizontalCenter: parent.horizontalCenter; width: parent.width - 16; height: 2; visible: control.activeFocus; color: root.theme.accent }
        }
        contentItem: Text {
            text: control.text
            color: control.activeFocus ? root.theme.focusText : root.theme.foreground
            opacity: control.enabled ? 1 : 0.5
            font.pixelSize: 13
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }
    }
    component Field: TextField {
        id: field
        Layout.fillWidth: true
        color: root.widget.ink("foreground")
        placeholderTextColor: root.widget.ink("muted")
        selectionColor: root.theme.selectionFill
        selectedTextColor: root.widget.ink("foreground")
        selectByMouse: true
        background: Rectangle {
            color: field.activeFocus ? root.theme.focusFill : root.widget.ink("lighter_background")
            border.color: root.theme.border
            border.width: 1
            Rectangle { anchors.bottom: parent.bottom; anchors.horizontalCenter: parent.horizontalCenter; width: parent.width - 12; height: 2; visible: field.activeFocus; color: root.theme.accent }
        }
    }
    component ThemeScrollBar: ScrollBar {
        id: scroll
        contentItem: Rectangle {
            implicitWidth: 6
            implicitHeight: 6
            color: root.widget.ink(scroll.pressed ? "accent" : "muted")
            opacity: scroll.active ? 1 : 0.5
        }
        background: Rectangle { color: root.widget.ink("dark_background") }
    }
    component Choice: ComboBox {
        id: control
        Layout.fillWidth: true
        implicitHeight: 40
        focusPolicy: Qt.StrongFocus
        leftPadding: 12
        rightPadding: 32
        background: Rectangle {
            color: control.down ? root.theme.pressedFill : control.activeFocus ? root.theme.focusFill : control.hovered ? root.theme.hoverFill : root.widget.ink("lighter_background")
            border.color: root.theme.border
            border.width: 1
            Rectangle { anchors.bottom: parent.bottom; anchors.horizontalCenter: parent.horizontalCenter; width: parent.width - 16; height: 2; visible: control.activeFocus; color: root.theme.accent }
        }
        contentItem: Text {
            text: control.count ? control.displayText : "No saved groups"
            color: !control.enabled ? root.widget.ink("muted") : control.activeFocus ? root.theme.focusText : root.theme.foreground
            font.pixelSize: 13
            verticalAlignment: Text.AlignVCenter
            elide: Text.ElideRight
        }
        indicator: Text {
            x: control.width - width - 12
            anchors.verticalCenter: parent.verticalCenter
            text: control.popup.visible ? "▴" : "▾"
            color: root.widget.ink(control.enabled ? "foreground" : "muted")
        }
        delegate: ItemDelegate {
            id: option
            required property var modelData
            required property int index
            width: control.width
            implicitHeight: 40
            highlighted: control.highlightedIndex === index
            contentItem: Text {
                text: option.modelData
                color: option.highlighted ? root.theme.focusText : root.theme.foreground
                font.pixelSize: 13
                verticalAlignment: Text.AlignVCenter
                elide: Text.ElideRight
            }
            background: Rectangle {
                color: option.down ? root.theme.pressedFill : option.highlighted ? root.theme.focusFill : "transparent"
            }
        }
        popup: Popup {
            y: control.height
            width: control.width
            implicitHeight: Math.min(contentItem.implicitHeight + 8, root.height * 0.45)
            padding: 4
            contentItem: ListView {
                clip: true
                implicitHeight: contentHeight
                model: control.popup.visible ? control.delegateModel : null
                currentIndex: control.highlightedIndex
                ScrollBar.vertical: ThemeScrollBar {}
            }
            background: Rectangle {
                color: root.widget.ink("background")
                border.color: root.widget.ink("accent")
                border.width: 1
            }
        }
    }
    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 16
        Text {
            Layout.fillWidth: true
            text: root.mode === "invite" ? "I found your local agent setups" : root.mode === "importing" ? "Importing selected sessions…" : root.mode === "group" ? "Define an agent group" : "Import local agents"
            color: root.widget.ink("foreground")
            font.pixelSize: 22
            wrapMode: Text.WordWrap
        }
        Text {
            Layout.fillWidth: true
            text: root.mode === "invite" ? "Would you like to import them into Miilo?" : root.mode === "importing" ? "Starting local observation and saving your Miilo connection." : root.mode === "group" ? "Match projects within a folder. * matches any text; ? matches one character." : "↑/↓ navigate · Space selects · Enter folds groups · Ctrl+G adds a group"
            color: root.widget.ink("foreground")
            font.pixelSize: 14
            wrapMode: Text.WordWrap
        }
        ColumnLayout {
            visible: root.mode === "invite"
            Layout.fillWidth: true
            spacing: 14
            Repeater {
                model: root.agents
                delegate: ColumnLayout {
                    required property var modelData
                    Layout.fillWidth: true
                    spacing: 4
                    Text {
                        text: modelData.name + (modelData.observationSupported ? "" : " · import not available yet")
                        color: root.widget.ink("foreground")
                        font.pixelSize: 14
                    }
                    Text {
                        Layout.fillWidth: true
                        text: (modelData.locations || []).map(p => p.replace(Quickshell.env("HOME"), "~")).join(" · ") || "Installed command found"
                        color: root.widget.ink("muted")
                        font.pixelSize: 12
                        wrapMode: Text.WrapAnywhere
                    }
                }
            }
        }
        RowLayout {
            visible: root.mode === "review"
            Layout.fillWidth: true
            Field {
                text: root.project
                placeholderText: "Project directory"
                onTextEdited: root.project = text
                onAccepted: root.review()
            }
            Action {
                text: "Rescan"
                enabled: !root.busy
                onClicked: root.review()
            }
        }
        RowLayout {
            visible: root.mode === "review"
            Layout.fillWidth: true
            Action { text: "Add group"; onClicked: root.editGroup(-1) }
            Choice {
                id: existingGroup
                Layout.fillWidth: true
                model: root.groups.map(g => g.name + (g.future ? " · future matches on" : ""))
                enabled: root.groups.length > 0
                Accessible.name: "Saved group to edit"
            }
            Action { text: "Edit group"; enabled: root.groups.length > 0; onClicked: root.editGroup(existingGroup.currentIndex) }
        }
        ScrollView {
            visible: root.mode === "group"
            Layout.fillWidth: true
            Layout.fillHeight: true
            contentWidth: availableWidth
            ScrollBar.vertical: ThemeScrollBar {}
            ColumnLayout {
                width: parent.width
                spacing: 12
                Field { id: groupName; objectName: "groupName"; placeholderText: "Group name, e.g. Vave"; Accessible.name: "Group name" }
                Field { id: groupFolder; objectName: "groupFolder"; placeholderText: "Absolute folder, e.g. /home/me/Work"; Accessible.name: "Group folder" }
                Field { id: groupPattern; objectName: "groupPattern"; placeholderText: "Project pattern, e.g. vave-* (blank matches all)"; Accessible.name: "Project pattern" }
                Choice { id: groupKind; objectName: "groupKind"; model: ["All supported agents", "Codex", "Cursor"]; Accessible.name: "Agent type" }
                CheckBox {
                    id: futureMatches
                    objectName: "futureMatches"
                    text: "Include future matching sessions"
                    focusPolicy: Qt.StrongFocus
                    indicator: Rectangle {
                        implicitWidth: 20
                        implicitHeight: 20
                        anchors.verticalCenter: parent.verticalCenter
                        color: root.widget.ink("background")
                        border.width: futureMatches.activeFocus ? 2 : 1
                        border.color: root.widget.ink(futureMatches.activeFocus ? "accent" : "foreground")
                        Rectangle {
                            anchors.centerIn: parent
                            width: 10; height: 10
                            visible: futureMatches.checked
                            color: root.widget.ink("accent")
                        }
                    }
                    contentItem: Text { leftPadding: 32; text: futureMatches.text; color: root.widget.ink(futureMatches.activeFocus ? "accent" : "foreground"); wrapMode: Text.WordWrap }
                }
                Text {
                    Layout.fillWidth: true
                    text: "Off by default. When enabled, Miilo checks every minute and enrolls new matching sessions, up to 100 total. Unchecking this on a later import stops future enrollment; existing selections remain."
                    wrapMode: Text.WordWrap
                    color: root.widget.ink("muted")
                }
            }
        }
        Text {
            visible: root.mode === "loading"
            Layout.fillWidth: true
            text: "Looking for existing sessions…"
            color: root.widget.ink("foreground")
        }
        ListView {
            id: sessionList
            objectName: "sessionList"
            visible: root.mode === "review"
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            spacing: 4
            model: root.rows
            activeFocusOnTab: true
            keyNavigationEnabled: true
            highlightMoveDuration: 0
            Keys.onSpacePressed: root.activateRow()
            Keys.onReturnPressed: {
                var row = root.rows[currentIndex];
                if (row && row.header) root.collapseGroup(row.group); else root.activateRow();
            }
            Keys.onLeftPressed: { var row = root.rows[currentIndex]; if (row) root.collapseGroup(row.group,true); }
            Keys.onRightPressed: { var row = root.rows[currentIndex]; if (row) root.collapseGroup(row.group,false); }
            delegate: ItemDelegate {
                id: choice
                required property var modelData
                required property int index
                width: ListView.view.width
                focusPolicy: Qt.NoFocus
                readonly property var groupSessions: root.grouped[modelData.group] ? root.grouped[modelData.group].sessions.filter(s => s.observable) : []
                readonly property int selectedCount: groupSessions.filter(s => root.selected.indexOf(s.kind + ":" + s.id) >= 0).length
                readonly property bool partial: modelData.header && selectedCount > 0 && selectedCount < groupSessions.length
                readonly property bool cursor: sessionList.activeFocus && sessionList.currentIndex === index
                checked: modelData.header ? groupSessions.length > 0 && groupSessions.every(s => root.selected.indexOf(s.kind + ":" + s.id) >= 0) : root.selected.indexOf(modelData.session.kind + ":" + modelData.session.id) >= 0
                onClicked: { sessionList.currentIndex = index; sessionList.forceActiveFocus(); root.activateRow(); }
                implicitHeight: modelData.header ? 48 : 64
                background: Rectangle {
                    color: choice.down ? root.theme.pressedFill : choice.cursor ? root.theme.focusFill : choice.checked || choice.partial ? root.theme.selectedFill : choice.hovered ? root.theme.hoverFill : choice.modelData.header ? root.widget.ink("lighter_background") : "transparent"
                    Rectangle { anchors.left: parent.left; anchors.verticalCenter: parent.verticalCenter; width: 3; height: 24; visible: choice.cursor; color: root.theme.accent }
                }
                ToolButton {
                    id: foldButton
                    visible: choice.modelData.header
                    anchors.right: parent.right
                    anchors.verticalCenter: parent.verticalCenter
                    text: root.collapsed[choice.modelData.name] ? "+" : "−"
                    Accessible.name: "Expand or collapse " + choice.modelData.name
                    focusPolicy: Qt.NoFocus
                    contentItem: Text { text: foldButton.text; color: root.widget.ink("foreground"); horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter }
                    background: Rectangle { color: root.widget.ink(foldButton.hovered ? "selection" : "lighter_background") }
                    onClicked: root.collapseGroup(choice.modelData.group)
                }
                Rectangle {
                    x: 10
                    anchors.verticalCenter: parent.verticalCenter
                    width: 20
                    height: 20
                    radius: 0
                    color: "transparent"
                    border.width: 1
                    border.color: root.widget.ink(choice.modelData.header || choice.modelData.session.observable ? "foreground" : "muted")
                    Rectangle {
                        anchors.centerIn: parent
                        width: 10
                        height: choice.partial ? 2 : 10
                        visible: choice.checked || choice.partial
                        color: root.theme.accent
                    }
                }
                contentItem: Column {
                    leftPadding: 42
                    spacing: 3
                    Text {
                        width: parent.width - (choice.modelData.header ? 76 : 32)
                        text: choice.modelData.header ? (root.collapsed[choice.modelData.name] ? "▸ " : "▾ ") + choice.modelData.name + " (" + choice.modelData.count + ")" : choice.modelData.session.name || "Unnamed session"
                        elide: Text.ElideRight
                        color: choice.cursor ? root.theme.focusText : root.theme.foreground
                        font.pixelSize: 14
                    }
                    Text {
                        visible: !choice.modelData.header
                        width: parent.width - 32
                        text: choice.modelData.header ? "" : choice.modelData.session.kind + " · " + (choice.modelData.session.cwd || choice.modelData.session.id) + (choice.modelData.session.observable ? "" : " · import not available yet")
                        elide: Text.ElideMiddle
                        color: root.widget.ink("muted")
                        font.pixelSize: 11
                    }
                }
            }
            ScrollBar.vertical: ThemeScrollBar {}
        }
        Text {
            visible: root.mode === "review" && !root.sessions.length
            Layout.fillWidth: true
            text: root.error ? "" : "No supported sessions were found here. Choose a project directory and rescan."
            color: root.widget.ink("foreground")
            wrapMode: Text.WordWrap
        }
        Item {
            visible: root.mode !== "review" && root.mode !== "group"
            Layout.fillHeight: true
        }
        Text {
            visible: !!root.error
            Layout.fillWidth: true
            text: root.error
            color: root.widget.ink("red")
            wrapMode: Text.WordWrap
        }
        Text {
            Layout.fillWidth: true
            text: root.groupsDirty ? "Group changes are saved when you confirm below. Future enrollment requires its own opt-in. Ctrl+Enter confirms; Escape goes back." : "Only selected sessions are imported, plus future matches of opted-in groups. Tab moves focus; Ctrl+Enter confirms. Agent credentials stay in place."
            color: root.widget.ink("muted")
            font.pixelSize: 12
            wrapMode: Text.WordWrap
        }
        RowLayout {
            Layout.fillWidth: true
            Action {
                id: reviewButton
                visible: root.mode === "invite"
                text: "Review sessions"
                onClicked: root.review()
            }
            Action { visible: root.mode === "group"; text: "Save group"; onClicked: root.saveGroup() }
            Action {
                visible: root.mode === "group" && root.editIndex >= 0
                text: "Remove group"
                onClicked: { var next = root.groups.slice(); next.splice(root.editIndex,1); root.pendingGroups = next; root.regroup(); }
            }
            Action {
                visible: root.mode === "review"
                text: (root.groupsDirty ? "Apply groups & import (" : "Import selected (") + root.selected.length + ")"
                enabled: (root.selected.length > 0 || root.groupsDirty) && !root.busy
                onClicked: root.importSelected()
            }
            Item {
                Layout.fillWidth: true
            }
            Action {
                visible: !root.busy
                text: root.mode === "invite" ? "Not now" : "Cancel"
                onClicked: { if (root.mode === "group") root.mode = "review"; else root.dismiss(); }
            }
        }
    }
}
