const fs=require('node:fs');
const os=require('node:os');
const path=require('node:path');
const test=require('node:test');
const assert=require('node:assert/strict');
const {violations}=require('./check-architecture.cjs');
function fixture(t) { const root=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-boundary-'));t.after(()=>fs.rmSync(root,{recursive:true,force:true}));return root; }
test('adapter imports fail and inward ports pass',t=>{
  const root=fixture(t), policy={layers:[{name:'core',paths:['core/**/*.rs'],forbid:['\\breqwest\\b']}]};
  fs.mkdirSync(path.join(root,'core'));
  fs.writeFileSync(path.join(root,'core/a.rs'),'use reqwest::Client;\n');
  assert.equal(violations(root,policy).length,1);
  fs.writeFileSync(path.join(root,'core/a.rs'),'pub trait Source {}\n');
  assert.deepEqual(violations(root,policy),[]);
});
test('size exceptions cannot grow',t=>{
  const root=fixture(t),policy={layers:[{name:'core',paths:['*.rs']}],existingSizeExceptions:{'core.rs':201}};
  fs.writeFileSync(path.join(root,'core.rs'),'// line\n'.repeat(202));assert.equal(violations(root,policy).length,1);
  fs.writeFileSync(path.join(root,'core.rs'),'// line\n'.repeat(201));assert.deepEqual(violations(root,policy),[]);
});
test('missing layer fails closed',t=>assert.equal(violations(fixture(t),{layers:[{name:'core',paths:['core/*.rs']}]}).length,1));
