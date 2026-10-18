// Exercise the actual widget renderer with controlled presentation time.
const fs = require('node:fs'), path = require('node:path'), Module = require('node:module');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname, '..');
const module_ = new Module('owl');
module_._compile(fs.readFileSync(path.join(root, 'desktop/runtime/Owl.js'), 'utf8').replace(/^\.pragma library/, '//'), 'owl');
const Owl = module_.exports;
const program = Owl.compile(fs.readFileSync(path.join(root, '70-miilo-fleet.owl'), 'utf8'));
function* walk(node) { for (const child of node.c || node.children || []) { yield child; yield* walk(child); } }
function robot({status = 'working', health = 'healthy', phase = 'implementing', saved = {}} = {}) {
    const errors = [];
    const palette = Owl.themeFromToml('');
    const instance = new Owl.Instance(program, {
        theme: name => palette[name] ?? null,
        tile: name => ({linux:true, tileWidth:260, tileHeight:180, screenWidth:720, screenHeight:760, cols:3, rows:2})[name] ?? null,
        report: value => errors.push(value), act() {}, changed() {}, runInto() { throw Error('Animation invoked a provider'); }
    }, saved);
    const slot = name => program.globals.findIndex(global => global.name === name);
    const fixture = {configured:true, errors:[], snapshot:{providers:[{id:'sample',name:'Sample'}],
        actors:[{_key:'agent',id:'agent',providerId:'sample',name:'Agent',status,health}],
        workItems:status === 'working' ? [{_key:'task',providerId:'sample',_actorKeys:['agent'],lifecycle:'active',phase,attention:'none'}] : [], attention:[]}};
    instance.setSource(slot('feed'), Owl.fromJson({ok:true,out:JSON.stringify(fixture),err:'',code:0}));
    function draw(time) {
        instance.frame(time);
        const canvas = [...walk(instance.render(false))].find(node => node.kind === 'Canvas' && !node.p?.fill);
        assert(canvas, 'Robot canvas missing');
        const operations = canvas.draw(canvas.p.width, canvas.p.height);
        assert(operations.length > 0, 'Robot drawing empty');
        assert.deepEqual(errors, [], 'Robot runtime errors');
        return operations;
    }
    return {draw,instance,slot};
}
const still = robot({saved:{robotMotion:'off'}});
assert.deepEqual(still.draw(0), still.draw(27.9), 'Off motion must freeze every drawing channel');
const sleeping = robot({status:'resting',saved:{robotMotion:'reduced'}});
assert.deepEqual(sleeping.draw(1), sleeping.draw(9), 'Reduced sleep marks must stay still');
const active = robot();
assert.notDeepEqual(active.draw(1), active.draw(2), 'Active robot should animate');
const unavailable = robot({health:'unknown'});
assert.deepEqual(unavailable.draw(1), unavailable.draw(2), 'Unavailable robot must not look busy');
const normal = robot({saved:{robotPace:'normal'}}), slow = robot({saved:{robotPace:'slow'}});
assert.deepEqual(normal.draw(0.7), slow.draw(1), 'Pace should scale the animation clock');
const focused = robot({phase:'verifying',saved:{robotMotion:'off'}});
assert.notDeepEqual(focused.draw(0), still.draw(0), 'Verification needs its focused expression');
const failed = robot({status:'failed',saved:{robotMotion:'off'}});
assert.notDeepEqual(failed.draw(0), still.draw(0), 'Failure needs a distinct expression');
const diagonalEyes = ops => ops.filter(op => op.op === 'line' && Math.abs(op.args[0]-op.args[2]) > 0.01 && Math.abs(op.args[1]-op.args[3]) > 0.01);
assert.equal(diagonalEyes(failed.draw(0)).length, 0, 'Healthy failed work preserves round eyes without brows or crosses');
assert.equal(diagonalEyes(robot({health:'unreachable',saved:{robotMotion:'off'}}).draw(0)).length, 4, 'Unavailable system reserves four crossed-eye strokes');
const troubledEyes = failed.draw(0).filter(op => op.op === 'circle' && op.fill && op.args[2] > 0);
assert(troubledEyes.some(a => troubledEyes.some(b => a !== b && Math.abs(a.args[2] / b.args[2] - 1.5) < 0.001)), 'Failed work keeps one larger and one regular round eye');
assert(robot({status:'blocked',saved:{robotMotion:'off'}}).draw(0).some(op=>op.op==='text'&&op.text==='?!'), 'Detailed blocked agents retain their attention mark without an explicit caller override');
assert.equal(failed.draw(0).filter(op=>op.op==='text'&&op.text==='!').length,2,'Detailed failed agents retain both error marks');
const settings = robot({saved:{robotMotion:'off',robotPace:'slow',robotPersonality:'calm'}});
settings.instance.values[settings.slot('setupOpen')] = Owl.fromJson(true);
settings.instance.values[settings.slot('setupSection')] = Owl.fromJson('appearance');
settings.instance.invalidate();
const screen = [...walk(settings.instance.render(true))];
for (const name of ['Calm','Expressive','Full','Reduced','Off']) assert(screen.some(node => node.p.text === name), `Missing ${name} control`);
const full = screen.find(node => node.kind === 'Button' && node.p.text === 'Full');
full.h.onTap(0,0);
assert.equal(settings.instance.values[settings.slot('robotMotion')], 'full', 'Motion control must update state');
assert(program.globals.find(global => global.name === 'robotMotion').persist, 'Motion preference must persist');
console.log('Robot draw tests passed: static motion, sleep, activity, unavailable, pace, expressions and preferences.');
