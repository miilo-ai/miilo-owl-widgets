import QtQuick
import QtTest
import "PreviewConfig.js" as Config
import "Capture.js" as Capture
import "Orbit.js" as Orbit
import "Crew.js" as Crew
TestCase {
    id: test
    name: "MiiloPreview"
    when: windowShown
    property var scene: null
    function init() { failOnWarning(/^(file:|qrc:|.*(?:ReferenceError|TypeError|SyntaxError):)/) }
    function cleanupTestCase() { if (scene) scene.destroy() }
    function test_preview() {
        var settings = Config.settings
        var component = Qt.createComponent(settings.fixture)
        compare(component.status, Component.Ready, component.errorString())
        scene = component.createObject(null, settings.properties)
        verify(scene !== null, component.errorString())
        tryCompare(scene, "visible", true)
        wait(100)
        if (settings.mode === "orbit") Orbit.run(test, scene, settings)
        else Crew.run(test, scene, settings)
    }
}
