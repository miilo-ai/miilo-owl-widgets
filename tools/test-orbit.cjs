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
// Golden frames come from the approved browser projection, not a second native scene.
const golden = JSON.parse(fs.readFileSync(path.join(root,'tests/orbit-projection.json')));
const near=(a,b,label)=>assert(Math.abs(a-b)<1e-7,`${label}: ${a} != ${b}`);
const projection=scene({count:3,work:3});
for(const expected of golden.frames) {
    const ops=projection.draw(expected.time)[expected.slot];
    const rails=ops.filter(op=>op.op==='path' && op.points.length===65);
    assert.equal(rails.length,16,'Four depth halves each have a core and three glow strokes');
    expected.rails.forEach((rail,index)=>{
        const group=rails.slice(index*4,index*4+4);
        group.forEach(op=>assert.deepEqual(op.points,group[0].points,'Glow and core share geometry'));
        for(const p of rail.points) {
            near(group[0].points[p.index][0],p.x+10,'Rail x');
            near(group[0].points[p.index][1],p.y,'Rail y');
        }
        for(const [x,y] of group[0].points) {
            assert(x>=0&&x<=340&&y>=0&&y<=192,'Ring bounds');
            assert(Math.hypot(x-170,y-96)>=52-1e-7,'Open nucleus clears original robot');
        }
    });
    // The solid task circles are distinguishable from halos/attention outlines.
    const orbs=ops.filter(op=>op.op==='circle' && op.args[2]>3 && op.args[2]<6);
    assert(orbs.length>=3 && orbs.length<=4, 'Three task cores and an optional antenna flash');
    for(const p of expected.orbs) {
        const orb=orbs.find(op=>Math.hypot(op.args[0]-(p.x+10),op.args[1]-p.y)<1e-7);
        assert(orb,'Each task stays on its assigned, projected ring');
        near(orb.args[2],4.5*p.scale,'Orb depth scale');
    }
}
const pathsAt=t=>scene({work:3}).draw(t)[0].filter(op=>op.op==='path'&&op.points.length===65);
for(const lane of [0,1]) assert.notDeepEqual(pathsAt(.2)[lane*4].points,pathsAt(5)[lane*4].points,'Both planes move');
const dormant=scene({work:0});dormant.data.snapshot.actors.forEach(a=>a.status='resting');dormant.update();
assert.deepEqual(dormant.draw(.2),dormant.draw(9),'Dormant rings stop');
const dormantOps=dormant.draw(.2)[0].filter(op=>op.op==='path');
assert.equal(dormantOps.length,4,'Dormant rings have no glow');
assert.notDeepEqual(dormantOps[0].stroke,pathsAt(.2)[3].stroke,'Dormant rail uses muted theme ink');
const filtered=scene({count:3,work:3});
const unfilteredFrame=filtered.draw(5)[2];
filtered.set('system','p2');
assert.deepEqual(filtered.draw(5)[0],unfilteredFrame,'System filtering preserves each provider phase');
const icons=scene().render(1).filter(n=>n.kind==='Canvas'&&n.p?.height===14);
assert(icons.length>0, 'State labels must have discreet icons');
for(const icon of icons) assert(icon.draw(14,14).length>0);
// Zaps use actual active task positions, are bounded and do not alter provider state.
const zapPaths = frame => frame.filter(op=>op.op==='path' && op.points.length===7);
const zapScene=scene({work:3});
assert.equal(zapPaths(zapScene.draw(2.3)[0]).length,0);
const bolt=zapPaths(zapScene.draw(2.49)[0]);
assert.equal(bolt.length,3,'Short burst has a soft halo and fine electrical stroke');
assert.equal(zapPaths(zapScene.draw(2.7)[0]).length,0,'Burst must end');
assert.equal(zapPaths(zapScene.draw(2.49)[1]).length,0,'Systems must stagger');
for(const path of bolt) for(const [x,y] of path.points) assert(x>=0&&x<=340&&y>=0&&y<=192);
for(const motion of ['reduced','off']) assert.equal(zapPaths(scene({motion}).draw(2.49)[0]).length,0);
assert.equal(zapPaths(scene({stale:true}).draw(2.49)[0]).length,0);
const restingZap=scene({work:3});restingZap.data.snapshot.actors[0].status='resting';restingZap.update();
assert.equal(zapPaths(restingZap.draw(2.49)[0]).length,0,'Resting robots must not zap');
const waitingZap=scene({work:3});waitingZap.data.snapshot.workItems.forEach(w=>{w.attention='review';});waitingZap.update();
assert.equal(zapPaths(waitingZap.draw(2.49)[0]).length,0,'Awaiting-action orbs must not zap');
assert.deepEqual(zapScene.commands,[]);
const statusLabels=scene().render(1).filter(n=>n.kind==='Text'&&n.p?.width===78);
assert(statusLabels.length>0&&statusLabels.every(n=>n.p.align==='start'),'State text must align directly beside its icon');
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
assert(many.render(1).some(n=>n.p?.text==='+1 task · inspect system'));
many.set('orbitPage',8);assert.equal(many.draw(.2).length,2);
many.data.snapshot.providers=many.data.snapshot.providers.slice(0,1);many.update();assert.equal(many.draw(.2).length,1,'Page clamps after removal');
const none=scene({count:0});assert(none.render(1).some(n=>n.p?.text==='No systems to display'));
const quiet=scene({view:'quiet'});assert.equal(quiet.draw(1).length,0,'Quiet must retain the current work view');
const toolbar=quiet.render(1).find(n=>n.kind==='Row' && n.c?.some(c=>c.p?.text==='All work'));
assert(toolbar && toolbar.c.at(-1).c.some(c=>c.p?.text==='Orbit'),'View controls must share the filter row');
quiet.render(1).find(n=>n.p?.text==='Orbit').h.onTap(0,0);
assert.equal(quiet.instance.values[quiet.slot('observatoryView')],'orbit');
assert.equal(quiet.draw(1).length,2);
assert(scene({work:1}).render(1).some(n=>n.p?.text==='1 task'),'Singular task count');
assert(scene({work:3}).render(1).some(n=>n.p?.text==='3 tasks'),'Plural task count');

const unsupported=scene({view:'unknown'});assert.equal(unsupported.draw(1).length,0,'Unknown saved views fall back to Quiet');
assert(program.globals.find(g=>g.name==='observatoryView').persist);
console.log('Orbit generated-renderer checks passed: themes, motion, stale state, bounds, pagination, detail selection and no frame I/O.');
