const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {execFileSync} = require('node:child_process');

function verify({tag, arch, folder, sourceCommit, existingAssets}, readBuildInfo) {
  if (!/^v[0-9]+\.[0-9]+\.[0-9]+([.-][A-Za-z0-9.-]+)?$/.test(tag)) throw Error('Invalid version tag');
  if (!['aarch64', 'x86_64'].includes(arch)) throw Error('Invalid architecture');
  if (!/^[a-f0-9]{40}$/.test(sourceCommit)) throw Error('Invalid source commit');
  const archive = `miilo-owl-${tag.slice(1)}-linux-${arch}.tar.gz`;
  const sums = `SHA256SUMS-native-${arch}`;
  const names = fs.readdirSync(folder).sort();
  if (JSON.stringify(names) !== JSON.stringify([archive, sums].sort())) throw Error('Unexpected release files');
  if (names.some(n => existingAssets.includes(n))) throw Error('Asset already exists; refusing to replace it');
  const checksum = fs.readFileSync(path.join(folder, sums), 'utf8');
  const digest = crypto.createHash('sha256').update(fs.readFileSync(path.join(folder, archive))).digest('hex');
  if (checksum !== `${digest}  ${archive}\n`) throw Error('Invalid release checksum');
  const expected = `version=${tag.slice(1)}\nsource_commit=${sourceCommit}\ntarget=${arch}-unknown-linux-musl\n`;
  if (readBuildInfo(path.join(folder, archive)) !== expected) throw Error('Release source identity mismatch');
  return names;
}

if (require.main === module) {
  const {TAG: tag, ARCH: arch, GITHUB_REPOSITORY: repo} = process.env;
  if (!repo || !/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(repo)) throw Error('Invalid repository');
  if (!/^v[0-9]+\.[0-9]+\.[0-9]+([.-][A-Za-z0-9.-]+)?$/.test(tag || '')) throw Error('Invalid version tag');
  const sourceCommit = execFileSync('git', ['rev-parse', '--verify', `refs/tags/${tag}^{commit}`], {encoding: 'utf8'}).trim();
  const release = JSON.parse(execFileSync('gh', ['release', 'view', tag, '--repo', repo, '--json', 'assets'], {encoding: 'utf8'}));
  const files = verify({tag, arch, folder: 'dist-native', sourceCommit, existingAssets: release.assets.map(a => a.name)},
    archive => execFileSync('tar', ['-xOzf', archive, './BUILD-INFO'], {encoding: 'utf8'}));
  console.log(`Verified ${files.length} new assets; existing release assets will be retained.`);
}
module.exports = {verify};
