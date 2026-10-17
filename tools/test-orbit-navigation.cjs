const assert=require('node:assert/strict');
module.exports=function testOrbitNavigation(scene) {
// Orbit clusters must expose the same task drill-down as Quiet robots.
for (const work of [0, 1, 9]) {
    const tileScene=scene({count:2,work});
    const tile=tileScene.render(1).find(n=>n.kind==='Column'&&n.h?.onTap&&n.c?.some(c=>c.kind==='Row'&&c.c?.some(t=>t.p?.text==='System 0')));
    assert(tile,'Animated Orbit tile has an activation handler');
    tile.h.onTap(0,0);
    assert.equal(tileScene.instance.values[tileScene.slot('expandedSystem')],'p0');
    assert.equal(tileScene.instance.values[tileScene.slot('observatoryView')],'orbit','Inspection preserves the chosen Orbit preference');
    const detail=tileScene.render(2);
    assert(detail.some(n=>n.p?.text==='← Back to Orbit'),'Tile opens system task list');
    assert(!detail.some(n=>n.p?.text==='Task p1 0'),'Other systems stay excluded');
    assert(detail.some(n=>n.p?.text===(work ? 'Task p0 0' : 'No work in flight')));
    detail.find(n=>n.p?.text==='← Back to Orbit').h.onTap(0,0);
    assert.equal(tileScene.draw(3).length,2,'Back returns to the Orbit tiles');
    assert.deepEqual(tileScene.commands,[],'Navigation must not invoke a provider');
}
// Returning through task details and overflow preserves the entry mode and page.
for (const origin of ['quiet','orbit']) {
    const route=scene({count:8,work:9,view:origin});
    if(origin==='orbit') route.set('orbitPage',1);
    const nodes=route.render(1);
    const entry=origin==='orbit'
      ? nodes.find(n=>n.kind==='Button'&&n.p?.text==='+1 task · inspect system')
      : nodes.find(n=>n.kind==='Column'&&n.h?.onTap&&n.c?.some(c=>c.p?.text==='System 0'));
    assert(entry);entry.h.onTap(0,0);
    const selected=route.instance.values[route.slot('expandedSystem')];
    const row=route.render(2).find(n=>n.kind==='Column'&&n.h?.onTap&&n.c?.some(c=>c.kind==='Row'&&c.c?.some(t=>t.p?.text==='Task '+selected+' 0')));
    assert(row);row.h.onTap(0,0);
    route.render(3).find(n=>n.p?.text==='←'&&n.h?.onTap).h.onTap(0,0);
    assert.equal(route.instance.values[route.slot('expandedSystem')],selected,'Work back returns to the task list');
    // Inspection origin must survive changes to the general display preference.
    route.set('observatoryView',origin==='orbit'?'quiet':'orbit');
    route.render(4).find(n=>n.p?.text===(origin==='orbit'?'← Back to Orbit':'← Systems')).h.onTap(0,0);
    assert.equal(route.instance.values[route.slot('observatoryView')],origin);
    assert.equal(route.instance.values[route.slot('inspectionView')],'');
    assert.equal(route.instance.values[route.slot('expandedSystem')],'');
    if(origin==='orbit') assert.equal(route.instance.values[route.slot('orbitPage')],1,'Return preserves the Orbit page');
    assert.deepEqual(route.commands,[]);
}
};
