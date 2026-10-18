// Compact density exercises the real generated widget; no provider side effects.
const fs=require('node:fs'),Module=require('node:module'),assert=require('node:assert/strict');
const runtime=new Module('owl');runtime._compile(fs.readFileSync('desktop/runtime/Owl.js','utf8').replace(/^\.pragma library/,''),'owl');
const Owl=runtime.exports,program=Owl.compile(fs.readFileSync('70-miilo-fleet.owl','utf8'));
function* walk(n){yield n;for(const c of n.c||n.children||[])yield*walk(c);}
for(const [width,height,columns,capacity]of [[260,180,2,4],[320,180,3,6],[393,180,4,8],[480,360,4,8]]){
 const errors=[],palette=Owl.themeFromToml(''),actors=Array.from({length:8},(_,i)=>({_key:'a'+i,id:'a'+i,providerId:'p'+i,name:'System '+i,status:'working',health:'healthy'}));
 const instance=new Owl.Instance(program,{theme:k=>palette[k]??null,tile:k=>({linux:false,tileWidth:width,tileHeight:height,screenWidth:width,screenHeight:800})[k]??null,report:e=>errors.push(e),changed(){},act(){},runInto(){throw Error('Unexpected provider action');}},{});
 const feed={configured:true,errors:[],snapshot:{actors,providers:actors.map(a=>({id:a.providerId,name:a.name,health:'healthy'})),workItems:actors.map(a=>({_key:a._key+'w',providerId:a.providerId,_actorKeys:[a._key],lifecycle:'active',attention:'none',phase:'implementing'})),attention:[]}};
 instance.setSource(program.globals.findIndex(g=>g.name==='feed'),Owl.fromJson({ok:true,out:JSON.stringify(feed),code:0,err:''}));
 const nodes=[...walk(instance.render(false))],grid=nodes.find(n=>n.kind==='Grid');assert.equal(grid.p.columns,columns);
 const fields=nodes.filter(n=>n.kind==='Canvas'&&n.p.fill);assert.equal(fields.length,capacity);
 const cell=(width-16-(columns-1)*6)/columns;
 for(const field of fields)for(const op of field.draw(cell,field.p.height))for(const [x,y]of op.points||[])assert(x>=0&&x<=cell&&y>=0&&y<=field.p.height,'Compact orbit stays within its cell');
 assert.deepEqual(errors,[]);
}
console.log('Compact orbit density passed: 2/3/4 columns, two rows, bounded rails and no provider actions.');
