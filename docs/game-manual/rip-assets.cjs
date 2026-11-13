// Explicit import step: refresh source files from the reference, never during a normal build.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const acorn = require('acorn');
const root = __dirname;
const html = fs.readFileSync(path.resolve(root, '../../references/index.html'), 'utf8');
const code = html.slice(html.indexOf('<script>') + 8, html.lastIndexOf('</script>'));
const statements = acorn.parse(code, {ecmaVersion: 'latest'}).body[0].expression.callee.body.body;
const functions = new Map(statements.filter(n => n.type === 'FunctionDeclaration').map(n => [n.id.name, n]));
const groups = {
  hound: ['Z_'], gull: ['j_'], pac9: ['w_'], containers: ['M3'],
  jersey_barrier: ['S3'], light_mast: ['T3'], fence_panel: ['A3'], guard_booth: ['R3'], gate_arm: ['C3'],
  bollard_mooring: ['P3'], bollard_post: ['I3'], sign_post: ['z3'], floodlight_stand: ['D3'],
  stairs_metal: ['k3'], catwalk: ['F3'], shelf_rack: ['U3'], reefer_unit: ['N3'], relay_node: ['B3'],
  barrel: ['HA'], pallet: ['VA'], pallet_stack: ['WA'], crate_wood: ['GA'], crate_stack: ['qA'],
  ibc_tote: ['XA'], cable_reel: ['YA'], tire_stack: ['$A'], jerry_can: ['ZA'], traffic_cone: ['jA'],
  gas_cylinders: ['JA'], pipe_stack: ['KA'], tarp_cargo: ['QA'], sandbags: ['tR'],
  cardboard_boxes: ['nR'], forklift: ['oR'], generator: ['rR'], dumpster: ['aR'], toolbox_cart: ['lR'],
};
const manifest = {reference: 'references/index.html', sha256: crypto.createHash('sha256').update(html).digest('hex'), files: {}};
const extracted = new Set();
for (const [file, names] of Object.entries(groups)) {
 const chunks = names.map(name => {
  const n = functions.get(name); if (!n) throw Error(`Missing builder ${name}`);
  extracted.add(n); return code.slice(n.start, n.end);
 });
 const destination = `assets/${file}.js`;
 fs.writeFileSync(path.join(root, 'gallery', destination), `// Extracted original builder(s): ${names.join(', ')}. Shared dependencies are in shared/reference.js.\n` + chunks.join('\n') + '\n');
 manifest.files[destination] = names.map(name => ({symbol: name, anchor: `function ${name}(`}));
}
const boot = statements.find(n => code.slice(n.start,n.end) === 'window.__frames=0;');
if (!boot) throw Error('Bootstrap boundary not found');
// Preserve the bundled Three.js, original geometry/material helpers and registry wiring.
// Retain support declarations intact; never start the FPS or its animation loop.
const shared = statements.filter(n => n.start < boot.start && !extracted.has(n)).map(n => code.slice(n.start,n.end)).join('\n');
fs.writeFileSync(path.join(root, 'gallery/shared/reference.js'), '// Extracted dependency bundle: Three.js + original helpers/materials/registrations. FPS bootstrap excluded.\n' + shared + '\n');
fs.writeFileSync(path.join(root,'gallery/provenance.json'), JSON.stringify(manifest,null,2)+'\n');
const scripts = [...Object.keys(manifest.files), 'shared/reference.js', 'registry.js', 'gallery.js'];
fs.writeFileSync(path.join(root,'gallery/index.html'), `<!doctype html>
<html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>SALTLINE asset gallery</title>
<style>
*{box-sizing:border-box}body{margin:0;background:#f5f1e8;color:#27343c;font:14px system-ui}header{padding:20px 24px;border-bottom:1px solid #d3cec4}h1{margin:0 0 6px}header p{margin:0;color:#50758a}.layout{display:grid;grid-template-columns:220px 1fr;height:calc(100vh - 110px)}nav{overflow:auto;padding:12px}nav button{display:block;width:100%;text-align:left;border:0;background:transparent;padding:8px;cursor:pointer}nav button[aria-pressed=true]{background:#e0d6bd}.viewer{position:relative;min-width:0;overflow:auto}canvas{display:block;width:min(100%,680px);height:auto}#status{padding:12px 24px}#details{padding:8px 24px;color:#50758a}#controls{padding:8px 24px}label,button{margin-right:12px}.error{color:#a94c30}
@media(max-width:650px){.layout{grid-template-columns:150px 1fr}}
</style><header><h1>SALTLINE / Asset gallery</h1><p>Original JavaScript geometry and procedural materials. Drag to orbit; scroll to zoom.</p></header>
<div class="layout"><nav aria-label="Assets" id="assets"></nav><main class="viewer"><div id="status">Preparing renderer…</div><canvas id="preview" width="512" height="512" aria-label="Interactive asset preview"></canvas><div id="details"></div><div id="controls"><label><input type="checkbox" id="wireframe"> Wireframe</label><button id="reset">Reset camera</button><button id="download">Download PNG</button></div></main></div>
${scripts.map(file=>`<script src="${file}"></script>`).join('\n')}</html>\n`);
console.log(`Extracted ${Object.keys(groups).length} asset JS files plus shared dependencies; no JSON geometry.`);
