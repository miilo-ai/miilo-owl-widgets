const assert=require('node:assert/strict');
module.exports=scene=>{
  function crossedEyes(health,withActor=false){
    const s=scene({count:1,work:0,motion:'off'});
    if(!withActor)s.data.snapshot.actors=[];
    else s.data.snapshot.actors[0].health='unknown';
    if(health===undefined)delete s.data.snapshot.providers[0].health;
    else s.data.snapshot.providers[0].health=health;
    s.update();
    const canvas=s.render(1).find(n=>n.kind==='Canvas'&&n.p.height>25&&n.p.width===n.p.height&&!n.p.fill);
    assert(canvas,'Provider robot remains visible without actor records');
    const lines=canvas.draw(canvas.p.width,canvas.p.height).filter(op=>op.op==='line'&&Math.abs(op.args[0]-op.args[2])>0.01&&Math.abs(op.args[1]-op.args[3])>0.01);
    assert.equal(s.data.snapshot.workItems.length,0,'Health fallback must not invent work');
    assert.deepEqual(s.commands,[],'Health fallback must not invoke providers');
    return lines.length;
  }
  assert.equal(crossedEyes('healthy'),0,'A healthy provider without actors must not look unavailable');
  for(const health of [undefined,'unknown','unreachable','stale'])
    assert.equal(crossedEyes(health),4,'Missing or unavailable provider health remains explicit');
  assert.equal(crossedEyes('healthy',true),4,'Provider readiness must not override an unavailable actor');
  console.log('Provider-only Detail health passed without invented work or hidden unavailable actors.');
};
