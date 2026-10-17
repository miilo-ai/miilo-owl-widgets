// Dependency and module-size policy; no application code is imported.
const fs = require('node:fs');
const path = require('node:path');
function walk(root, prefix = '') {
  return fs.readdirSync(path.join(root,prefix), {withFileTypes:true}).flatMap(entry => {
    const name = prefix ? prefix + '/' + entry.name : entry.name;
    if (entry.name === '.git' || entry.name === 'target' || entry.name === 'node_modules' || entry.name === '__pycache__') return [];
    return entry.isDirectory() ? walk(root,name) : [name];
  });
}
function glob(pattern) {
  return new RegExp('^' + pattern.split(/(\*\*\/|\*\*|\*|\?)/).map(part =>
    part === '**/' ? '(?:.*/)?' : part === '**' ? '.*' : part === '*' ? '[^/]*' : part === '?' ? '[^/]' : part.replace(/[.+^$(){}|[\]\\]/g, '\\$&')).join('') + '$');
}
function violations(root, policy) {
  const problems=[], files=walk(root);
  for (const name of policy.absent || []) if (fs.existsSync(path.join(root,name))) problems.push('Packaging repository must not contain ' + name);
  for (const layer of policy.layers) {
    const matched = files.filter(name => layer.paths.some(pattern => glob(pattern).test(name)));
    if (!matched.length) problems.push('No source matched required layer: ' + layer.name);
    for (const name of matched) {
      const text=fs.readFileSync(path.join(root,name),'utf8');
      for (const expression of layer.forbid || []) if (new RegExp(expression,'m').test(text)) problems.push(layer.name + ': forbidden dependency in ' + name + ': ' + expression);
      const maximum = policy.existingSizeExceptions?.[name] ?? 200;
      const lines = text ? text.split('\n').length - (text.endsWith('\n') ? 1 : 0) : 0;
      if (lines > maximum) problems.push('Module exceeds ' + maximum + ' lines: ' + name);
    }
  }
  return problems;
}
if (require.main === module) {
  const root=path.resolve(process.argv[2] || path.join(__dirname,'..'));
  const problems=violations(root,JSON.parse(fs.readFileSync(path.join(root,'.specify/architecture.json'),'utf8')));
  console.log(problems.length ? problems.join('\n') : 'Architecture dependency and module-size checks passed.');
  process.exitCode=problems.length ? 1 : 0;
}
module.exports={violations,walk};
