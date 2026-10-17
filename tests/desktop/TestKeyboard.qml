import QtQuick
import QtTest
import "../../desktop" as Miilo
import "../../desktop/runtime" as Owl

Item {
    width: 900
    height: 800
    QtObject {
        id: palette
        property real dp: 1
        property string sansFamily: "sans-serif"
        property string monoFamily: "monospace"
        function ink(name) { return name === "background" ? "#111111" : name === "accent" ? "#ff6600" : "#eeeeee" }
        function alpha(color, opacity) { return color }
    }
    Miilo.LocalImport { id: importer; anchors.fill: parent; widget: palette; active: false }
    Owl.OwlNode {
        id: action
        property int activated: 0
        width: 100; height: 40
        widget: palette
        visible: false
        node: ({kind:"Text",p:{text:"Action"},h:{onTap:() => action.activated++},a:{},c:[]})
    }
    TestCase {
        name: "MiiloKeyboard"
        onCompletedChanged: if (completed) console.log(qtest_results.passCount >= 4 && qtest_results.failCount === 0 ? "MIILO_KEYBOARD_OK" : "MIILO_KEYBOARD_FAILED", qtest_results.passCount, qtest_results.failCount)
        when: true
        function init() {
            importer.pendingGroups = null; importer.rows = []; importer.grouped = [];
            importer.groups = []; importer.collapsed = ({}); importer.selected = [];
            importer.sessions = [
                {kind:"codex",id:"one",name:"First",cwd:"/tmp",observable:true},
                {kind:"codex",id:"two",name:"Second",cwd:"/tmp",observable:true},
                {kind:"devin",id:"unsupported",name:"Unavailable",cwd:"/tmp",observable:false}
            ];
            importer.mode = "review"; importer.regroup();
            tryVerify(() => importer.rows.length === 5);
        }
        function cleanup() { console.log("KEYBOARD_CASE",qtest_results.functionName,qtest_results.failed ? "FAILED" : "passed"); importer.mode = ""; action.visible = false }
        function test_list_keys_select_and_fold() {
            var list = findChild(importer,"sessionList"); verify(list);
            list.currentIndex = 0; list.forceActiveFocus();
            keyClick(Qt.Key_Space); compare(importer.selected.length,2);
            keyClick(Qt.Key_Down); compare(list.currentIndex,1);
            keyClick(Qt.Key_Space); compare(importer.selected.length,1);
            keyClick(Qt.Key_Up); keyClick(Qt.Key_Return);
            verify(list.currentItem.partial);
            compare(importer.selected.length,1);
            compare(importer.rows.length,3);
            keyClick(Qt.Key_Right); compare(importer.rows.length,5);
            keyClick(Qt.Key_Tab); verify(!list.activeFocus);
            keyClick(Qt.Key_Tab,Qt.ShiftModifier); verify(list.activeFocus);
            keyClick(Qt.Key_Escape); compare(importer.mode,"");
        }
        function test_group_editor_is_keyboard_accessible_and_future_is_off() {
            importer.editGroup(-1);
            var name = findChild(importer,"groupName"); verify(name);
            tryVerify(() => name.activeFocus);
            keyClick("M"); keyClick("i"); keyClick("i"); keyClick("l"); keyClick("o"); compare(name.text,"Miilo");
            keyClick(Qt.Key_Tab);
            var folder = findChild(importer,"groupFolder"); verify(folder.activeFocus);
            keyClick(Qt.Key_Tab); keyClick("m"); keyClick("i"); keyClick("i"); keyClick("l"); keyClick("o"); keyClick("-"); keyClick("*");
            compare(findChild(importer,"groupPattern").text,"miilo-*");
            verify(!findChild(importer,"futureMatches").checked);
            keyClick(Qt.Key_Tab);
            var kind = findChild(importer,"groupKind"); verify(kind.activeFocus);
            keyClick(Qt.Key_Space); tryVerify(() => kind.popup.visible);
            keyClick(Qt.Key_Down); keyClick(Qt.Key_Return);
            compare(kind.currentIndex,1);
            tryVerify(() => !kind.popup.visible);
            keyClick(Qt.Key_Escape); compare(importer.mode,"review");
        }
        function test_native_dashboard_actions_accept_keyboard() {
            importer.mode = ""; action.visible = true; action.activated = 0;
            action.forceActiveFocus();
            keyClick(Qt.Key_Return); compare(action.activated,1);
            keyClick(Qt.Key_Space); compare(action.activated,2);
        }
        function test_group_preview_accepts_valid_rules_and_rejects_invalid_folder() {
            importer.editGroup(-1);
            findChild(importer,"groupName").text = "Temporary sessions";
            findChild(importer,"groupFolder").text = "/tmp";
            importer.saveGroup();
            tryCompare(importer,"mode","review");
            compare(importer.groups.length,1);
            compare(importer.groups[0].future,false);
            compare(importer.grouped[0].sessions.length,2);
            importer.editGroup(0);
            findChild(importer,"groupFolder").text = "relative/path";
            importer.saveGroup();
            tryVerify(() => importer.error.length > 0);
            compare(importer.mode,"group");
            compare(importer.groups[0].folder,"/tmp");
        }
    }
}
