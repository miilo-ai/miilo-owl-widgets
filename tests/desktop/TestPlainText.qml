import QtQuick
import QtTest
import "../../desktop/runtime" as Owl
Item {
    id: root
    width: 900; height: 700
    property string canaryBase: ""
    function payload(name) { return '<b>' + name + '</b><img src="' + canaryBase + '/' + name + '">' }
    Owl.MiiloTypography { id: typeSystem }
    QtObject {
        id: palette
        property real dp: 1
        property var typography: typeSystem
        property bool active: false
        property string monoFamily: "monospace"
        property string sansFamily: "sans-serif"
        function ink(name) { return name === "background" ? "#111111" : "#eeeeee" }
        function alpha(color, value) { return Qt.rgba(0,0,0,value) }
    }
    Text {
        id: control
        textFormat: Text.RichText
        text: '<img src="' + root.canaryBase + '/positive-control">'
    }
    Owl.OwlNode {
        id: button; y: 50; width: 500; height: 45; widget: palette
        node: ({kind:"Button",p:{text:root.payload("button")},h:{},a:{},c:[]})
    }
    Owl.OwlNode {
        id: label; y: 110; width: 500; height: 45; widget: palette
        node: ({kind:"Text",p:{text:root.payload("label")},h:{},a:{},c:[]})
    }
    Owl.OwlNode {
        id: input; y: 170; width: 500; height: 45; widget: palette
        node: ({kind:"Input",p:{placeholder:root.payload("placeholder"),text:""},h:{},a:{},c:[]})
    }
    Owl.OwlTerminal {
        id: terminal; y: 230; width: 700; height: 180; widget: palette
        lines: [root.payload("terminal")]
    }
    function findText(item, expected) {
        if (item.text !== undefined && item.text === expected && item.textFormat !== undefined) return item;
        for (var i=0; i<item.children.length; i++) {
            var result=findText(item.children[i],expected);
            if (result) return result;
        }
        return null;
    }
    TestCase {
        name: "PlainTextSecurity"
        when: true
        onCompletedChanged: if (completed) console.log(qtest_results.failCount === 0 ? "MIILO_PLAIN_TEXT_OK" : "MIILO_PLAIN_TEXT_FAILED")
        function test_untrusted_display_is_literal() {
            wait(500);
            for (var entry of [[button,"button"],[label,"label"],[input,"placeholder"],[terminal,"terminal"]]) {
                var textItem=null;
                tryVerify(function() { textItem=root.findText(entry[0],root.payload(entry[1])); return textItem !== null; });
                compare(textItem.textFormat,Text.PlainText,entry[1]+" must be plain text");
                compare(textItem.text,root.payload(entry[1]));
            }
            wait(1000);
        }
    }
}
