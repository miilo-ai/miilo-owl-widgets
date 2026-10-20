.pragma library
.import "Capture.js" as Capture
function run(test, scene, config) {
    for (var name of ["compactAction", "compactField", "compactChoice"]) {
        var control = test.findChild(scene.contentItem, name)
        test.verify(control !== null, "Missing compact control: " + name)
        test.verify(control.topPadding <= 4 && control.bottomPadding <= 4, "Control padding was not reduced")
        var textHeight = typeof control.contentHeight === "number" ? control.contentHeight : control.contentItem.implicitHeight
        test.verify(Number.isFinite(textHeight) && control.height >= textHeight,
                    "Compact control clips text: " + name + " (" + control.height + " < " + textHeight + ")")
    }
    Capture.capture(test, scene, config, "crew-dark.png")
    var row = Capture.find(scene.contentItem, function(item) { return item.expanded !== undefined && !item.expanded })
    Capture.click(test, row)
    test.verify(Capture.find(scene.contentItem, function(item) { return item.expanded === true }) !== null,
                "Agent row did not expand")
    Capture.capture(test, scene, config, "crew-expanded.png")
    scene.light = true
    scene.width = 360
    Capture.capture(test, scene, config, "crew-light-narrow.png")
}
