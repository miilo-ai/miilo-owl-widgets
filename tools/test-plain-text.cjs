// Real Qt render plus a loopback request canary, without touching the live shell.
const fs=require('node:fs'),path=require('node:path'),os=require('node:os');
const http=require('node:http'),assert=require('node:assert/strict'),{spawn}=require('node:child_process');
const root=path.resolve(__dirname,'..');
const fixture=process.argv[2]||'tests/typography/TestPlainText.qml';
const folder=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-plain-text-'));
const requests=[];
const server=http.createServer((req,res)=>{requests.push(req.url);res.writeHead(200,{'Content-Type':'image/png'});res.end(Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=','base64'));});
server.listen(0,'127.0.0.1',()=>{
 const base='http://127.0.0.1:'+server.address().port;
 fs.mkdirSync(path.join(folder,'runtime'),{mode:0o700});
 fs.writeFileSync(path.join(folder,'shell.qml'),`import Quickshell\nimport "file:${path.dirname(path.join(root,fixture))}" as Tests\nShellRoot { FloatingWindow { implicitWidth:900; implicitHeight:700; Tests.TestPlainText { canaryBase: "${base}" } } }\n`);
 const env={...process.env,XDG_RUNTIME_DIR:path.join(folder,'runtime'),QT_QPA_PLATFORM:'offscreen',QT_QUICK_BACKEND:'software',QT_QPA_PLATFORMTHEME:'basic',QT_QUICK_CONTROLS_STYLE:'Basic'};
 delete env.WAYLAND_DISPLAY;
 const child=spawn('quickshell',['-p',path.join(folder,'shell.qml'),'--no-color'],{env});let output='';
 child.stdout.on('data',v=>output+=v);child.stderr.on('data',v=>output+=v);
 const timer=setTimeout(()=>child.kill('SIGKILL'),25000);
 child.on('error',e=>{output+=String(e);});
 child.on('close',code=>{
  clearTimeout(timer);console.log(output);console.log("Canary requests:",JSON.stringify(requests));
  try{
   assert.equal(code,0,'Qt test process failed');
   assert(output.includes('MIILO_PLAIN_TEXT_OK'),'Native assertions did not pass');
   assert(requests.includes('/positive-control'),'Image request monitor was not exercised');
   assert.deepEqual(requests.filter(v=>v!=='/positive-control'),[],'Untrusted labels triggered image requests');
   console.log('PASS: positive control requested an image; provider/button/placeholder/terminal labels made zero requests.');
  }catch(e){console.error(e.message);process.exitCode=1;}
  server.close();fs.rmSync(folder,{recursive:true,force:true});
 });
});
