.pragma library
.import "Capture.js" as Capture
function run(test, scene, config) {
    var first = Capture.capture(test, scene, config, "orbit.png")
    // Reduced motion retains blinks; sample inside the first robot's blink window.
    scene.advance(config.motion === "reduced" ? 0.05 : 1.4)
    var next = Capture.capture(test, scene, config, "orbit-next-frame.png")
    if (config.motion !== "off") test.verify(!first.equals(next), "Animated fixture frames are identical")
    else test.verify(first.equals(next), "Off motion changed the rendered frame")
    if (config.record) {
        for (var index = 0; index <= config.seconds * 12; ++index) {
            if (index === 24 && config.view === "inspect") {
                Capture.click(test, Capture.text(scene, "Review summary"))
                test.verify(scene.selection !== "", "Inspection click failed")
            }
            scene.advance(index / 12)
            Capture.capture(test, scene, config, "frames/" + String(index).padStart(4, "0") + ".png")
        }
    }
    if (config.view === "quiet" || config.view === "quiet-expanded") {
        scene.height = 680
        test.wait(100)
        Capture.click(test, Capture.text(scene, "Automation factory"))
    }
    if (scene.selection === "") {
        Capture.click(test, Capture.text(scene, "Review summary"))
        test.verify(scene.selection !== "", "Work selection did not open details")
    }
    for (var text of ["Files & changes", "activity-summary.ts", "Verification output"])
        test.verify(Capture.text(scene, text) !== null, "Missing evidence: " + text)
    Capture.capture(test, scene, config, "orbit-detail.png")
    test.compare(scene.errors.length, 0, "OWL runtime errors")
}
