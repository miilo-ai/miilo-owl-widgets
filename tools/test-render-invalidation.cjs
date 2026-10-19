// Exercise retained drawing closures rather than rebuilding a tree for each frame.
const fs=require('node:fs'),path=require('node:path'),Module=require('node:module'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..'),runtime=new Module('owl');
runtime._compile(fs.readFileSync(path.join(root,'desktop/runtime/Owl.js'),'utf8').replace(/^\.pragma library/,''),'owl');
const Owl=runtime.exports,errors=[];
const host={theme:()=>0xff88aaff,tile:()=>240,report:e=>errors.push(e),changed(){},act(){throw Error('Unexpected action');},runInto(){throw Error('Unexpected process');}};
const program=Owl.compile(`widget "clock" { size:2x2
 state offset = 1
 component Orbit() { let nested = time * 2 let shifted = nested + offset
 Canvas { width:120; height:80 circle(shifted, 20, 3, fill:#ffffff) }
 Canvas { width:120; height:80 circle(offset, 20, 3, fill:#ffffff) }
 }
 Orbit {}
}`);
const instance=new Owl.Instance(program,host,{}),tree=instance.render(false);
function* walk(n){for(const c of n.c||n.children||[]){yield c;yield*walk(c);}}
const [animated,staticNode]=[...walk(tree)].filter(n=>n.kind==='Canvas');
assert.equal(tree.readsTime,false);
const at0=animated.draw(120,80),still=staticNode.draw(120,80);
for(let n=1;n<=120;n++){
 instance.frame(n/60);
 assert.equal(animated.draw(120,80)[0].args[0],1+n/30,'Nested time-dependent lets refresh without layout render');
 assert.equal(staticNode.draw(120,80),still,'Static display list retains its identity across frames');
}
instance.setState(program.globals.findIndex(g=>g.name==='offset'),7);
assert.equal(animated.draw(120,80)[0].args[0],11,'State changes invalidate retained environments');
assert.equal(staticNode.draw(120,80)[0].args[0],7);
assert.notEqual(animated.draw(130,80),animated.draw(120,80),'Canvas resize invalidates its display list');
assert.notDeepEqual(at0,animated.draw(120,80));assert.deepEqual(errors,[]);
const counterProgram=Owl.compile('widget "counter" { size:1x1 component Counter(){let clock=time Text{text:clock}} Counter{} }');
const counter=new Owl.Instance(counterProgram,host,{});counter.frame(1);
const first=counter.render(false);assert.equal(first.readsTime,true,'Layout time dependencies remain explicit');
counter.frame(2);const second=counter.render(false);
assert.notDeepEqual(first,second,'Time-dependent labels still update');
console.log('Retained canvas clocks, nested time dependencies, static caching, resize and state invalidation passed.');
