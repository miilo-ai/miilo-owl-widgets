const fs=require('node:fs');
const os=require('node:os');
const path=require('node:path');
const test=require('node:test');
const assert=require('node:assert/strict');
const {violations}=require('./check-no-python.cjs');
test('language source, workflow bootstrap, interpreter and preview bindings fail closed',t=>{
  const root=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-toolchain-'));t.after(()=>fs.rmSync(root,{recursive:true,force:true}));
  const cases={'tools/old.py':'print(1)','tools/run.sh':'python3 old.py','.github/action.yml':'uses: actions/setup-python@fixture','tools/capture.cpp':'// PySide6'};
  for(const [name,text]of Object.entries(cases)){fs.mkdirSync(path.dirname(path.join(root,name)),{recursive:true});fs.writeFileSync(path.join(root,name),text);assert.ok(violations(root,[name]).length,name);}
  fs.writeFileSync(path.join(root,'Makefile'),'check:\n\tnode tools/check-no-python.cjs\n');assert.deepEqual(violations(root,['Makefile']),[]);
});
