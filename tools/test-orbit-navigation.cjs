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
    assert(detail.some(n=>n.p?.text==='Back'),'Tile opens system task list');
    assert(!detail.some(n=>n.p?.text==='Task p1 0'),'Other systems stay excluded');
    assert(detail.some(n=>n.p?.text===(work ? 'Task p0 0' : 'No work in flight')));
    detail.find(n=>n.p?.text==='Back').h.onTap(0,0);
    assert.equal(tileScene.draw(3).length,2,'Back returns to the Orbit tiles');
    assert.deepEqual(tileScene.commands,[],'Navigation must not invoke a provider');
}
// Detail means the overview even when Compact has an open system list.
for (const linux of [true,false]) {
    const route=scene({count:2,work:3,view:'quiet',linux,screenWidth:393});
    const compact=route.render(1).find(n=>n.kind==='Column'&&n.h?.onTap&&n.c?.some(c=>c.p?.text==='System 0'));
    assert(compact);compact.h.onTap(0,0);
    assert.equal(route.instance.values[route.slot('expandedSystem')],'p0');
    const detail=route.render(2).find(n=>n.kind==='Box'&&n.h?.onTap&&n.c?.some(c=>c.p?.text==='Detail'));
    assert(detail);detail.h.onTap(0,0);
    assert.equal(route.instance.values[route.slot('expandedSystem')],'','First Detail activation clears the old drill-down');
    assert.equal(route.render(3).filter(n=>n.kind==='Column'&&n.h?.onTap&&n.c?.some(c=>c.kind==='Row'&&c.c?.some(t=>t.p?.text==='System 0'))).length,1);
    assert.deepEqual(route.commands,[]);
}
// Density follows viewport width, including Android guests reporting Linux.
for (const linux of [true,false]) for (const width of [320,360,393,440]) {
    const mobile=scene({count:4,work:3,linux,screenWidth:width});
    const nodes=mobile.render(1),grid=nodes.find(n=>n.kind==='Grid');
    assert.equal(grid.p.columns,2,'Normal phone widths fit two Detail cards');
    const fields=nodes.filter(n=>n.kind==='Canvas'&&n.p?.height===92);
    assert.equal(fields.length,4);
    const cell=(grid.p.width-10)/2-16;
    for(const field of fields) for(const op of field.draw(cell,92)) for(const [x,y]of op.points||[])
        assert(x>=0&&x<=cell&&y>=0&&y<=92,'Compact Detail rings remain inside their cards');
    assert.deepEqual(mobile.commands,[]);
}
// Both fleets center their grid, including content narrower than a wide viewport.
for (const view of ['quiet','orbit']) for (const screenWidth of [320,393,900]) {
    const layout=scene({count:1,work:1,view,screenWidth}).render(1);
    const container=layout.find(n=>n.kind==='Box'&&n.p.fill==='width'&&n.p.align==='top'&&n.c?.some(c=>c.kind==='Grid'));
    assert(container,'Both view modes must center the grid within available content');
    assert(container.c.find(c=>c.kind==='Grid').p.width<=screenWidth);
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
    route.render(3).find(n=>n.kind==='Box'&&n.p?.width===44&&n.h?.onTap&&n.c?.some(c=>c.kind==='Canvas'&&c.p.width===24)).h.onTap(0,0);
    assert.equal(route.instance.values[route.slot('expandedSystem')],selected,'Work back returns to the task list');
    // Inspection origin must survive changes to the general display preference.
    route.set('observatoryView',origin==='orbit'?'quiet':'orbit');
    route.render(4).find(n=>n.p?.text==='Back').h.onTap(0,0);
    assert.equal(route.instance.values[route.slot('observatoryView')],origin);
    assert.equal(route.instance.values[route.slot('inspectionView')],'');
    assert.equal(route.instance.values[route.slot('expandedSystem')],'');
    if(origin==='orbit') assert.equal(route.instance.values[route.slot('orbitPage')],1,'Return preserves the Orbit page');
    assert.deepEqual(route.commands,[]);
}
};
