const fs=require('node:fs'),Module=require('node:module'),assert=require('node:assert/strict');
const runtime=new Module('owl');runtime._compile(fs.readFileSync('desktop/runtime/Owl.js','utf8').replace(/^\.pragma library/,''),'owl');
const Owl=runtime.exports;
const program=Owl.compile(fs.readFileSync('70-miilo-fleet.owl','utf8')),actions=[],errors=[];
const instance=new Owl.Instance(program,{theme:()=>null,tile:k=>({linux:true,tileWidth:393,tileHeight:180,screenWidth:393,screenHeight:800})[k]??null,report:e=>errors.push(e),changed(){},act:(...a)=>actions.push(a),runInto:()=>assert.fail('Rendering must not run installer')},{});
const slot=k=>program.globals.findIndex(g=>g.name===k);
instance.values[slot('setupOpen')]=true;
instance.setSource(slot('feed'),Owl.fromJson({ok:false,out:'',code:127,err:'helper missing'}));
function* walk(n){yield n;for(const c of n.c||n.children||[])yield*walk(c);}
let nodes=[...walk(instance.render(true))];
assert(nodes.some(n=>n.p?.text==='Miilo helper required'));
assert.equal(actions.length,0);
const install=nodes.find(n=>n.kind==='Button'&&n.p?.text==='Install helper & continue');assert(install);
install.h.onTap(0,0);assert.equal(instance.values[slot('helperInstalling')],true);
assert.equal(actions.length,1);assert.equal(errors.length,0);
console.log('Helper disclosure and explicit installation action passed.');
