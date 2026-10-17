import QtQuick
import QtTest
import "../../desktop" as Miilo
import "../../desktop/runtime" as Owl

Item {
    width: 320; height: 480
    Owl.MiiloTypography { id: types; profile: "phone"; textScale: 2 }
    QtObject {
        id: palette
        property var typography: types
        function ink(name) { return name === "background" ? "#111111" : "#eeeeee" }
    }
    Miilo.LocalImport { id: importer; anchors.fill: parent; widget: palette; active: false }
    TestCase {
        name: "LegacyTypography"
        when: true
        onCompletedChanged: if (completed) console.log(qtest_results.failCount === 0 ? "MIILO_TYPOGRAPHY_OK" : "MIILO_TYPOGRAPHY_FAILED", qtest_results.passCount, qtest_results.failCount)
        function test_narrow_large_text_is_scrollable() {
            importer.mode = "invite";
            importer.agents = [{name:"Long agent setup name",locations:["/tmp/project"],observationSupported:true}];
            var scroll = findChild(importer,"onboardingScroll"); verify(scroll);
            tryVerify(() => scroll.contentHeight > scroll.availableHeight);
            var button = findChild(importer,"reviewButton"); verify(button);
            tryVerify(() => button.width <= scroll.availableWidth - 32);
            verify(button.contentItem.paintedWidth <= button.contentItem.width + 1);
            verify(button.implicitHeight >= button.contentItem.implicitHeight);
            scroll.contentItem.contentY = scroll.contentHeight - scroll.availableHeight;
            verify(scroll.contentItem.contentY > 0);
        }
    }
}
