// Exercise actual generated OWL with historical sessions preceding current work.
const fs=require('node:fs'),path=require('node:path'),Module=require('node:module');
const assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..'),runtime=new Module('owl');
runtime._compile(fs.readFileSync(path.join(root,'desktop/runtime/Owl.js'),'utf8').replace(/^\.pragma library/,'//'),'owl');
const Owl=runtime.exports;
const source=path.resolve(process.argv[2]||path.join(root,'70-miilo-fleet.owl'));
const program=Owl.compile(fs.readFileSync(source,'utf8'));
function* walk(n){yield n;for(const child of n.c||n.children||[])yield*walk(child);}
function fixture(){
  const providers=Array.from({length:13},(_,i)=>({id:`p${i}`,name:`Session ${i}`,health:'healthy'}));
  const actors=providers.map((p,i)=>({_key:`a${i}`,id:`a${i}`,name:p.name,providerId:p.id,status:i===8?'working':'resting',health:'healthy',extensions:{'dev.miilo.session':{runtimeStatusAuthoritative:false}}}));
  const workItems=providers.map((p,i)=>({_key:`w${i}`,id:`w${i}`,providerId:p.id,title:`Work ${i}`,lifecycle:i===8?'active':'waiting',phase:'implementing',attention:'none',_actorKeys:[`a${i}`],_revisionKey:'r1',actions:[]}));
  return {configured:true,errors:[],snapshot:{providers,actors,workItems,attention:[]}};
}
function scene(data,saved={}){
  const errors=[],palette=Owl.themeFromToml('');
  const instance=new Owl.Instance(program,{
    theme:k=>palette[k]??null,tile:k=>({linux:true,tileWidth:260,tileHeight:180,screenWidth:900,screenHeight:760})[k]??null,
    report:e=>errors.push(e),changed(){},act(){},runInto(){throw Error('Visibility must not invoke a provider');}
  },{observatoryView:'orbit',...saved});
  const slot=k=>program.globals.findIndex(g=>g.name===k);
  const feed=value=>instance.setSource(slot('feed'),Owl.fromJson({ok:true,out:JSON.stringify(value),err:'',code:0}));
  const set=(k,v)=>{instance.values[slot(k)]=Owl.fromJson(v);instance.invalidate();};
  const get=k=>Owl.toJson(instance.read(slot(k)));
  const render=()=>{const nodes=[...walk(instance.render(true))];assert.deepEqual(errors,[]);return nodes;};
  feed(data);return {instance,feed,set,get,render};
}
const data=fixture(),s=scene(data);
assert.equal(s.get('allProviders').length,13,'Enrollment inventory must stay intact');
assert.deepEqual(s.get('actors').map(a=>a._key),['a8'],'Observed working session must be visible despite original ninth position');
assert.deepEqual(s.get('selectableProviders').map(p=>p.id),['p8']);
assert.equal(s.get('allActors').filter(a=>a.status==='resting').length,12,'Never fake activity');
assert.equal(s.get('provs').length,1);
assert.equal(s.render().filter(n=>n.kind==='Canvas'&&n.p.height===164).length,1,'Only visible session gets an Orbit field');
assert.equal(s.get('systemsOpen'),false,'Menu starts collapsed');
const menu=s.render().filter(n=>n.kind==='Box'&&n.p?.width===44&&n.h?.onTap&&(n.c||n.children||[]).some(c=>c.kind==='Canvas'&&c.p.width===24)).at(-1);
assert(menu,'Menu can be explicitly opened');menu.h.onTap(0,0);
assert.equal(s.get('setupOpen'),true);
assert.equal(s.get('setupSection'),'filters');
let nodes=s.render();
assert(nodes.some(n=>n.p?.text==='Session 8'));
assert(!nodes.some(n=>n.p?.text==='Session 0'),'Inactive sessions stay out of selector');
assert(!nodes.some(n=>String(n.p?.text||'').includes('inactive sessions')),'No inactive toggle');
const sleeping=fixture();sleeping.snapshot.actors[0].extensions['dev.miilo.session'].recentlyActive=true;
const recent=scene(sleeping,{showInactiveSessions:true});
assert.deepEqual(recent.get('provs').map(p=>p.id),['p8','p0']);
assert.equal(recent.get('actors')[1].status,'resting','Recent idle sessions keep sleeping animation');
assert.equal(recent.instance.persisted().showInactiveSessions,undefined,'Legacy reveal preference is ignored');
recent.set('setupOpen',true);recent.set('setupSection','filters');
assert(recent.render().some(n=>n.p?.text==='✓ All systems'),'Selected all-systems state is explicit');
recent.set('system','p0');
recent.render().find(n=>n.kind==='Box'&&n.h?.onTap&&[...walk(n)].some(c=>c.p?.text==='All systems')).h.onTap(0,0);
assert.equal(recent.get('system'),'','Clear-filter action works');
recent.set('setupOpen',false);s.set('setupOpen',false);
sleeping.snapshot.actors[0].extensions['dev.miilo.session'].recentlyActive=false;
recent.feed(sleeping);assert.deepEqual(recent.get('provs').map(p=>p.id),['p8'],'Session expires on next snapshot');
s.set('selectedWork','w8');
const ended=fixture();ended.snapshot.actors[8].status='resting';ended.snapshot.workItems[8].lifecycle='waiting';
s.feed(ended);
assert.equal(s.get('provs').length,0);
assert.equal(s.get('focused')._key,'w8','Do not discard an open work detail when activity ends');
s.set('selectedWork','');
assert(s.render().some(n=>n.p?.text==='No systems to display'));
const resumed=fixture();resumed.snapshot.actors[2].status='reviewing';s.feed(resumed);
assert.deepEqual(s.get('provs').map(p=>p.id),['p2','p8'],'Fresh activity rejoins automatically');
const needsYou=fixture();needsYou.snapshot.workItems[0].attention='approval';
assert.deepEqual(scene(needsYou).get('provs').map(p=>p.id),['p0','p8'],'Pending decisions must not disappear');
const offline=fixture();offline.snapshot.actors[8].health='unreachable';
assert.equal(scene(offline).get('provs').length,0,'Unavailable session must not be inferred active');
const remote=fixture();remote.snapshot.actors[0].extensions={};
assert.deepEqual(scene(remote).get('provs').map(p=>p.id),['p8','p0'],'Idle persistent remote factories stay accessible');
const quiet=scene(data,{observatoryView:'quiet'});assert.equal(quiet.get('actors').length,1);quiet.render();
console.log('Session visibility passed: active-first, 12-hour sleeping visibility, lifecycle updates, decisions, remote factories and no provider commands.');

const paired = fixture(); paired.snapshot.actors[0]._pairedComputer = true;
const pairedScene = scene(paired, {});
assert(pairedScene.get("provs").some(p=>p.id === "p0"), "Explicitly paired idle sessions remain visible without inventing activity");
assert.equal(pairedScene.get("actors").find(a=>a.id === "a0").status,"resting");

const activity = fixture(); activity.snapshot.workItems[8].extensions={"dev.miilo.session":{}}; activity.snapshot.workItems[8].summary="Checking the latest observation";
const activityScene = scene(activity, {}); activityScene.set("expandedSystem","p8");
assert(activityScene.render().some(n=>n.kind === "Text" && n.p.text === "Checking the latest observation"), "Session rows show their latest summary instead of repeating the session title");
