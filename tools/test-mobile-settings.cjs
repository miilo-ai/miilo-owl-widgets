// Exercise navigation and deferred configuration through the actual OWL evaluator.
const fs=require('node:fs'),Module=require('node:module'),assert=require('node:assert/strict');
const runtime=new Module('owl');runtime._compile(fs.readFileSync('desktop/runtime/Owl.js','utf8').replace(/^\.pragma library/,''),'owl');
const Owl=runtime.exports;
function* walk(n){yield n;for(const c of n.c||n.children||[])yield*walk(c);}
let cases=0;
for(const file of ['70-miilo-fleet.owl','71-miilo-attention.owl','72-miilo-board.owl'])for(const width of [320,393,600])for(const light of [false,true]){
 const program=Owl.compile(fs.readFileSync(file,'utf8')),errors=[],commands=[],palette=Owl.themeFromToml(light?'background="#ffffff"\nforeground="#111111"':'');
 const instance=new Owl.Instance(program,{theme:k=>palette[k]??null,tile:k=>({linux:true,tileWidth:width,tileHeight:180,screenWidth:width,screenHeight:800})[k]??null,report:e=>errors.push(e),changed(){},act(){},runInto:c=>commands.push(c)},{});
 const slot=k=>program.globals.findIndex(g=>g.name===k),set=(k,v)=>{instance.values[slot(k)]=Owl.fromJson(v);instance.invalidate();};
 const item={_key:'w1',providerId:'p1',title:'Inspect a workflow',phase:'reviewing',attention:'review',lifecycle:'waiting',_revisionKey:'r1',actions:[]};
 instance.setSource(slot('feed'),Owl.fromJson({ok:true,out:JSON.stringify({configured:true,errors:[],snapshot:{providers:[{id:'p1',name:'Private factory',health:'healthy'}],actors:[],workItems:[item],attention:[]}}),code:0,err:''}));
 const nodes=()=>[...walk(instance.render(true))],button=label=>nodes().find(n=>n.kind==='Button'&&n.p.text===label);
 set('selectedWork','w1');set('setupOpen',true);
 assert(!nodes().some(n=>n.p?.text===item.title),'Task content must be isolated from settings');
 for(const [label,section]of [['Appearance','appearance'],['Connections','systems'],['Chat model','chat'],['Systems','filters'],['Pair computer','pair']]){
  set('setupSection','menu');const entry=nodes().find(n=>n.kind==='Box'&&n.h?.onTap&&[...walk(n)].some(c=>c.p?.text===label));assert(entry,label+' entry');entry.h.onTap(0,0);
  assert.equal(instance.values[slot('setupSection')],section);
  assert(!nodes().some(n=>n.kind==='Input'),'Task search must not render in settings');
  assert(!nodes().some(n=>n.kind==='Terminal'),'Opening a page must not start a terminal');
  if(section==='appearance'){
   button('Reduced').h.onTap(0,0);assert.equal(instance.values[slot('robotMotion')],'reduced');
  }else if(section==='filters'){
   const provider=nodes().find(n=>n.kind==='Box'&&n.h?.onTap&&[...walk(n)].some(c=>c.p?.text==='Private factory'));provider.h.onTap(0,0);assert.equal(instance.values[slot('system')],'p1');
  }else{
   button(section==='chat'?'Configure chat model':section==='pair'?'Pair computer':'Manage connections').h.onTap(0,0);
   const terminal=nodes().find(n=>n.kind==='Terminal');assert(terminal);assert.equal(terminal.p.command,section==='pair'?'export PATH="$HOME/.local/bin:$PATH"; miilo-owl pair':section==='chat'?'export PATH="$HOME/.local/bin:$PATH"; miilo-owl chat-setup':'export PATH="$HOME/.local/bin:$PATH"; miilo-owl setup');assert(terminal.p.height<=800);
   set('setupConfiguring',false);
  }
 }
 button('Done').h.onTap(0,0);assert.equal(instance.values[slot('setupOpen')],false);assert.equal(instance.values[slot('selectedWork')],'w1');
 assert(nodes().some(n=>n.p?.text===item.title),'Closing settings restores task');
 assert.deepEqual(errors,[]);assert.deepEqual(commands,[],'Settings cannot dispatch provider mutations');cases++;
}
console.log(`${cases} mobile settings scenarios passed: isolation, navigation, preferences, deferred terminals and task restoration.`);
