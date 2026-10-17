// Exercise generated OWL, never a second scene implementation.
const fs = require('node:fs'), path = require('node:path'), Module = require('node:module');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname, '..'), module_ = new Module('owl');
module_._compile(fs.readFileSync(path.join(root,'desktop/runtime/Owl.js'),'utf8').replace(/^\.pragma library/,'//'),'owl');
const Owl = module_.exports, program = Owl.compile(fs.readFileSync(path.join(root,'70-miilo-fleet.owl'),'utf8'));
function* walk(n) { yield n; for (const c of n.c || n.children || []) yield* walk(c); }
function scene({count=2, work=9, motion='full', stale=false, light=false, view='orbit'}={}) {
    const errors=[], commands=[], palette=Owl.themeFromToml(light?'background="#ffffff"\nforeground="#111111"\naccent="#9c245b"':'accent="#58c4b8"');
    const host={theme:k=>palette[k]??null,tile:k=>({linux:true,tileWidth:260,tileHeight:180,screenWidth:900,screenHeight:760})[k]??null,
        report:e=>errors.push(e),changed(){},act(){},runInto:c=>commands.push(c)};
    const instance=new Owl.Instance(program,host,{observatoryView:view,robotMotion:motion});
    const slot=k=>program.globals.findIndex(g=>g.name===k);
    const set=(k,v)=>{instance.values[slot(k)]=Owl.fromJson(v);instance.invalidate();};
    const providers=Array.from({length:count},(_,i)=>({id:`p${i}`,name:`System ${i}`,health:stale?'stale':'healthy'}));
    const actors=providers.map(p=>({_key:p.id+'a',id:'a',providerId:p.id,name:'Builder',status:'working',health:'healthy'}));
    const workItems=providers.flatMap(p=>Array.from({length:work},(_,i)=>({_key:`${p.id}w${i}`,id:`w${i}`,providerId:p.id,title:`Task ${p.id} ${i}`,
        _actorKeys:[p.id+'a'],lifecycle:'active',phase:'implementing',attention:i===0?'review':'none',actions:[],_revisionKey:'r1'})));
    const data={configured:true,errors:[],snapshot:{providers,actors,workItems,attention:[]}};
    const update=()=>instance.setSource(slot('feed'),Owl.fromJson({ok:true,out:JSON.stringify(data),code:0,err:''}));update();
    const render=t=>{instance.frame(t);const tree=instance.render(true);assert.deepEqual(errors,[]);return [...walk(tree)];};
    const draw=t=>render(t).filter(n=>n.kind==='Canvas'&&n.p?.height===192).map(n=>n.draw(340,192));
    return {instance,slot,set,data,update,render,draw,commands,errors};
}
for(const light of [false,true]) {
    const s=scene({light});const a=s.draw(.2), b=s.draw(1.4);
    assert.equal(a.length,2);assert.notDeepEqual(a,b,'Full scene should move');
    for(const frame of [...a,...b])for(const op of frame)for(const arg of op.args||[]) {
        const check=v=>{if(typeof v==='number')assert(Number.isFinite(v));else if(Array.isArray(v))v.forEach(check);};check(arg);
    }
    const nodes=s.render(1), row=nodes.find(n=>n.kind==='Row'&&n.h?.onTap&&(n.c||[]).some(c=>c.p.text==='Task p0 0'));
    assert(row,'Stationary work selection missing'); row.h.onTap(0,0);
    assert.equal(s.instance.values[s.slot('selectedWork')],'p0w0');
    assert(s.render(2).some(n=>n.p?.text==='Task p0 0'),'Existing detail must open');
    assert.deepEqual(s.commands,[],'Drawing/selection must not invoke a provider');
}
for(const motion of ['reduced','off']) {
    const s=scene({motion});assert.deepEqual(s.draw(.2),s.draw(9),'Reduced/off scene must be static');
}
const stale=scene({stale:true});assert.deepEqual(stale.draw(.2),stale.draw(9),'Stale geometry must stop');
assert(stale.render(1).some(n=>n.p?.text==='Unavailable'));
const many=scene({count:50});assert.equal(many.draw(.2).length,6,'Page must bound clusters');
assert(many.render(1).some(n=>n.p?.text==='+1 work · inspect system'));
many.set('orbitPage',8);assert.equal(many.draw(.2).length,2);
many.data.snapshot.providers=many.data.snapshot.providers.slice(0,1);many.update();assert.equal(many.draw(.2).length,1,'Page clamps after removal');
const none=scene({count:0});assert(none.render(1).some(n=>n.p?.text==='No systems to display'));
const quiet=scene({view:'quiet'});assert.equal(quiet.draw(1).length,0,'Quiet must retain the current work view');
const unsupported=scene({view:'unknown'});assert.equal(unsupported.draw(1).length,0,'Unknown saved views fall back to Quiet');
assert(program.globals.find(g=>g.name==='observatoryView').persist);
console.log('Orbit generated-renderer checks passed: themes, motion, stale state, bounds, pagination, detail selection and no frame I/O.');
