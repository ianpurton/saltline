// Execute the reference generators, preserving their linear data channels and +Z row order.
const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const crypto = require('node:crypto');
const zlib = require('node:zlib');
const { chromium } = require('playwright');
const root = path.resolve(__dirname, '..');
(async () => {
  const server = http.createServer((req, res) => {
    const file = path.resolve(root, '.' + new URL(req.url, 'http://localhost').pathname);
    if (!file.startsWith(root + path.sep)) { res.end('<!doctype html>'); return; }
    fs.readFile(file, (error, data) => {
      if (error) { res.writeHead(404).end(); return; }
      res.setHeader('Content-Type', file.endsWith('.js') ? 'text/javascript' : 'application/octet-stream'); res.end(data);
    });
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  let browser;
  try {
    browser = await chromium.launch({headless: true, args: ['--no-sandbox']});
    const page = await browser.newPage();
    await page.goto(`http://127.0.0.1:${server.address().port}/`);
    const maps = await page.evaluate(async () => {
      const { Rb, Cb, Pb } = await import('/references/src/world/ground/geometry.js');
      const font = new FontFace('Big Shoulders Display', 'url(/references/public/fonts/fC1_PZJEZG-e9gHhdI4-NBbfd2ys3SjJCx1czNDuDJAM2w.woff2)', {weight: '900'});
      document.fonts.add(await font.load());
      const encode = (size, data) => {
        let raw = '';
        for (let i=0; i<data.length; i+=16384) raw += String.fromCharCode(...data.subarray(i,i+16384));
        return {size, pixels: btoa(raw)};
      };
      const { det, nrm } = Cb(512);
      const marks = Pb(2048, 8).image;
      return { mask: encode(1024, Rb(1024)), detail: encode(512, det), normal: encode(512, nrm), marks: marks.toDataURL('image/png').split(',')[1] };
    });
    const hash = data => crypto.createHash('sha256').update(data).digest('hex');
    const manifest = { maskRect: [-44, -40, 80], marksRect: [-40, -30, 70, 60], colorSpace: 'linear data', rowOrder: 'increasing world Z', maps: {}, inputs: {} };
    // Encode data maps directly: a canvas would premultiply their non-opacity alpha channel.
    const png = (size, pixels) => {
      const chunk = (type, data) => {
        const name = Buffer.from(type); let crc=0xffffffff;
        for (const byte of Buffer.concat([name,data])) {
          crc ^= byte;
          for(let bit=0;bit<8;bit++) crc=(crc>>>1)^((crc&1)?0xedb88320:0);
        }
        const header=Buffer.alloc(4); header.writeUInt32BE(data.length);
        const trailer=Buffer.alloc(4); trailer.writeUInt32BE((crc^0xffffffff)>>>0);
        return Buffer.concat([header,name,data,trailer]);
      };
      const ihdr=Buffer.alloc(13);ihdr.writeUInt32BE(size,0);ihdr.writeUInt32BE(size,4);ihdr[8]=8;ihdr[9]=6;
      const scanlines=Buffer.alloc(size*(size*4+1));
      for(let y=0;y<size;y++) pixels.copy(scanlines,y*(size*4+1)+1,y*size*4,(y+1)*size*4);
      return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]),chunk('IHDR',ihdr),chunk('IDAT',zlib.deflateSync(scanlines)),chunk('IEND',Buffer.alloc(0))]);
    };
    for (const [name, value] of Object.entries(maps)) {
      const data = typeof value === 'string' ? Buffer.from(value,'base64') : png(value.size,Buffer.from(value.pixels,'base64'));
      fs.writeFileSync(path.join(root, 'assets/ground', `${name}.png`), data);
      manifest.maps[name] = { sha256: hash(data), bytes: data.length };
      console.log(`${name}: ${data.length} bytes`);
    }
    for (const file of ['references/src/world/ground/geometry.js', 'scripts/export-ground.cjs', 'references/public/fonts/fC1_PZJEZG-e9gHhdI4-NBbfd2ys3SjJCx1czNDuDJAM2w.woff2']) manifest.inputs[file] = hash(fs.readFileSync(path.join(root, file)));
    fs.writeFileSync(path.join(root, 'assets/ground/manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
  } finally { if (browser) await browser.close(); await new Promise(resolve => server.close(resolve)); }
})().catch(error => { console.error(error); process.exitCode = 1; });
