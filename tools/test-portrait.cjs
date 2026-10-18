// Guard against redesign drift: compare the generated portrait to the fleet Bot.
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),Module=require('node:module'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..'),module_=new Module('owl');
module_._compile(fs.readFileSync(path.join(root,'desktop/runtime/Owl.js'),'utf8').replace(/^\.pragma library/,''),'owl');
const Owl=module_.exports,scope={};
vm.runInNewContext(fs.readFileSync(path.join(root,'desktop/RobotProgram.js'),'utf8').replace(/^\.pragma library/,''),scope);
const authored=fs.readFileSync(path.join(root,'widgets/robot.inc'),'utf8');
assert(scope.source.includes(authored),'Portrait must embed the exact authored Bot');
const portrait=Owl.compile(scope.source);
assert(!portrait.globals.some(g=>g.type==='source'),'Portrait must have no external sources');
const fleet=Owl.compile(fs.readFileSync(path.join(root,'70-miilo-fleet.owl'),'utf8'));
function* walk(n){yield n;for(const c of n.c||n.children||[])yield*walk(c);}
for(const light of [false,true])for(const status of ['ready','resting','working'])for(const motion of ['full','reduced','off']){
 const errors=[],palette=Owl.themeFromToml(light?'background="#faf5ee"\nforeground="#28232e"':'');
 const host={theme:n=>palette[n]??null,tile:n=>({tileWidth:260,tileHeight:180,screenWidth:720,screenHeight:760})[n]??null,report:e=>errors.push(e),changed(){},act(){},runInto(){throw Error('Unexpected portrait I/O');}};
 const agent={_key:'welcome',name:'Miilo',providerId:'sample',status,health:'healthy'};
 const a=new Owl.Instance(fleet,host,{robotMotion:motion}),b=new Owl.Instance(portrait,host,{});
 const slot=(program,k)=>program.globals.findIndex(g=>g.name===k);
 a.setSource(slot(fleet,'feed'),Owl.fromJson({ok:true,out:JSON.stringify({configured:true,errors:[],snapshot:{providers:[{id:'sample',name:'Sample'}],actors:[agent],workItems:[],attention:[]}})}));
 for(const t of [.6,1.4,6.0]){
  a.frame(t);const canvas=[...walk(a.render(false))].find(n=>n.kind==='Canvas'&&!n.p?.fill);assert(canvas);
  for(const [name,value]of Object.entries({portraitAgent:agent,portraitSize:canvas.p.width,portraitAnchored:true,robotMotion:motion}))b.values[slot(portrait,name)]=Owl.fromJson(value);
  b.invalidate();b.frame(t);const standalone=b.render(false).children.find(n=>n.kind==='Canvas');
  assert.deepEqual(standalone.draw(canvas.p.width,canvas.p.height),canvas.draw(canvas.p.width,canvas.p.height),`${status}/${motion}/${light}/${t} design drift`);
 }
 assert.deepEqual(errors,[]);
}
console.log('Original Bot portrait parity passed: 18 state/theme/motion combinations across 3 controlled frames (54 comparisons), exact draw operations and no I/O.');
