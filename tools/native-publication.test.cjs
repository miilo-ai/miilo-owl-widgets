const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs'), os = require('node:os'), path = require('node:path'), crypto = require('node:crypto');
const {verify} = require('./verify-native-publication.cjs');
for (const scenario of ['valid', 'checksum', 'source', 'existing', 'extra', 'tag', 'architecture']) {
  test(`native publication rejects unsafe state: ${scenario}`, () => {
    const folder = fs.mkdtempSync(path.join(os.tmpdir(), 'miilo-native-publication-'));
    try {
      const name = 'miilo-owl-1.7.2-linux-x86_64.tar.gz';
      const data = Buffer.from('bounded archive fixture');
      fs.writeFileSync(path.join(folder, name), data);
      const hash = crypto.createHash('sha256').update(data).digest('hex');
      fs.writeFileSync(path.join(folder, 'SHA256SUMS-native-x86_64'), `${scenario === 'checksum' ? '0'.repeat(64) : hash}  ${name}\n`);
      if (scenario === 'extra') fs.writeFileSync(path.join(folder, 'unexpected'), 'refuse');
      const options = {folder, tag: scenario === 'tag' ? 'v1.7.2;bad' : 'v1.7.2', arch: scenario === 'architecture' ? 'other' : 'x86_64', sourceCommit: 'a'.repeat(40), existingAssets: scenario === 'existing' ? [name] : ['existing-arm-bundle.tar.gz', 'SHA256SUMS']};
      const buildInfo = () => `version=1.7.2\nsource_commit=${(scenario === 'source' ? 'b' : 'a').repeat(40)}\ntarget=x86_64-unknown-linux-musl\n`;
      if (scenario === 'valid') assert.equal(verify(options, buildInfo).length, 2);
      else assert.throws(() => verify(options, buildInfo));
    } finally { fs.rmSync(folder, {recursive: true, force: true}); }
  });
}
