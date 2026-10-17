import QtQuick
import QtQuick.Window
import "runtime" as Runtime
import "runtime/Owl.js" as Owl
Window {
 id: root; width: 900; height: 850; visible: true; color: sceneLight ? "#f4f2f8" : "#16151d"
 property var errors: []
 property real sceneTime: .2
 property string selection: { var f=widget.frame; return widget.instance ? String(widget.instance.read(widget.program.globals.findIndex(g=>g.name==="selectedWork"))) : "" }
 function advance(t) { sceneTime=t; widget.renderAt(t) }
 Item {
  id: widget
  property real dp:1
  property int frame:0
  property string home:""
  property string sansFamily:"Helvetica"
  property string monoFamily:"Menlo"
  property string iconFamily:"Helvetica"
  property var configureLocal:null
  property var palette: Owl.themeFromToml(sceneLight ? 'accent="#6047ad"\nbackground="#f4f2f8"\nforeground="#25202e"' : 'accent="#ac9cf5"\nbackground="#16151d"\nforeground="#eeeaf4"')
  property var program: Owl.compile(widgetSource)
  property var instance: null
  property var tree:null
  function ink(n) { return Owl.hex8(palette[n] || palette.foreground) }
  function alpha(n,a) { var c=Qt.color(n);return Qt.rgba(c.r,c.g,c.b,a) }
  function renderAt(t) {
   instance.frame(t);var v=instance.render(true);tree=v.children.length===1?v.children[0]:{kind:"Column",p:{gap:0},h:{},a:{},c:v.children};frame++;
  }
  Component.onCompleted: {
   instance=new Owl.Instance(program,{
    theme:n=>palette[n]===undefined?null:palette[n],
    tile:n=>({linux:true,tileWidth:260,tileHeight:180,screenWidth:root.width-36,screenHeight:root.height-36})[n] || null,
    report:e=>root.errors.push(e),changed:()=>Qt.callLater(()=>widget.renderAt(root.sceneTime)),act:()=>{},runInto:()=>root.errors.push("Unexpected process")
   },{observatoryView:"orbit",robotMotion:sceneMotion});
   var slot=program.globals.findIndex(g=>g.name==="feed");
   instance.setSource(slot,Owl.fromJson({ok:true,out:JSON.stringify(sceneData),code:0,err:""}));
   renderAt(.2);
  }
 }
 Runtime.OwlNode { anchors.fill:parent; anchors.margins:18; widget:widget;node:widget.tree }
}
