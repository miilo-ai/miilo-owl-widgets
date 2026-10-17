// Build and run an isolated native Qt capture harness; no application dependencies.
const fs=require('node:fs');
const os=require('node:os');
const path=require('node:path');
const {execFileSync}=require('node:child_process');
function runPreview(root,mode,output,prepare) {
  const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-qt-preview-'));
  try {
    const env={...process.env};
    if(!env.CMAKE_PREFIX_PATH&&process.platform==='darwin')env.CMAKE_PREFIX_PATH='/opt/homebrew';
    const build=path.join(scratch,'build');
    execFileSync('cmake',['-S',path.join(root,'tools/qt-preview'),'-B',build,'-DCMAKE_BUILD_TYPE=Release'],{env,stdio:'inherit',timeout:120000});
    execFileSync('cmake',['--build',build,'--config','Release','--parallel','2'],{env,stdio:'inherit',timeout:120000});
    const inputs=prepare(scratch);
    const binary=path.join(build,process.platform==='win32'?'Release/miilo-preview.exe':'miilo-preview');
    execFileSync(binary,[mode,inputs.fixture,path.resolve(output),root,inputs.data||'-',inputs.source||'-'],{env,stdio:'inherit',timeout:90000});
  } finally {fs.rmSync(scratch,{recursive:true,force:true});}
}
module.exports={runPreview};
