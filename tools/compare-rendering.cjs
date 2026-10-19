// Compare retained animation output against an explicitly selected baseline.
const fs=require('node:fs'),path=require('node:path'),Module=require('node:module'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..');
function load(file){const m=new Module('owl');m._compile(fs.readFileSync(file,'utf8').replace(/^\.pragma library/,''),'owl');return m.exports;}
function scene(Owl,file,view){
 const program=Owl.compile(fs.readFileSync(file,'utf8')),palette=Owl.themeFromToml(''),errors=[];
 const instance=new Owl.Instance(program,{theme:k=>palette[k]??null,tile:k=>({linux:true,tileWidth:580,tileHeight:620,screenWidth:580,screenHeight:620})[k]??null,report:e=>errors.push(e),changed(){},act(){throw Error('Unexpected action');},runInto(){throw Error('Unexpected process');}},{observatoryView:view});
 const states=['working','failed','resting','blocked'];
 const actors=states.map((status,i)=>({_key:'a'+i,id:'a'+i,providerId:'p'+i,name:'Sample '+i,status,health:'healthy'}));
 const feed={configured:true,errors:[],snapshot:{providers:actors.map(a=>({id:a.providerId,name:a.name,health:'healthy'})),actors,workItems:actors.map((a,i)=>({_key:a.id+'w',id:a.id+'w',title:'Inspect sample',providerId:a.providerId,_actorKeys:[a._key],lifecycle:i===1?'failed':i===2?'completed':i===3?'waiting':'active',phase:'implementing',attention:i===3?'approval':'none'})),attention:[]}};
 instance.setSource(program.globals.findIndex(g=>g.name==='feed'),Owl.fromJson({ok:true,out:JSON.stringify(feed),code:0,err:''}));return {instance,errors};
}
function* walk(n){for(const c of n.c||n.children||[]){yield c;yield*walk(c);}}
function operations(tree){return [...walk(tree)].filter(n=>n.kind==='Canvas').map(c=>c.draw(c.p.width||260,c.p.height||164));}
function equal(a,b){if(typeof a==='number'&&typeof b==='number'){assert(Math.abs(a-b)<1e-9,`${a} != ${b}`);return;}
 if(a&&b&&typeof a==='object'&&typeof b==='object'){assert.deepEqual(Object.keys(a),Object.keys(b));for(const k of Object.keys(a))equal(a[k],b[k]);return;}assert.equal(a,b);}
assert(process.argv[2]&&process.argv[3],'Usage: node tools/compare-rendering.cjs BASELINE_RUNTIME BASELINE_WIDGET');
const old=load(process.argv[2]),current=load(path.join(root,'desktop/runtime/Owl.js'));let comparisons=0;
for(const view of ['quiet','orbit'])for(const expanded of [false,true]){
 const a=scene(old,process.argv[3],view),b=scene(current,path.join(root,'70-miilo-fleet.owl'),view);let retained=null;
 for(const time of [0,.1,1,2,7.3,17.7,33]){
  a.instance.frame(time);b.instance.frame(time);
  const expected=operations(a.instance.render(expanded));
  if(!retained||retained.readsTime)retained=b.instance.render(expanded);
  equal(expected,operations(retained));comparisons+=expected.length;
 }
 assert.deepEqual(a.errors,[]);assert.deepEqual(b.errors,[]);
}
console.log(`${comparisons} canvas comparisons matched the baseline across modes, states and clock phases (numeric tolerance 1e-9).`);
