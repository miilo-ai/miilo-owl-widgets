const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs'), path = require('node:path'), os = require('node:os');
const {spawnSync} = require('node:child_process');
for (const arch of ['aarch64','x86_64','unsupported']) test(`widget bundle architecture selection: ${arch}`, () => {
 const root=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-bundle-'));
 try {
  const home=path.join(root,'home'),bin=path.join(root,'commands');fs.mkdirSync(home);fs.mkdirSync(bin);
  fs.copyFileSync('install.sh',path.join(root,'install.sh'));
  fs.writeFileSync(path.join(bin,'uname'),`#!/bin/sh\necho ${arch}\n`,{mode:0o755});
  if(arch!=='unsupported'){
   const helpers=path.join(root,'bin',`linux-${arch}`);fs.mkdirSync(helpers,{recursive:true});
   fs.writeFileSync(path.join(helpers,'miilo-owl'),'#!/bin/sh\ncase "$1" in --version) echo "miilo-owl 1.7.2";; uninstall|record-widget-install) exit 0;; *) exit 99;; esac\n',{mode:0o755});
  }
  fs.writeFileSync(path.join(root,'70-miilo-fleet.owl'),'widget fixture');
  const config=path.join(home,'.config/miilo-owl');fs.mkdirSync(config,{recursive:true});
  fs.writeFileSync(path.join(config,'config.json'),'preserve configuration');
  const r=spawnSync('sh',[path.join(root,'install.sh'),'--no-setup'],{env:{...process.env,HOME:home,PATH:bin+':'+process.env.PATH},encoding:'utf8'});
  if(arch==='unsupported'){assert.notEqual(r.status,0);assert.equal(fs.existsSync(path.join(home,'.local')),false);}
  else {assert.equal(r.status,0,r.stderr);assert.ok(fs.existsSync(path.join(home,'.local/bin/miilo-owl')));assert.equal(fs.readFileSync(path.join(home,'.config/omalauncher/widgets/70-miilo-fleet.owl'),'utf8'),'widget fixture');}
  assert.equal(fs.readFileSync(path.join(config,'config.json'),'utf8'),'preserve configuration');
 } finally {fs.rmSync(root,{recursive:true,force:true});}
});
