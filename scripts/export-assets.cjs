// Execute the existing catalogue; never parse or print the monolithic reference.
const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const validator = require('gltf-validator');
const root = path.resolve(__dirname, '..');
const out = path.join(root, 'assets/models');
const hash = data => crypto.createHash('sha256').update(data).digest('hex');
(async () => {
  const selected = process.argv.slice(2);
  require('../tools/game-manual/prepare-gallery.cjs')();
  const gallery = 'target/game-manual/gallery/';
  const source = fs.readFileSync(path.join(root, gallery, 'index.html'), 'utf8');
  const scripts = [...source.matchAll(/<script src="([^"]+)"/g)].map(m => gallery + m[1]).filter(s => !s.endsWith('/gallery.js'));
  const html = `<!doctype html><script type="importmap">{"imports":{"three":"/node_modules/three/build/three.module.js"}}</script>${scripts.map(s => `<script src="/${s}"></script>`).join('')}<script type="module" src="/scripts/asset-export/browser.js"></script>`;
  const server = http.createServer((req, res) => {
    const url = new URL(req.url, 'http://localhost');
    if (url.pathname === '/') {res.setHeader('Content-Type', 'text/html'); res.end(html); return;}
    const file = path.resolve(root, '.' + decodeURIComponent(url.pathname));
    if (!file.startsWith(root + path.sep)) {res.writeHead(403).end(); return;}
    fs.readFile(file, (err, data) => {
      if (err) {res.writeHead(404).end(); return;}
      res.setHeader('Content-Type', file.endsWith('.js') ? 'text/javascript' : 'application/octet-stream'); res.end(data);
    });
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  let browser;
  try {
    browser = await chromium.launch({headless: true, args: ['--no-sandbox', '--use-angle=swiftshader', '--enable-unsafe-swiftshader']});
    const page = await browser.newPage(); page.setDefaultTimeout(180000);
    const errors = []; page.on('pageerror', e => errors.push(e.message));
    page.on('console', m => {if (m.type() === 'error' && !m.text().includes('favicon')) errors.push(m.text());});
    await page.goto(`http://127.0.0.1:${server.address().port}/`);
    await page.waitForFunction(() => window.exportReady || window.exportError);
    const failure = await page.evaluate(() => window.exportError); if (failure) throw Error(failure);
    const ids = await page.evaluate(() => window.exportIds);
    for (const id of selected) if (!ids.includes(id)) throw Error(`Unknown asset: ${id}`);
    fs.mkdirSync(out, {recursive: true});
    const manifestPath = path.join(out, 'manifest.json');
    const previous = fs.existsSync(manifestPath) ? JSON.parse(fs.readFileSync(manifestPath)) : {assets: {}};
    const inputs = Object.fromEntries([...scripts, 'scripts/asset-export/browser.js', 'scripts/export-assets.cjs', 'package-lock.json'].map(f => [f, hash(fs.readFileSync(path.join(root, f)))]));
    const assets = selected.length ? previous.assets : {};
    for (const id of selected.length ? selected : ids) {
      const result = await page.evaluate(id => window.exportAsset(id), id);
      if (result.backdrop) {fs.writeFileSync(path.join(out, `${id}.json`), JSON.stringify(result.backdrop, null, 2) + '\n'); delete result.backdrop;}
      const bytes = Buffer.from(result.base64, 'base64'); delete result.base64;
      const previewDir = path.join(root, 'target/asset-export'); fs.mkdirSync(previewDir, {recursive: true});
      fs.writeFileSync(path.join(previewDir, `${id}.png`), Buffer.from(result.preview, 'base64')); delete result.preview;
      const report = await validator.validateBytes(new Uint8Array(bytes), {uri: `${id}.glb`, maxIssues: 100});
      if (report.issues.numErrors) throw Error(`${id}: ${JSON.stringify(report.issues)}`);
      fs.writeFileSync(path.join(out, `${id}.glb`), bytes);
      assets[id] = {...result, file: `${id}.glb`, bytes: bytes.length, sha256: hash(bytes), inputs, validation: report.issues};
      console.log(`Exported and reloaded ${id}: ${bytes.length} bytes`);
    }
    if (errors.length) throw Error(errors.join('\n'));
    fs.writeFileSync(manifestPath, JSON.stringify({threeRevision: '180', seed: 7, textureQuality: 256, pose: 'catalogue inspection', assets}, null, 2) + '\n');
    console.log(`Validated ${Object.keys(assets).length}/${ids.length} catalogue assets.`);
  } finally {if (browser) await browser.close(); await new Promise(resolve => server.close(resolve));}
})().catch(e => {console.error(e); process.exitCode = 1;});
