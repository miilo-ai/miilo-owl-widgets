const fs=require('node:fs');
const path=require('node:path');
const {runPreview}=require('./lib/qt-preview.cjs');
const root=path.resolve(__dirname,'..');
const providers=[{id:'demo-factory',name:'Automation factory'},{id:'demo-agent',name:'Coding sessions'},{id:'demo-rest',name:'Resting system'}];
const actors=providers.map(p=>({_key:p.id+'-actor',id:'actor',providerId:p.id,name:p.name,status:p.id==='demo-rest'?'resting':'working',health:'healthy'}));
const tasks={
  'demo-factory':[['Review summary','verifying','review'],['Fix reconnect','implementing','none'],['Verify reconnect','verifying','none']],
  'demo-agent':[['Add search','implementing','none'],['Tighten layout','implementing','none'],['Test preferences','verifying','none']],
  'demo-rest':[],
};
const workItems=providers.flatMap(p=>tasks[p.id].map(([title,phase,attention],i)=>({_key:p.id+i,id:String(i),providerId:p.id,title,_actorKeys:[p.id+'-actor'],lifecycle:'active',phase,attention,_revisionKey:'r1',actions:[],summary:'Groups tasks by system and keeps review items in view. Synthetic preview data.',nativeUrl:'',_actionsEnabled:true,_evidenceKeys:i===0?['checks']:[],_artifactKeys:i===0?['diff','output']:[]})));
const data={configured:true,errors:[],snapshot:{providers,actors,workItems,attention:[],evidence:[{_key:'checks',outcome:'Passed',summary:'3 grouping, filtering and reconnect checks.',_missingArtifactCount:0}],artifacts:[{_key:'diff',name:'activity-summary.ts',kind:'change_set',_preview:'@@ group tasks by system @@\n- return tasks;\n+ return groupBySystem(tasks);',_url:'',_previewTruncated:false},{_key:'output',name:'Verification output',kind:'log',_preview:'PASS system grouping\nPASS needs-you filter\nPASS reconnect recovery',_url:'',_previewTruncated:false}]}};
const plugin=fs.existsSync(path.join(root,'vendor/owl'));
try {
  runPreview(root,'orbit',process.argv[2]||'/tmp/miilo-orbit-preview',scratch=>{
    const runtime=path.join(scratch,'runtime');fs.mkdirSync(runtime);
    for(const name of ['Owl.js','OwlLayout.js','OwlNode.qml','OwlCanvas.qml','MiiloTypography.qml','MiiloTypeScale.js'])fs.copyFileSync(path.join(root,plugin?'vendor/owl':'desktop/runtime',name),path.join(runtime,name));
    fs.writeFileSync(path.join(runtime,'OwlTerminal.qml'),'import QtQuick\nItem { property var node; property var widget }\n');
    const fixture=path.join(scratch,'orbit.qml');fs.copyFileSync(path.join(root,plugin?'tests/qml/OrbitFixture.qml':'tests/desktop/OrbitFixture.qml'),fixture);
    const json=path.join(scratch,'data.json');fs.writeFileSync(json,JSON.stringify(data));
    return {fixture,data:json,source:path.join(root,plugin?'assets/70-miilo-fleet.owl':'70-miilo-fleet.owl')};
  });
} catch(error) {console.error('Native Qt preview failed: '+error.message);process.exitCode=1;}
