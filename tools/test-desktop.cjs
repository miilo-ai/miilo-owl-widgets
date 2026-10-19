// Exercise actual OWL handlers through the vendored Linux evaluator.
const fs = require('node:fs'), path = require('node:path'), Module = require('node:module'), assert = require('node:assert/strict');
const root = path.resolve(__dirname, '..');
const module_ = new Module('owl');
module_._compile(fs.readFileSync(path.join(root, 'desktop/runtime/Owl.js'), 'utf8').replace(/^\.pragma library/, '//'), 'owl');
const Owl = module_.exports;
function* walk(tree) { for (const n of tree.children || tree.c || []) { yield n; yield* walk(n); } }
function text(tree) { return [...walk(tree)].map(n => n.p.text || '').join('\n'); }
let cases = 0;
for (const file of fs.readdirSync(process.argv[2]).filter(f => f.endsWith('.json'))) {
    const data = JSON.parse(fs.readFileSync(path.join(process.argv[2], file)));
    for (const light of [false, true]) for (const [width,height] of [[120,64],[260,180],[720,760]]) {
        const palette = Owl.themeFromToml(light ? 'background = "#ffffff"\nforeground = "#111111"' : '');
        const problems = [], commands = [];
        const program = Owl.compile(fs.readFileSync(path.join(root,'70-miilo-fleet.owl'),'utf8'));
        const instance = new Owl.Instance(program, {
            theme: n => palette[n] ?? null,
            tile: n => ({ linux:true, cols:3, rows:2, tileWidth:width, tileHeight:height, screenWidth:720, screenHeight:760, corners:0 })[n] ?? null,
            act() {}, runInto: cmd => commands.push(cmd), report: m => problems.push(m), changed() {}
        }, {});
        const slot = name => program.globals.findIndex(g => g.name === name);
        const set = (name, value) => { instance.values[slot(name)] = Owl.fromJson(value); instance.invalidate(); };
        instance.setSource(slot('feed'), Owl.fromJson({ok:true,out:JSON.stringify(data),err:'',code:0}));
        assert(instance.render(false).children.length);
        assert(instance.render(true).children.length);
        const work = data.snapshot.workItems?.[0];
        if (work) {
            set('selectedWork',work._key);
            let screen = instance.render(true);
            assert(text(screen).includes(work.title), 'Task detail missing');
            const approve = [...walk(screen)].find(n => n.kind === 'Button' && n.p.text === 'Approve');
            if (approve) {
                approve.h.onTap(0,0);
                assert.equal(commands.length,0,'Proposal dispatched a mutation');
                screen = instance.render(true);
                const confirm = [...walk(screen)].find(n=>n.kind === 'Button' && n.p.text === 'Confirm');
                assert(confirm,'Missing confirmation control');
                confirm.h.onTap(0,0);
                assert.equal(commands.length,1,'Confirmation must dispatch exactly one command');
                assert(commands[0].startsWith('export PATH="$HOME/.local/bin:$PATH"; miilo-owl action '),
                    'Confirmation must resolve the installed helper and dispatch an action');
            }
        }
        assert.deepEqual(problems,[],file+' runtime errors');
        cases++;
    }
}
assert(cases > 0);
console.log(`${cases} Linux OWL scenarios passed (themes, sizes, tasks, confirmation).`);
