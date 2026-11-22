const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs'), path = require('node:path'), os = require('node:os'), crypto = require('node:crypto');
const {spawnSync} = require('node:child_process');
for (const scenario of ['valid', 'missing', 'tampered', 'wrong-architecture']) test(`collect architecture releases: ${scenario}`, () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'miilo-release-'));
  try {
    for (const arch of ['aarch64', 'x86_64']) {
      if (arch === 'x86_64' && scenario === 'missing') continue;
      const folder = path.join(root, 'input', `miilo-owl-widgets-linux-${arch}`); fs.mkdirSync(folder, {recursive:true});
      const name = `miilo-owl-widgets-1.7.3-linux-${scenario === 'wrong-architecture' ? 'aarch64' : arch}.tar.gz`;
      const data = Buffer.from(arch); const digest = crypto.createHash('sha256').update(data).digest('hex');
      fs.writeFileSync(path.join(folder, name), scenario === 'tampered' ? 'changed' : data);
      fs.writeFileSync(path.join(folder, 'SHA256SUMS'), `${digest}  ${name}\n`);
    }
    const output=path.join(root,'output');
    const r=spawnSync(process.execPath,['tools/collect-release-assets.cjs',path.join(root,'input'),output],{encoding:'utf8'});
    if(scenario === 'valid') { assert.equal(r.status,0,r.stderr); assert.equal(fs.readdirSync(output).length,3); }
    else { assert.notEqual(r.status,0); assert.equal(fs.existsSync(output),false); }
  } finally { fs.rmSync(root,{recursive:true,force:true}); }
});
