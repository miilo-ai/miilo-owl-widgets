const fs=require('node:fs');
const {test}=require('node:test');
const assert=require('node:assert/strict');
const path=require('node:path');
const root=path.resolve(__dirname,'..');
for(const file of ['OwlNode.qml','OwlTerminal.qml'])test(file+' uses explicit plain text for every Text item',()=>{
 const source=fs.readFileSync(path.join(root,'desktop/runtime',file),'utf8');
 const blocks=[...source.matchAll(/\bText\s*\{([\s\S]*?)^\s*\}/gm)];
 assert(blocks.length>=3,'Expected runtime text surfaces');
 for(const block of blocks)assert.match(block[1],/textFormat:\s*Text\.PlainText/,'Implicit rich-text detection in '+file);
});
