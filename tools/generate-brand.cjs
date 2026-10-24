// Project SVG artwork projected into stock OWL6 canvas commands, without image URLs.
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '..');
const round = n => Number(n.toFixed(3));
function attributes(text) {
  return Object.fromEntries([...text.matchAll(/([\w-]+)="([^"]*)"/g)].map(m => [m[1], m[2]]));
}
function contours(source) {
  const tokens = source.match(/[A-Za-z]|-?\d*\.?\d+(?:e[-+]?\d+)?/g);
  const paths = []; let points, x = 0, y = 0, start, command, i = 0;
  const number = () => Number(tokens[i++]);
  while (i < tokens.length) {
    if (/^[A-Za-z]$/.test(tokens[i])) command = tokens[i++];
    if (command === 'Z') { points.push(start); command = ''; continue; }
    if (command === 'M') {
      x = number(); y = number(); start = [x, y]; points = [start]; paths.push(points); command = 'L';
    } else if (command === 'L') { x = number(); y = number(); points.push([x, y]); }
    else if (command === 'H') { x = number(); points.push([x, y]); }
    else if (command === 'V') { y = number(); points.push([x, y]); }
    else if (command === 'Q' || command === 'C') {
      const from = [x, y], first = [number(), number()];
      const second = command === 'C' ? [number(), number()] : first;
      const end = [number(), number()];
      for (let step = 1; step <= 16; step++) {
        const t = step / 16, u = 1 - t;
        points.push([0, 1].map(axis => command === 'C'
          ? u*u*u*from[axis] + 3*u*u*t*first[axis] + 3*u*t*t*second[axis] + t*t*t*end[axis]
          : u*u*from[axis] + 2*u*t*first[axis] + t*t*end[axis]));
      }
      [x, y] = end;
    } else throw Error('Unsupported brand path command: ' + command);
  }
  return paths;
}
function area(points) { return points.reduce((sum, p, i) => { const next = points[(i+1)%points.length]; return sum+p[0]*next[1]-next[0]*p[1]; }, 0); }
function canvas(svg) {
  const stack = [{fill: 'ink', stroke: 'none', 'stroke-width': '1', tx: 0, ty: 0}], output = [];
  for (const tag of svg.matchAll(/<(\/?)(g|path|circle)\b([^>]*)>/g)) {
    if (tag[1]) { stack.pop(); continue; }
    const style = {...stack.at(-1), ...attributes(tag[3])};
    if (style.transform) {
      const translation = /^translate\(([-\d.]+)[ ,]+([-\d.]+)\)$/.exec(style.transform);
      if (!translation) throw Error('Unsupported brand transform');
      style.tx += Number(translation[1]); style.ty += Number(translation[2]); delete style.transform;
    }
    if (tag[2] === 'g') { stack.push(style); continue; }
    const coordinate = (n, axis) => `${round(Number(n) + (axis ? style.ty : style.tx))}*k`;
    const options = [];
    if (style.fill !== 'none') options.push('fill: ink');
    if (style.stroke !== 'none') options.push(`stroke: ink`, `width: ${style['stroke-width']}*k`, 'cap: round');
    if (tag[2] === 'circle') output.push(`      circle(${coordinate(style.cx, 0)}, ${coordinate(style.cy, 1)}, ${style.r}*k, ${options.join(', ')})`);
    else {
      const shapes = contours(style.d);
      if (style.fill === "none") {
        for (const points of shapes) {
          const positions = points.map(p => `[${coordinate(p[0], 0)},${coordinate(p[1], 1)}]`).join(",");
          output.push(`      path([${positions}], ${options.join(", ")})`);
        }
        continue;
      }
      let points = shapes[0];
      for (let hole of shapes.slice(1)) {
        if (style['fill-rule'] !== 'evenodd') throw Error('Unexpected compound brand path');
        if (Math.sign(area(hole)) === Math.sign(area(points))) hole = hole.slice().reverse();
        points = [...points, points[0], hole[0], ...hole, hole[0], points[0]];
      }
      if (style.fill !== 'none') options.push('closed: true');
      const positions = points.map(p => `[${coordinate(p[0], 0)},${coordinate(p[1], 1)}]`).join(',');
      output.push(`      path([${positions}], ${options.join(', ')})`);
    }
  }
  return output.join('\n');
}
function generate() {
  let text = '  // Generated from assets/brand SVGs by tools/generate-brand.cjs.\n';
  for (const [name, file, width, height] of [['BrandWordmark','miilo-wordmark.svg',184,72], ['BrandSymbol','miilo-symbol.svg',64,64]]) {
    const svg = fs.readFileSync(path.join(root, 'assets/brand', file), 'utf8');
    text += `  component ${name}(w = ${name === 'BrandWordmark' ? 86 : 24}, ink = foreground) {\n    Canvas {\n      width: w; height: w * ${height} / ${width}\n      let k = width / ${width}\n${canvas(svg)}\n    }\n  }\n`;
  }
  return text;
}
if (require.main === module) {
  const file = path.join(root, 'widgets/brand.inc'), text = generate();
  if (process.argv.includes('--check')) {
    if (fs.readFileSync(file, 'utf8') !== text) throw Error('Brand canvas is stale; run node tools/generate-brand.cjs');
    console.log('Brand canvas matches the canonical SVG artwork.');
  } else fs.writeFileSync(file, text);
}
module.exports = {generate, contours};
