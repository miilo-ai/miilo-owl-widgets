const fs=require('node:fs');
const os=require('node:os');
const path=require('node:path');
const test=require('node:test');
const assert=require('node:assert/strict');
const {pinnedReference,main}=require('./sdk-access.cjs');
const root=path.resolve(__dirname,'../..'),revision='40c2008207ae8abac2b82eeca7511ad6bf665d35';
test('canonical immutable pin required',()=>{
  assert.equal(pinnedReference({git:'https://github.com/miilo-ai/miilo-sdk',rev:'a'.repeat(40)}),'a'.repeat(40));
  for(const value of [{git:'https://example.invalid/sdk',rev:'a'.repeat(40)},{git:'https://github.com/miilo-ai/miilo-sdk',rev:'main'},{git:'https://github.com/miilo-ai/miilo-sdk',tag:'v1.0.0',rev:'a'.repeat(40)}])assert.throws(()=>pinnedReference(value));
});
test('missing private access cannot qualify and clearly skips publication',async t=>{
  const folder=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-access-'));t.after(()=>fs.rmSync(folder,{recursive:true,force:true}));const output=path.join(folder,'out'),logs=[];
  assert.equal(await main({root,env:{GITHUB_OUTPUT:output},request:async()=>({ok:false,status:404}),log:m=>logs.push(m)}),false);
  assert.equal(fs.readFileSync(output,'utf8'),'ready=false\n');assert.match(logs.join('\n'),/No release has been made/);
});
test('public SDK must still resolve exact revision',async()=>{
  const calls=[];
  assert.equal(await main({root,env:{},request:async url=>{calls.push(url);return {ok:true,json:async()=>({sha:revision})};}}),true);
  assert.equal(calls.length,2);assert.match(calls[1],new RegExp(revision+'$'));
  await assert.rejects(main({root,env:{},request:async()=>({ok:true,json:async()=>({sha:'wrong'})})}),/Pinned SDK/);
});
test('bad credential/service failure is not mistaken for an allowed skip',async()=>{
  await assert.rejects(main({root,env:{SDK_READ_TOKEN:'fixture'},request:async()=>({ok:false,status:401})}),/access check failed/);
  await assert.rejects(main({root,env:{},request:async()=>({ok:false,status:503})}),/access check failed/);
});

test('private access config is owner-only, masked and never overwrites an existing path',async t=>{
  const folder=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-private-access-'));t.after(()=>fs.rmSync(folder,{recursive:true,force:true}));
  const env={SDK_READ_TOKEN:'inert-fixture-token',RUNNER_TEMP:folder,GITHUB_ENV:path.join(folder,'env')},logs=[];
  const request=async()=>({ok:true,json:async()=>({sha:revision})});
  assert.equal(await main({root,env,request,log:m=>logs.push(m)}),true);
  const config=path.join(folder,'miilo-sdk.gitconfig');
  assert.equal(fs.statSync(config).mode&0o777,0o600);
  assert.ok(logs.some(m=>m.startsWith('::add-mask::')));
  assert.ok(!logs.some(m=>m.includes(env.SDK_READ_TOKEN)));
  assert.match(fs.readFileSync(env.GITHUB_ENV,'utf8'),/CARGO_NET_GIT_FETCH_WITH_CLI=true/);
  await assert.rejects(main({root,env,request,log:()=>{}}),/EEXIST/);
});
