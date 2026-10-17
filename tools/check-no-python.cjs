// Enforce source/development-tool scope, excluding immutable historical evidence.
const fs=require('node:fs');
const path=require('node:path');
const {execFileSync}=require('node:child_process');
function violations(root, files) {
  const errors=[];
  for(const file of files) {
    const full=path.join(root,file);
    if(!fs.existsSync(full) || !fs.statSync(full).isFile()) continue;
    if(/\.(?:py|pyc|pyo|pyi)$/.test(file) || /(?:^|\/)(?:pyproject\.toml|Pipfile|requirements[^/]*\.txt)$/.test(file)) errors.push('Disallowed source: '+file);
    if(!/^(?:tools\/|tests\/|\.github\/|Makefile$|install\.sh$)/.test(file) || /check-no-python\.cjs$|no-python\.test\.cjs$/.test(file)) continue;
    if(!/\.(?:sh|cjs|mjs|js|ya?ml|toml|json|qml|cpp|h)$|Makefile$|CMakeLists\.txt$/.test(file)) continue;
    const text=fs.readFileSync(full,'utf8');
    if(/(?<![A-Za-z0-9_-])python(?:3(?:\.\d+)?)?\b|actions\/setup-python|\bPySide\d*\b|\bpytest\b|\bpip(?:3)?\s+install\b|\.py\b/i.test(text)) errors.push('Disallowed tool dependency: '+file);
  }
  return errors;
}
if(require.main===module) {
  const root=path.resolve(__dirname,'..');
  const files=execFileSync('git',['ls-files','--cached','--others','--exclude-standard','-z'],{cwd:root,encoding:'utf8'}).split('\0').filter(Boolean);
  const errors=violations(root,[...new Set(files)]);
  console.log(errors.length?errors.join('\n'):'No Python source, installer, CI, or preview dependency.');
  process.exitCode=errors.length?1:0;
}
module.exports={violations};
