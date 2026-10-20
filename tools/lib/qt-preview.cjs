// Prepare synthetic inputs; Rust owns the bounded native QML test process.
const fs=require('node:fs');
const os=require('node:os');
const path=require('node:path');
const {execFileSync}=require('node:child_process');
function runPreview(root,mode,output,prepare) {
  const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-qt-preview-'));
  try {
    const inputs=prepare(scratch);
    execFileSync('cargo',['run','--locked','--manifest-path',path.join(root,'tools/qt-preview/Cargo.toml'),'--',
      mode,inputs.fixture,path.resolve(output),root,inputs.data||'-',inputs.source||'-'],
      {stdio:'inherit',timeout:180000});
  } finally {fs.rmSync(scratch,{recursive:true,force:true});}
}
module.exports={runPreview};
