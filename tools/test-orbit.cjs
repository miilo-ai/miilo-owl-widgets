// Exercise generated OWL, never a second scene implementation.
const fs = require('node:fs'), path = require('node:path'), Module = require('node:module');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname, '..'), module_ = new Module('owl');
module_._compile(fs.readFileSync(path.join(root,'desktop/runtime/Owl.js'),'utf8').replace(/^\.pragma library/,'//'),'owl');
const Owl = module_.exports, program = Owl.compile(fs.readFileSync(path.join(root,'70-miilo-fleet.owl'),'utf8'));
function* walk(n) { yield n; for (const c of n.c || n.children || []) yield* walk(c); }
function scene({count=2, work=9, motion='full', stale=false, light=false, view='orbit', linux=true, screenWidth=900}={}) {
    const errors=[], commands=[], palette=Owl.themeFromToml(light?'background="#ffffff"\nforeground="#111111"\naccent="#9c245b"\ngreen="#206844"\nred="#a12635"':'accent="#58c4b8"\ngreen="#9ece6a"\nred="#f7768e"');
    const host={theme:k=>palette[k]??null,tile:k=>({linux,tileWidth:260,tileHeight:180,screenWidth,screenHeight:760})[k]??null,
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
    const draw=t=>render(t).filter(n=>n.kind==='Canvas'&&n.p?.height===164).map(n=>n.draw(340,164));
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
            near(group[0].points[p.index][0],170+(p.x-160)*.85,'Rail x');
            near(group[0].points[p.index][1],82+(p.y-96)*.85,'Rail y');
        }
        for(const [x,y] of group[0].points) {
            assert(x>=0&&x<=340&&y>=0&&y<=164,'Ring bounds');
            assert(Math.hypot(x-170,y-82)>=52*.85-1e-7,'Open nucleus clears original robot');
        }
    });
    // The solid task circles are distinguishable from halos/attention outlines.
    const orbs=ops.filter(op=>op.op==='circle' && op.args[2]>3 && op.args[2]<6);
    assert(orbs.length>=3 && orbs.length<=4, 'Three task cores and an optional antenna flash');
    for(const p of expected.orbs) {
        const orb=orbs.find(op=>Math.hypot(op.args[0]-(170+(p.x-160)*.85),op.args[1]-(82+(p.y-96)*.85))<1e-7);
        assert(orb,'Each task stays on its assigned, projected ring');
        near(orb.args[2],4.5*p.scale*.85,'Orb depth scale');
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
// Attention marks follow system work, including tasks outside the eight-orb cap.
const marks=s=>s.render(1).filter(n=>n.kind==='Canvas'&&n.p?.width===95.2).flatMap(n=>n.draw(95.2,95.2)).filter(op=>op.op==='text').map(op=>op.text);
const attentionScene=scene({count:1});
assert(marks(attentionScene).includes('?!'),'Needs-you systems must show their attention mark');
attentionScene.data.snapshot.workItems[8].lifecycle='failed';attentionScene.update();
assert.deepEqual(marks(attentionScene).filter(t=>t==='!'),['!','!'],'Failures beyond visible orbs take priority with !!');
assert(!marks(attentionScene).includes('?!'));
const clearScene=scene({count:1,work:3});clearScene.data.snapshot.workItems.forEach(w=>w.attention='none');clearScene.update();
assert(!marks(clearScene).some(t=>t==='?!'||t==='!'),'Working systems must not retain attention marks');
assert(!marks(scene({count:1,stale:true})).some(t=>t==='?!'||t==='!'),'Stale systems cannot advertise actionable marks');
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
assert(!scene().render(1).some(n=>n.kind==='Text'&&n.p?.width===78),'Orbit must not repeat state labels beneath titles');
for(const light of [false,true]) {
    const s=scene({light});const a=s.draw(.2), b=s.draw(1.4);
    assert.equal(a.length,2);assert.notDeepEqual(a,b,'Full scene should move');
    for(const frame of [...a,...b])for(const op of frame)for(const arg of op.args||[]) {
        const check=v=>{if(typeof v==='number')assert(Number.isFinite(v));else if(Array.isArray(v))v.forEach(check);};check(arg);
    }
    const nodes=s.render(1), row=nodes.find(n=>n.kind==='Row'&&n.h?.onTap&&[...walk(n)].some(c=>c.p?.text==='Task p0 0'));
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
require('./test-orbit-navigation.cjs')(scene);
const quiet=scene({view:'quiet'});assert.equal(quiet.draw(1).length,0,'Compact fields use content-sized geometry');
const quietBots = s => s.render(1).filter(n => n.kind === 'Canvas' && n.p?.height > 25 && !n.p?.fill);
assert.equal(quietBots(quiet).length, 2, 'Quiet initially shows the robot fleet');
assert(!quiet.render(1).some(n=>n.p?.text==='Task p0 0'), 'Task lists stay behind robot selection');
assert(quietBots(quiet)[0].draw(58,58).some(op=>op.op==='text'&&op.text==='?!'), 'Quiet aggregates attention');
const noActors=scene({view:'quiet'});noActors.data.snapshot.actors=[];noActors.update();
assert.equal(quietBots(noActors).length,2,'Provider-only sources retain system robots');
const compactQuiet=scene({view:'quiet',screenWidth:320});
assert.equal(compactQuiet.render(1).filter(n=>n.kind==='Canvas'&&n.p?.height > 25 && !n.p?.fill).length,2,'Narrow fleet keeps smaller robots');
const quietOffline=scene({view:'quiet',stale:true});
assert(!quietBots(quietOffline)[0].draw(88,88).some(op=>op.op==='text'&&['?!','!'].includes(op.text)),'Stale fleets suppress attention');
const robotTile=quiet.render(1).find(n=>n.kind==='Column'&&n.h?.onTap&&n.c?.some(c=>c.p?.text==='System 0'));
robotTile.h.onTap(0,0);
assert.equal(quiet.instance.values[quiet.slot('expandedSystem')],'p0','Robot selects its system');
assert(quiet.render(1).some(n=>n.p?.text==='Task p0 0'),'Selected robot opens its task list');
assert(!quiet.render(1).some(n=>n.p?.text==='Task p1 0'),'Other systems stay outside the selected list');
quiet.render(1).find(n=>n.p?.text==='Back').h.onTap(0,0);
assert.equal(quietBots(quiet).length,2,'Back returns to the robot fleet');
const toolbar=quiet.render(1).find(n=>n.kind==='Row' && n.c?.some(c=>c.c?.some(t=>t.p?.text==='All work')));
assert(toolbar && toolbar.c.at(-1).c.some(c=>c.c?.some(t=>t.p?.text==='Detail')),'View controls must share the filter row');
quiet.render(1).find(n=>n.kind==='Box'&&n.c?.some(t=>t.p?.text==='Detail')).h.onTap(0,0);
assert.equal(quiet.instance.values[quiet.slot('observatoryView')],'orbit');
assert.equal(quiet.draw(1).length,2);
assert(scene({work:1}).render(1).some(n=>n.p?.text==='1 task'),'Singular task count');
assert(scene({work:3}).render(1).some(n=>n.p?.text==='3 tasks'),'Plural task count');

const unsupported=scene({view:'unknown'});assert.equal(unsupported.draw(1).length,0,'Unknown saved views fall back to Quiet');
assert(program.globals.find(g=>g.name==='observatoryView').persist);
console.log('Orbit generated-renderer checks passed: themes, motion, stale state, bounds, pagination, detail selection and no frame I/O.');

for(const [width,columns]of [[360,1],[440,1],[540,2],[600,2],[684,3],[900,3],[1400,3]]) {
 const grid=scene({count:3,screenWidth:width}).render(1).find(n=>n.kind==='Grid' && n.c?.some(c=>c.kind==='Column'));
 assert(grid,'Orbit grid must render');assert.equal(grid.p.columns,columns,'Content-based column count at '+width);
 assert(grid.p.width<=width && grid.p.width<=1010,'Tile group stays bounded');
}
assert.equal(scene({count:1,screenWidth:1400}).render(1).find(n=>n.kind==='Grid').p.width,330,'One system must not stretch across the desktop');

// A Linux environment can be the Android PRoot; both retain touch targets.
for(const [linux,height]of [[true,44],[false,44]]) {
 const nodes=scene({count:3,work:3,screenWidth:684,linux}).render(1);
 const rows=nodes.filter(n=>n.kind==='Row'&&n.h?.onTap&&[...walk(n)].some(c=>c.p?.text?.startsWith('Task p')));
 assert.equal(rows.length,9);assert(rows.every(n=>n.p.height===height));
 const grid=nodes.find(n=>n.kind==='Grid');const cell=(grid.p.width-20)/3-20;
 for(const field of nodes.filter(n=>n.kind==='Canvas'&&n.p?.height===164)) {
  for(const op of field.draw(cell,164))for(const [x,y] of op.points||[])assert(x>=0&&x<=cell&&y>=0&&y<=164,'Narrow rails stay inside their own tile');
 }
}
// Smooth hump sampling is independent of panel width; only the orb breathes.
for(const width of [260,684,1440]) {
 const s=scene({count:1,work:1});s.set('selectedWork','p0w0');
 const draw=t=>s.render(t).find(n=>n.kind==='Canvas'&&n.p?.height===25).draw(width,25);
 const first=draw(.2),later=draw(1.4),rail=first.find(op=>op.op==='path');
 assert.deepEqual(rail,later.find(op=>op.op==='path'),'The curve must stay still');
 const orb=first.find(op=>op.op==='circle');
 const hump=rail.points.filter(p=>Math.abs(p[0]-orb.args[0])<=18.001);
 assert.equal(hump.length,65,'The hump keeps all local samples even on wide panels');
 for(const [x,y]of hump)assert(x>=0&&x<=width&&y<=17&&y>=9.5);
 assert(Math.min(...hump.map(p=>Math.hypot(p[0]-orb.args[0],p[1]-17)))-3.8-.75>2.8,'Curve clears the maximum pulsing orb');
}
// Unified diff markers color additions/removals without coloring ordinary output.
const details=scene({count:1,work:1});
details.data.snapshot.workItems[0]._artifactKeys=['diff'];
details.data.snapshot.artifacts=[{_key:'diff',name:'demo.ts',kind:'change_set',_url:'',_previewTruncated:false,_preview:'--- a/demo.ts\n+++ b/demo.ts\n-old\n+new\n unchanged'}];details.update();details.set('selectedWork','p0w0');
const diffText=details.render(1).filter(n=>n.kind==='Text');
const color=t=>diffText.find(n=>n.p?.text===t)?.p.color;
assert(color('-old')&&color('+new')&&color(' unchanged'));
assert.notEqual(color('-old'),color('+new'));
assert.notEqual(color('+new'),color(' unchanged'));
assert.equal(color('--- a/demo.ts'),color('+++ b/demo.ts'),'File headers share neutral accent ink');
assert.deepEqual(details.commands,[],'Inspecting a diff must remain read-only');
