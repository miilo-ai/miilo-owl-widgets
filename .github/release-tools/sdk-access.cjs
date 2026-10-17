// Bootstrap access before Cargo can load its pinned SDK. Node built-ins only.
const fs=require('node:fs');
const path=require('node:path');
const {execFileSync}=require('node:child_process');
function pinnedReference(dependency) {
  if(dependency.git!=='https://github.com/miilo-ai/miilo-sdk')throw Error('Canonical SDK repository required');
  const {tag,rev}=dependency;
  if(Boolean(tag)===Boolean(rev))throw Error('Exactly one SDK tag or revision is required');
  if(rev&&!/^[a-f0-9]{40}$/.test(rev))throw Error('SDK revision must be a full commit SHA');
  if(tag&&!/^v[0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9.-]+)?$/.test(tag))throw Error('SDK tag must be versioned');
  return rev||tag;
}
function readDependency(file) {
  // Only the explicitly supported inline dependency table is accepted. No eval.
  const text=fs.readFileSync(file,'utf8'),tables=[...text.matchAll(/^miilo-adapter-sdk\s*=\s*\{([^}\n]+)\}/gm)];
  if(!tables.length)throw Error('Missing inline SDK dependency');
  const dependencies=tables.map(match=>Object.fromEntries([...match[1].matchAll(/\b(git|rev|tag)\s*=\s*"([^"\n]*)"/g)].map(m=>[m[1],m[2]])));
  const pin=pinnedReference(dependencies[0]);
  for(const dependency of dependencies)if(pinnedReference(dependency)!==pin)throw Error('SDK pins must agree');
  return dependencies[0];
}
async function main({env=process.env,request=fetch,log=console.log,root=process.cwd()}={}) {
  const dependency=readDependency(path.join(root,'helper/Cargo.toml'));pinnedReference(dependency);
  const token=env.SDK_READ_TOKEN||'',headers={Accept:'application/vnd.github+json','User-Agent':'miilo-release-check'};
  if(token)headers.Authorization='Bearer '+token;
  const response=await request('https://api.github.com/repos/miilo-ai/miilo-sdk',{headers,signal:AbortSignal.timeout(20000)});
  let ready=true;
  if(!response.ok) {
    if(!token&&[401,404].includes(response.status))ready=false;
    else throw Error('SDK access check failed; inspect credential scope or service availability');
  } else await response.json();
  const gitEnv={...env,GIT_TERMINAL_PROMPT:'0'};
  if(ready) {
    if(token) {
      if(!env.RUNNER_TEMP||!env.GITHUB_ENV)throw Error('Missing secure runner output paths');
      const encoded=Buffer.from('x-access-token:'+token).toString('base64');log('::add-mask::'+encoded);
      const config=path.join(env.RUNNER_TEMP,'miilo-sdk.gitconfig');
      // Refuse an existing file or symlink instead of following a planted path.
      fs.writeFileSync(config,'[http "https://github.com/miilo-ai/miilo-sdk"]\n\textraheader = AUTHORIZATION: basic '+encoded+'\n',{flag:'wx',mode:0o600});
      gitEnv.GIT_CONFIG_GLOBAL=config;
      fs.appendFileSync(env.GITHUB_ENV,'GIT_CONFIG_GLOBAL='+config+'\nCARGO_NET_GIT_FETCH_WITH_CLI=true\n');
    }
    if(dependency.rev) {
      const commit=await request('https://api.github.com/repos/miilo-ai/miilo-sdk/commits/'+dependency.rev,{headers,signal:AbortSignal.timeout(20000)});
      if(!commit.ok||(await commit.json()).sha!==dependency.rev)throw Error('Pinned SDK commit cannot be fetched; no release may proceed');
    } else {
      try{execFileSync('git',['ls-remote','--exit-code',dependency.git,'refs/tags/'+dependency.tag],{env:gitEnv,stdio:['ignore','pipe','pipe'],timeout:30000});}
      catch{throw Error('Pinned SDK tag cannot be fetched; no release may proceed');}
    }
  } else log('Private SDK access is not configured. No release has been made');
  if(env.GITHUB_OUTPUT)fs.appendFileSync(env.GITHUB_OUTPUT,'ready='+ready+'\n');
  if(!ready&&env.GITHUB_STEP_SUMMARY)fs.appendFileSync(env.GITHUB_STEP_SUMMARY,'No release has been made\nPrivate SDK requires MIILO_SDK_READ_TOKEN (contents read).\n');
  return ready;
}
if(require.main===module)main().catch(()=>{console.error('SDK access qualification failed; no release may proceed.');process.exitCode=1;});
module.exports={pinnedReference,readDependency,main};
