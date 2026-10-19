const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),os=require('node:os'),path=require('node:path');
const {spawnSync}=require('node:child_process');
const script=path.resolve('tools/install-helper.sh');
for(const scenario of ['success','checksum','download','existing','architecture'])test('helper installer: '+scenario,()=>{
 const root=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-install-'));
 try{
  const bin=path.join(root,'bin'),home=path.join(root,'home'),assets=path.join(root,'assets');
  fs.mkdirSync(bin);fs.mkdirSync(home);fs.mkdirSync(assets);
  const write=(name,text)=>fs.writeFileSync(path.join(bin,name),'#!/bin/sh\n'+text,{mode:0o755});
  write('uname',scenario==='architecture'?'echo x86_64':'if [ "$1" = -s ]; then echo Linux; else echo aarch64; fi');
  write('curl',`[ "$SCENARIO" != download ] || exit 22\nfor arg; do case "$arg" in https:*) url="$arg";; esac; last="$arg"; done\ncase "$url" in *SHA256SUMS) cp "$ASSETS/SHA256SUMS" "$last";; *) cp "$ASSETS/bundle.tgz" "$last";; esac`);
  fs.mkdirSync(path.join(assets,'bin/linux-aarch64'),{recursive:true});
  fs.writeFileSync(path.join(assets,'bin/linux-aarch64/miilo-owl'),'#!/bin/sh\necho "miilo-owl 1.6.0"\n',{mode:0o755});
  assert.equal(spawnSync('tar',['-czf',path.join(assets,'bundle.tgz'),'-C',assets,'./bin']).status,0);
  const hash=require('node:crypto').createHash('sha256').update(fs.readFileSync(path.join(assets,'bundle.tgz'))).digest('hex');
  fs.writeFileSync(path.join(assets,'SHA256SUMS'),`${scenario==='checksum'?'0'.repeat(64):hash}  miilo-owl-widgets-1.6.0-linux-aarch64.tar.gz\n`);
  const destination=path.join(home,'.local/bin/miilo-owl');
  if(scenario==='existing'){fs.mkdirSync(path.dirname(destination),{recursive:true});fs.writeFileSync(destination,'preserve');}
  const result=spawnSync('bash',[script],{env:{...process.env,HOME:home,PATH:bin+':'+process.env.PATH,ASSETS:assets,SCENARIO:scenario},encoding:'utf8'});
  if(scenario==='success'||scenario==='existing')assert.equal(result.status,0,result.stdout+result.stderr); else assert.notEqual(result.status,0);
  assert.equal(fs.existsSync(destination),scenario==='success'||scenario==='existing');
  if(scenario==='existing')assert.equal(fs.readFileSync(destination,'utf8'),'preserve');
 }finally{fs.rmSync(root,{recursive:true,force:true});}
});
