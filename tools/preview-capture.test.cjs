// Real installed Qt execution with disposable synthetic inputs; no device access.
const test=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),os=require('node:os'),path=require('node:path');
const {spawnSync}=require('node:child_process');
const root=path.resolve(__dirname,'..');
function capture(t,options={},tool='render-orbit.cjs') {
  const output=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-preview-matrix-'));
  t.after(()=>fs.rmSync(output,{recursive:true,force:true}));
  const env={...process.env};
  for(const key of Object.keys(env))if(key.startsWith('MIILO_PREVIEW_'))delete env[key];
  const result=spawnSync(process.execPath,[path.join(root,'tools',tool),output],
    {cwd:root,env:{...env,...options},encoding:'utf8',timeout:180000});
  assert.ifError(result.error);
  return {output,result};
}
for(const [name,options]of [
  ['orbit',{}],
  ['light reduced',{MIILO_PREVIEW_THEME:'light',MIILO_PREVIEW_MOTION:'reduced'}],
  ['motion off',{MIILO_PREVIEW_MOTION:'off'}],
  ['quiet',{MIILO_PREVIEW_VIEW:'quiet',MIILO_PREVIEW_WIDTH:'720',MIILO_PREVIEW_HEIGHT:'230',MIILO_PREVIEW_PALETTE:path.join(root,'tests/themes/osaka-jade.toml')}],
  ['quiet expanded',{MIILO_PREVIEW_VIEW:'quiet-expanded'}],
  ['inspection recording',{MIILO_PREVIEW_VIEW:'inspect',MIILO_PREVIEW_RECORD:'1',MIILO_PREVIEW_SECONDS:'2'}]
])test('native QML capture: '+name,t=>{
  const {output,result}=capture(t,options);
  assert.equal(result.status,0,result.stdout+'\n'+result.stderr);
  for(const file of ['orbit.png','orbit-next-frame.png','orbit-detail.png'])assert.ok(fs.statSync(path.join(output,file)).size>24);
  if(options.MIILO_PREVIEW_RECORD)assert.equal(fs.readdirSync(path.join(output,'frames')).length,25);
});
if(fs.existsSync(path.join(root,'tools/render-crew.cjs')))test('native QML onboarding expansion and narrow layout',t=>{
  const {output,result}=capture(t,{},'render-crew.cjs');
  assert.equal(result.status,0,result.stdout+'\n'+result.stderr);
  for(const file of ['crew-dark.png','crew-expanded.png','crew-light-narrow.png'])assert.ok(fs.statSync(path.join(output,file)).size>24);
});
for(const [name,options]of [
  ['invalid viewport',{MIILO_PREVIEW_WIDTH:'99'}],
  ['missing test runner',{MIILO_QMLTESTRUNNER:'/nonexistent/miilo-qmltestrunner'}]
])test('native preview fails closed: '+name,t=>{
  const {output,result}=capture(t,options);
  assert.notEqual(result.status,0);
  assert.deepEqual(fs.readdirSync(output),[],'A failed run must not publish captures');
});
test('native preview fails closed on QML compilation errors',t=>{
  const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-preview-invalid-qml-'));
  t.after(()=>fs.rmSync(scratch,{recursive:true,force:true}));
  const fixture=path.join(scratch,'invalid.qml'),output=path.join(scratch,'captures');
  fs.writeFileSync(fixture,'import QtQuick\nWindow { property int broken: }\n');
  const result=spawnSync('cargo',['run','--locked','--manifest-path',path.join(root,'tools/qt-preview/Cargo.toml'),
    '--','crew',fixture,output,root,'-','-'],{cwd:root,encoding:'utf8',timeout:180000});
  assert.ifError(result.error);
  assert.notEqual(result.status,0);
  assert.match(result.stdout,/FAIL!|Error/);
  assert.equal(fs.existsSync(output),false);
});
