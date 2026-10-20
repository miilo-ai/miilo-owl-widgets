import QtQuick
import QtQuick.Window
import "runtime" as Runtime
import "runtime/Owl.js" as Owl
Window {
 id: root; width: 900; height: 850; visible: true; color: widget.ink("background")
 property string widgetSource: ""
 property var sceneData: ({})
 property string sceneTheme: ""
 property string sceneView: "orbit"
 property string sceneMotion: "full"
 property bool sceneLight: false
 Rectangle { anchors.fill: parent; color: root.color; z: -1 }
 property var errors: []
 property real sceneTime: .2
 property bool showScreen: sceneView !== "quiet"
 property string selection: { var f=widget.frame; return widget.instance ? String(widget.instance.read(widget.program.globals.findIndex(g=>g.name==="selectedWork"))) : "" }
 function advance(t) { sceneTime=t; widget.renderAt(t) }
 Item {
  id: widget
  property bool active: true
  property real dp:1
  property int frame:0
  property string home:""
  property string sansFamily:"Helvetica"
  property string monoFamily:"Menlo"
  property string iconFamily:"Helvetica"
  property var configureLocal:null
  // Dark palette: Omarchy Tokyo Night colors.toml; retain synthetic light QA.
  // Same muted-text contrast floor as the native MiiloSurface host.
    function readablePalette(toml) {
        var p = Owl.themeFromToml(toml)
        function luminance(c) {
            var q = Qt.color(Owl.hex8(c))
            function linear(v) { return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4) }
            return 0.2126 * linear(q.r) + 0.7152 * linear(q.g) + 0.0722 * linear(q.b)
        }
        var bg = luminance(p.background), original = Qt.color(Owl.hex8(p.muted)), fg = Qt.color(Owl.hex8(p.foreground))
        for (var step = 0; step <= 20; step++) {
            var t = step / 20
            var c = Qt.rgba(original.r * (1-t) + fg.r*t, original.g * (1-t) + fg.g*t, original.b * (1-t) + fg.b*t, 1)
            var l = luminance(c)
            if ((Math.max(l,bg)+0.05)/(Math.min(l,bg)+0.05) >= 4.5 || step === 20) {
                p.muted = Owl.argb(c.toString())
                break
            }
        }
        return p
    }
  property var palette: readablePalette(sceneTheme !== "" ? sceneTheme : sceneLight ? 'accent="#6047ad"\nbackground="#f4f2f8"\nforeground="#25202e"' : "mode = \"dark\"\n\naccent = \"#7aa2f7\"\nselection = \"#292e42\"\nmuted = \"#414868\"\n\nbackground = \"#1a1b26\"\ndark_background = \"#13141c\"\ndarker_background = \"#0e0e14\"\nlighter_background = \"#24283b\"\n\nforeground = \"#a9b1d6\"\ndark_foreground = \"#565f89\"\nlight_foreground = \"#b4bee6\"\nbright_foreground = \"#c0caf5\"\n\nred = \"#f7768e\"\nyellow = \"#e0af68\"\norange = \"#eb927b\"\ngreen = \"#9ece6a\"\ncyan = \"#449dab\"\nblue = \"#7aa2f7\"\nmagenta = \"#ad8ee6\"\nbrown = \"#75493d\"\n\nbright_red = \"#ff7a93\"\nbright_yellow = \"#ff9e64\"\nbright_green = \"#b9f27c\"\nbright_cyan = \"#0db9d7\"\nbright_blue = \"#7da6ff\"\nbright_magenta = \"#bb9af7\"\n")
  property var program: Owl.compile(widgetSource)
  property var instance: null
  property var tree:null
  function ink(n) { return Owl.hex8(palette[n] || palette.foreground) }
  function alpha(n,a) { var c=Qt.color(n);return Qt.rgba(c.r,c.g,c.b,a) }
  function renderAt(t) {
   instance.frame(t);var v=instance.render(root.showScreen);tree=v.children.length===1?v.children[0]:{kind:"Column",p:{gap:0},h:{},a:{},c:v.children};frame++;
  }
  Component.onCompleted: {
   instance=new Owl.Instance(program,{
    theme:n=>palette[n]===undefined?null:palette[n],
    tile:n=>({linux:true,tileWidth:root.width-36,tileHeight:root.height-36,screenWidth:root.width-36,screenHeight:root.height-36})[n] || null,
    report:e=>root.errors.push(e),changed:()=>Qt.callLater(()=>widget.renderAt(root.sceneTime)),act:(name)=>{ if(name==="screen"){root.showScreen=true;widget.renderAt(root.sceneTime)} },runInto:()=>root.errors.push("Unexpected process")
   },{observatoryView:sceneView === "quiet-expanded" ? "quiet" : "orbit",robotMotion:sceneMotion});
   var slot=program.globals.findIndex(g=>g.name==="feed");
   instance.setSource(slot,Owl.fromJson({ok:true,out:JSON.stringify(sceneData),code:0,err:""}));
   renderAt(.2);
  }
 }
 Runtime.OwlNode { anchors.fill:parent; anchors.margins:18; widget:widget;node:widget.tree }
}
