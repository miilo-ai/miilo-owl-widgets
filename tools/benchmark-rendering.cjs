// Deterministic evaluator benchmark; native paint and GPU time are separate gates.
const fs=require('node:fs'),path=require('node:path'),Module=require('node:module'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..'),runtimeFile=process.argv[2]||path.join(root,'desktop/runtime/Owl.js'),sourceFile=process.argv[3]||path.join(root,'70-miilo-fleet.owl');
const runtime=new Module('owl');runtime._compile(fs.readFileSync(runtimeFile,'utf8').replace(/^\.pragma library/,''),'owl');
const Owl=runtime.exports,program=Owl.compile(fs.readFileSync(sourceFile,'utf8')),palette=Owl.themeFromToml(''),errors=[];
const instance=new Owl.Instance(program,{theme:k=>palette[k]??null,tile:k=>({linux:true,tileWidth:580,tileHeight:620,screenWidth:580,screenHeight:620})[k]??null,report:e=>errors.push(e),changed(){},act(){throw Error('Unexpected action');},runInto(){throw Error('Unexpected I/O');}},{observatoryView:'orbit'});
const actors=Array.from({length:4},(_,i)=>({_key:'a'+i,id:'a'+i,providerId:'p'+i,name:'Sample '+i,status:'working',health:'healthy'}));
const feed={configured:true,errors:[],snapshot:{providers:actors.map(a=>({id:a.providerId,name:a.name,health:'healthy'})),actors,workItems:actors.flatMap(a=>Array.from({length:3},(_,i)=>({_key:a.id+'w'+i,id:a.id+'w'+i,title:'Inspect sample '+i,providerId:a.providerId,_actorKeys:[a._key],lifecycle:'active',phase:'implementing',attention:'none'}))),attention:[]}};
instance.setSource(program.globals.findIndex(g=>g.name==='feed'),Owl.fromJson({ok:true,out:JSON.stringify(feed),code:0,err:''}));
function* walk(n){for(const c of n.c||n.children||[]){yield c;yield*walk(c);}}
const forced=process.env.MIILO_BENCH_FORCE_LAYOUT==='1',times=[];let renders=0,tree=null,totalOps=0;
for(let f=0;f<150;f++){
 const start=performance.now();instance.frame(f/60);
 if(!tree||tree.readsTime||forced){tree=instance.render(Boolean(program.screen && program.screen.length));renders++;}
 for(const c of walk(tree))if(c.kind==='Canvas')totalOps+=c.draw(c.p.width||260,c.p.height||164).length;
 if(f>=30)times.push(performance.now()-start);
}
assert.deepEqual(errors,[]);times.sort((a,b)=>a-b);
console.log(JSON.stringify({frames:150,layoutRenders:renders,medianEvaluatorMs:Number(times[Math.floor(times.length/2)].toFixed(3)),p95EvaluatorMs:Number(times[Math.floor(times.length*.95)].toFixed(3)),totalOps}));
