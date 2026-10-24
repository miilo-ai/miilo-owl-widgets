// Exercise generated brand drawing and About navigation without a helper or network.
const fs = require('node:fs');
const Module = require('node:module');
const assert = require('node:assert/strict');
const {generate} = require('./generate-brand.cjs');
assert.equal(fs.readFileSync('widgets/brand.inc', 'utf8'), generate());
const runtime = new Module('owl');
runtime._compile(fs.readFileSync('desktop/runtime/Owl.js', 'utf8').replace(/^\.pragma library/, ''), 'owl');
const Owl = runtime.exports;
const version = /^version = "([^"]+)"/m.exec(fs.readFileSync('helper/Cargo.toml', 'utf8'))[1];
function* walk(node) { yield node; for (const child of node.c || node.children || []) yield* walk(child); }
let cases = 0;
for (const file of ['70-miilo-fleet.owl', '71-miilo-attention.owl', '72-miilo-board.owl']) {
  const program = Owl.compile(fs.readFileSync(file, 'utf8'));
  for (const width of [320, 393, 600]) for (const light of [false, true]) for (const missing of [false, true]) {
    const actions = [], errors = [], palette = Owl.themeFromToml(light ? 'background="#ffffff"\nforeground="#111111"' : '');
    const instance = new Owl.Instance(program, {
      theme: key => palette[key] ?? null,
      tile: key => ({linux: true, tileWidth: width, tileHeight: 180, screenWidth: width, screenHeight: 800})[key] ?? null,
      report: error => errors.push(error), changed() {}, act: (name, args) => actions.push({name, args}),
      runInto() { assert.fail('Brand/About must not invoke a command'); },
    }, {});
    const slot = key => program.globals.findIndex(g => g.name === key);
    const set = (key, value) => { instance.values[slot(key)] = Owl.fromJson(value); instance.invalidate(); };
    instance.setSource(slot('feed'), Owl.fromJson(missing ? {ok: false, out: '', err: 'missing', code: 127} : {
      ok: true, out: JSON.stringify({configured: false, errors: [], snapshot: {providers: [], actors: [], workItems: [], attention: []}}), code: 0, err: '',
    }));
    const nodes = () => [...walk(instance.render(true))];
    set('setupOpen', true);
    const entry = nodes().find(n => n.kind === 'Box' && n.h?.onTap && [...walk(n)].some(c => c.p?.text === 'About'));
    assert(entry, 'About remains reachable without the helper'); entry.h.onTap(0, 0);
    assert.equal(instance.values[slot('setupSection')], 'about');
    let rendered = nodes();
    assert(rendered.some(n => n.p?.text === `Widget · ${version}`));
    assert(!rendered.some(n => n.kind === 'Terminal' || n.kind === 'Input'));
    assert(rendered.some(n => n.p?.text?.includes('No Miilo account')));
    assert.deepEqual(actions, [], 'Rendering never opens a site or starts installation');
    assert(!rendered.some(n => n.kind === 'Canvas' && n.p.width >= 72 && n.p.width <= 96 && n.p.height === n.p.width * 72 / 184), 'The launcher title must not be duplicated by an internal wordmark');
    for (const leftHanded of [false, true]) {
      set('leftHanded', leftHanded); rendered = nodes();
      const row = rendered.find(n => n.kind === 'Row' && n.c?.some(c => c.p?.text === 'Sponsor Miilo ↗'));
      assert.equal(row.c[0].kind, leftHanded ? 'Button' : 'Spacer');
      assert(row.c.some(c => c.p?.text === 'miilo.tech ↗'), 'Both links share one row');
    }
    rendered.find(n => n.kind === 'Button' && n.p.text === 'miilo.tech ↗').h.onTap(0, 0);
    rendered.find(n => n.kind === 'Button' && n.p.text === 'Sponsor Miilo ↗').h.onTap(0, 0);
    assert.deepEqual(actions, [{name: 'url', args: ['https://miilo.tech/']}, {name: 'url', args: ['https://github.com/sponsors/miilo-ai']}]);
    const back = rendered.find(n => n.kind === 'Box' && n.h?.onTap && n.p.width === 44);
    assert(back); back.h.onTap(0, 0); assert.equal(instance.values[slot('setupSection')], 'menu');
    assert.equal(instance.values[slot('setupOpen')], true);
    assert.deepEqual(errors, []); cases++;
  }
}
console.log(`${cases} brand/About scenarios passed: themes, mobile widths, missing helpers, version, explicit links and Back navigation.`);
