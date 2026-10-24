const fs = require('node:fs'), Module = require('node:module'), assert = require('node:assert/strict');
const runtime = new Module('owl');
runtime._compile(fs.readFileSync('desktop/runtime/Owl.js', 'utf8').replace(/^\.pragma library/, ''), 'owl');
const Owl = runtime.exports;
const program = Owl.compile(fs.readFileSync('70-miilo-fleet.owl', 'utf8'));
function* walk(node) { yield node; for (const child of node.c || node.children || []) yield* walk(child); }
function fleet(count, width, height) {
  const palette = Owl.themeFromToml(''), errors = [];
  const instance = new Owl.Instance(program, {
    theme: key => palette[key] ?? null,
    tile: key => ({ linux: true, tileWidth: width, tileHeight: height, screenWidth: width, screenHeight: height })[key] ?? null,
    changed() {}, report: error => errors.push(error), act() { assert.fail('Layout cannot trigger actions'); }, runInto() { assert.fail('Layout cannot run commands'); },
  }, { observatoryView: 'quiet' });
  const providers = Array.from({length: count}, (_, i) => ({id: 'p' + i, name: 'System ' + i, health: 'healthy'}));
  instance.setSource(program.globals.findIndex(g => g.name === 'feed'), Owl.fromJson({ok: true, code: 0, err: '', out: JSON.stringify({configured: true, errors: [], snapshot: {
    providers, actors: providers.map(p => ({_key: p.id, id: p.id, providerId: p.id, name: p.name, status: 'working', health: 'healthy'})),
    workItems: providers.map(p => ({_key: p.id + 'w', id: p.id + 'w', providerId: p.id, lifecycle: 'active', attention: 'none', _actorKeys: [p.id], actions: []})), attention: [],
  }})}));
  instance.values[program.globals.findIndex(g => g.name === 'providerLimit')] = count; instance.invalidate();
  const nodes = [...walk(instance.render(true))]; assert.deepEqual(errors, []);
  return { grid: nodes.find(n => n.kind === 'Grid'), portraits: nodes.filter(n => n.kind === 'Canvas' && n.p.fill) };
}
for (const width of [240, 360, 600, 960, 1600]) for (const height of [400, 800]) for (const count of [1, 2, 3, 5, 13, 20]) {
  const {grid, portraits} = fleet(count, width, height);
  assert(grid && grid.p.columns >= 1 && grid.p.columns <= 4 && grid.p.columns <= count);
  assert.equal(portraits.length, count, 'Density changes must not hide enrolled systems');
  assert(grid.p.width <= width, 'Compact fleet must fit the viewport');
  for (const portrait of portraits) {
    assert(Number.isFinite(portrait.p.height) && portrait.p.height >= 24);
    assert(portrait.p.height <= height * 0.24, 'Portraits respect the available viewport height');
    assert(portrait.p.height <= (width - 24 - (grid.p.columns - 1) * 6) / grid.p.columns, 'Portraits fit their columns');
  }
}
assert(fleet(2, 960, 1000).portraits[0].p.height > fleet(20, 960, 1000).portraits[0].p.height, 'Small fleets grow while dense fleets stay compact');
console.log('Adaptive fleet passed: 60 viewport/population cases, column/height bounds, no hidden systems or actions.');
