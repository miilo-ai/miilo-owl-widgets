const fs=require('node:fs'),os=require('node:os'),path=require('node:path');
const test=require('node:test'),assert=require('node:assert/strict');
const {violations}=require('./check-preview-toolchain.cjs');
test('preview rejects authored native builds and Rust bindings that regenerate C++',t=>{
  const root=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-preview-guard-'));
  t.after(()=>fs.rmSync(root,{recursive:true,force:true}));
  const cases={'main.cpp':'int main() {}','CMakeLists.txt':'project(preview)','build.rs':'fn main() {}','Cargo.toml':'cxx-qt = "1"'};
  for(const [name,source]of Object.entries(cases)) {
    const relative='tools/qt-preview/'+name,file=path.join(root,relative);
    fs.mkdirSync(path.dirname(file),{recursive:true});fs.writeFileSync(file,source);
    assert.ok(violations(root,[relative]).length,name);
  }
  const safe='tools/qt-preview/src/main.rs';fs.mkdirSync(path.dirname(path.join(root,safe)),{recursive:true});
  fs.writeFileSync(path.join(root,safe),'fn main() {}');assert.deepEqual(violations(root,[safe]),[]);
});
