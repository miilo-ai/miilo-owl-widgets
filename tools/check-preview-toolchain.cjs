// Preview orchestration is Rust; rendering/tests use the installed Qt QML runner.
const fs=require('node:fs');
const path=require('node:path');
const {execFileSync}=require('node:child_process');
function violations(root,files) {
  const errors=[];
  for(const name of files) {
    if(!name.startsWith('tools/qt-preview/') && name!=='tools/lib/qt-preview.cjs')continue;
    const file=path.join(root,name);
    if(!fs.existsSync(file)||!fs.statSync(file).isFile())continue;
    if(/\.(?:c|cc|cpp|cxx|h|hpp)$|(?:^|\/)(?:CMakeLists\.txt|build\.rs)$/.test(name))errors.push('Custom native preview build is forbidden: '+name);
    if(/\.(?:rs|qml|js)$/.test(name)) {
      const source=fs.readFileSync(file,'utf8');
      if(source.trimEnd().split('\n').length>200)errors.push('Preview module exceeds 200 lines: '+name);
      if(name.endsWith('.rs') && /\buse\s+[^;]*::\*/.test(source))errors.push('Preview imports must be explicit: '+name);
    }
    if(!/Cargo\.toml$|qt-preview\.cjs$/.test(name))continue;
    if(/\b(?:cmake|cxx[-_]qt|qmetaobject|bindgen)\b|\[build-dependencies\]/.test(fs.readFileSync(file,'utf8')))errors.push('Preview must use the installed QML runner: '+name);
  }
  return errors;
}
if(require.main===module) {
  const root=path.resolve(__dirname,'..');
  const files=execFileSync('git',['ls-files','--cached','--others','--exclude-standard','-z'],{cwd:root,encoding:'utf8'}).split('\0').filter(Boolean);
  const errors=violations(root,[...new Set(files)]);
  console.log(errors.length?errors.join('\n'):'Preview tooling has no authored C/C++, CMake or Qt binding build.');
  process.exitCode=errors.length?1:0;
}
module.exports={violations};
