// Keep each architecture's checksum list until every archive has been verified.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const [input, output] = process.argv.slice(2);
if (!input || !output || fs.existsSync(output)) throw new Error('Use a new output directory');
const files = [];
for (const arch of ['aarch64', 'x86_64']) {
  const folder = path.join(input, `miilo-owl-widgets-linux-${arch}`);
  const lines = fs.readFileSync(path.join(folder, 'SHA256SUMS'), 'utf8').trim().split('\n');
  if (lines.length !== 1) throw new Error('Expected one verified archive per architecture');
  const match = lines[0].match(/^([a-f0-9]{64})  (miilo-owl-widgets-[0-9][A-Za-z0-9.-]*-linux-(aarch64|x86_64)\.tar\.gz)$/);
  if (!match || match[3] !== arch) throw new Error('Invalid checksum filename');
  const data = fs.readFileSync(path.join(folder, match[2]));
  if (crypto.createHash('sha256').update(data).digest('hex') !== match[1]) throw new Error('Checksum mismatch');
  files.push({name: match[2], data, checksum: lines[0]});
}
fs.mkdirSync(output);
for (const file of files) fs.writeFileSync(path.join(output, file.name), file.data);
fs.writeFileSync(path.join(output, 'SHA256SUMS'), files.map(f => f.checksum).join('\n') + '\n');
