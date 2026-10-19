import QtQuick
import QtTest
import "../../desktop/runtime" as Owl
Item {
    id: root
    width: 360; height: 240
    property string scratchPath: ""
    Owl.OwlWidget {
        id: subject
        width: 360; height: 240; active: true
        host: Owl.OwlHost { stateDir: root.scratchPath + "/state" }
        errorDir: root.scratchPath + "/errors"
        Component.onCompleted: {
            stateRead=true;
            sourceText='widget "clock-check" { size: 2x2 component Orbit() { let clock = time Canvas { width: 180; height: 120 circle(10 + clock * 15, 20, 4, fill: "#ffffff") } } Orbit {} }';
            load();
        }
    }
    Owl.OwlNode {
        anchors.fill: parent
        widget: subject
        node: subject.tree ? subject.tree.children[0] : null
    }
    property int visibleDraws: 0
    property int staticDraws: 0
    Item {
        id: fakeWidget
        x: 180; width: 180; height: 100
        property bool active: true
        property int frame: 0
        property real dp: 1
        property var palette: ({})
        property string home: root.scratchPath
        property string sansFamily: "sans-serif"
        function ink(name) { return "#ffffff" }
        Flickable {
            id: viewport
            width: 180; height: 80; contentHeight: 400; clip: true
            Owl.OwlCanvas {
                y: 200; width: 160; height: 60; widget: fakeWidget
                node: ({ draw: function(w,h) { root.visibleDraws++; return [] } })
            }
        }
        Owl.OwlCanvas {
            id: staticCanvas
            width: 160; height: 60; widget: fakeWidget
            node: ({ draw: root.stillDraw })
        }
    }
    function stillDraw(w,h) { root.staticDraws++; root.stillDraw.readsTime=false; return [] }
    function phaseX() {
        var canvas=subject.tree.children[0];
        return canvas.draw(180,120)[0].args[0];
    }
    TestCase {
        name: "CanvasClock"
        when: true
        onCompletedChanged: if(completed) console.log(qtest_results.failCount===0 ? "MIILO_CANVAS_CLOCK_OK" : "MIILO_CANVAS_CLOCK_FAILED")
        function test_static_canvas_and_offscreen_culling() {
            wait(100);
            compare(root.visibleDraws,0,"Offscreen Canvas must not evaluate drawing");
            var still=root.staticDraws;
            for(var n=0;n<4;n++){fakeWidget.frame++;wait(40)}
            compare(root.staticDraws,still,"Static Canvas must not repaint on clock ticks");
            viewport.contentY=190;fakeWidget.frame++;wait(100);
            verify(root.visibleDraws>0,"Scrolling Canvas into view must resume drawing");
            viewport.contentY=0;fakeWidget.frame++;wait(80);
            var hidden=root.visibleDraws;
            for(var i=0;i<4;i++){fakeWidget.frame++;wait(40)}
            compare(root.visibleDraws,hidden,"Offscreen motion must stop drawing");
            viewport.contentY=190;wait(100);
            verify(root.visibleDraws>hidden,"Scroll must repaint without a clock tick");
            staticCanvas.width=150;wait(80);
            verify(root.staticDraws>still,"Static resize must repaint");
        }
        function test_component_clock_advances_without_source_refresh() {
            tryVerify(()=>subject.instance!==null && subject.tree!==null);
            console.log("CLOCK_READY",subject.error,subject.problems,subject.tree.readsTime);
            verify(subject.program.animated);
            verify(!subject.tree.readsTime,"Fixture must read its clock only during deferred draw");
            var initialTree=subject.tree;
            wait(200);
            var previous=root.phaseX();
            for(var i=0;i<5;i++) {
                wait(100);
                var next=root.phaseX();
                console.log("CLOCK_SAMPLE",previous,next,subject.frame);
                verify(next>previous+0.2,"Canvas clock froze between source refreshes: "+previous+" -> "+next);
                verify(next-previous<6,"Unexpected jump between consecutive frames");
                previous=next;
            }
            compare(subject.tree,initialTree,"Canvas-only motion must not replace layout");
            compare(subject.problems.length,0);
        }
    }
}
