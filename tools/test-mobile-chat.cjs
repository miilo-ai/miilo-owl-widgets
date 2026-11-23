// Real OWL handlers: privacy, streaming correlation, keyboard and cancellation.
const fs=require('node:fs'),os=require('node:os'),path=require('node:path');
const Module=require('node:module'),assert=require('node:assert/strict'),cp=require('node:child_process');
const runtime=new Module('owl');runtime._compile(fs.readFileSync('desktop/runtime/Owl.js','utf8').replace(/^\.pragma library/,''),'owl');
const Owl=runtime.exports;
function* walk(n){yield n;for(const c of n.c||n.children||[])yield*walk(c);}
let cases=0;
for(const file of ['70-miilo-fleet.owl','71-miilo-attention.owl','72-miilo-board.owl'])for(const width of [320,393,600]){
 const program=Owl.compile(fs.readFileSync(file,'utf8')),errors=[],actions=[],commands=[];
 const instance=new Owl.Instance(program,{theme:k=>Owl.themeFromToml('')[k]??null,tile:k=>({linux:true,tileWidth:width,tileHeight:180,screenWidth:width,screenHeight:800})[k]??null,report:e=>errors.push(e),changed(){},act:(...a)=>actions.push(a),runInto:c=>commands.push(c)},{});
 const slot=k=>program.globals.findIndex(g=>g.name===k),get=k=>instance.values[slot(k)],set=(k,v)=>instance.setState(slot(k),Owl.fromJson(v));
 const source=(k,v)=>instance.setSource(slot(k),Owl.fromJson(v));
 const nodes=()=>[...walk(instance.render(true))],button=t=>nodes().find(n=>n.kind==='Button'&&n.p.text===t);
 const input=()=>nodes().find(n=>n.kind==='Input');
 source('chatClock',12345);source('feed',{ok:true,out:JSON.stringify({configured:true,errors:[],snapshot:{providers:[],workItems:[],actors:[],attention:[]}}),code:0});
 set('chatOpen',true);
 assert(!input(),'An older helper cannot expose a nonfunctional chat composer');
 source('chatStatus',{ok:true,out:JSON.stringify({widgetApi:1,ready:false}),code:0});
 assert(!input(),'Configure the model before enabling the composer');
 source('chatStatus',{ok:true,out:JSON.stringify({widgetApi:1,ready:true}),code:0});
 assert.equal(get('chatShare'),false);assert.equal(get('chatSpoken'),false);
 assert(!nodes().some(n=>n.kind==='Canvas'&&n.p.width===22),'No recognizer means no microphone control');
 const question="What is 'this'?\n$(touch SHOULD_NOT_EXIST); <img src=x>";
 input().h.onSubmit(question);
 assert.equal(get('chatBusy'),true);assert.equal(get('chatPrompt'),question);
 const command=instance.sources().find(s=>s.name==='chatReply').arg();
 assert(command.includes('widget-chat'));assert(!command.includes(' action '));
 const dir=fs.mkdtempSync(path.join(os.tmpdir(),'miilo-chat-wire-'));
 try{
  fs.mkdirSync(path.join(dir,'.local/bin'),{recursive:true});
  fs.writeFileSync(path.join(dir,'.local/bin/miilo-owl'),'#!/bin/sh\ncat > "$WIRE"\n',{mode:0o700});
  cp.execFileSync('/bin/sh',['-c',command],{cwd:dir,env:{...process.env,HOME:dir,WIRE:path.join(dir,'wire')}});
  const fields=fs.readFileSync(path.join(dir,'wire'),'utf8').split('\0');
  assert.equal(fields[1],question);assert.equal(fields[2],'false');assert(!fs.existsSync(path.join(dir,'SHOULD_NOT_EXIST')));
 }finally{fs.rmSync(dir,{recursive:true,force:true});}
 const id=get('chatId');
 source('chatReply',{requestId:'old',text:'Wrong conversation',done:true,error:null});
 assert.equal(get('chatAnswer'),'');assert.equal(get('chatBusy'),true);
 source('chatReply',{requestId:id,text:'First',done:false,error:null});assert.equal(get('chatAnswer'),'First');
 source('chatReply',{requestId:id,text:'First answer',done:true,error:null});
 assert.equal(get('chatBusy'),false);assert.equal(get('chatHistory').length,2);assert.deepEqual(commands,[],'No speech or action request without opt-in');
 source('said',{available:true,listening:false,final:'Dictated question',text:'',error:null});
 assert.equal(get('chatDraft'),'Dictated question');assert.equal(get('chatBusy'),false,'Dictation is a draft, never implicit send');
 const mic=nodes().find(n=>n.kind==='Box'&&n.p.radius===22&&n.h?.onTap);
 mic.h.onTap(0,0);assert(actions.some(a=>a[0]==='speech'&&a[1][0]==='toggle'));
 button('Share context: Off').h.onTap(0,0);assert.equal(get('chatHistory').length,0,'Changing context consent clears conversational carryover');
 input().h.onSubmit('Another question');button('Stop reply').h.onTap(0,0);
 assert.equal(get('chatBusy'),false);
 source('chatReply',{requestId:get('chatId'),text:'Late answer',done:true,error:null});
 assert.equal(get('chatHistory').length,0,'Late completion after Stop must be discarded');
 source('feed',{ok:true,out:JSON.stringify({configured:true,errors:[],snapshot:{providers:[{id:'p'}],workItems:[{_key:'w',providerId:'p',_revisionKey:'r1',title:'Fixture'}],actors:[],attention:[]}}),code:0});
 set('selectedWork','w');input().h.onSubmit('Explain selected work');assert.equal(get('chatBusy'),true);
 const oldId=get('chatId');
 source('feed',{ok:true,out:JSON.stringify({configured:true,errors:[],snapshot:{providers:[{id:'p'}],workItems:[{_key:'w',providerId:'p',_revisionKey:'r2',title:'Changed fixture'}],actors:[],attention:[]}}),code:0});
 assert.equal(get('chatBusy'),false,'A changed shared revision cancels the old turn');
 source('chatReply',{requestId:oldId,text:'Stale explanation',done:true,error:null});
 assert.equal(get('chatAnswer'),'','Stale explanations must not enter the new revision');
 assert(!Object.keys(instance.persisted()).some(k=>k.startsWith('chat')),'Conversation must not persist by accident');
 assert.deepEqual(errors,[]);cases++;
}
console.log(`${cases} mobile chat scenarios passed: stdin quoting, stream correlation, context consent, keyboard send, dictation drafts and cancellation.`);
