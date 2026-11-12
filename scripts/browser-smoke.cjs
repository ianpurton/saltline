// Exercise the rendered game, not just WASM initialization. Requires a running web server.
const { chromium } = require('playwright');
const fs = require('node:fs');
const assert = require('node:assert/strict');
const output = process.env.WEB_OUTPUT_DIR || 'tmp/screenshots/browser-smoke';
fs.mkdirSync(output, { recursive: true });

async function changedPixels(page, before, after, region) {
  return page.evaluate(async ({ before, after, region }) => {
    const pixels = async encoded => {
      const bitmap = await createImageBitmap(await (await fetch(`data:image/png;base64,${encoded}`)).blob());
      const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
      const ctx = canvas.getContext('2d');
      ctx.drawImage(bitmap, 0, 0);
      const [x, y, w, h] = region;
      return ctx.getImageData(Math.floor(x * bitmap.width), Math.floor(y * bitmap.height),
        Math.floor(w * bitmap.width), Math.floor(h * bitmap.height)).data;
    };
    const a = await pixels(before), b = await pixels(after);
    let changed = 0;
    for (let i = 0; i < a.length; i += 4) {
      if (Math.abs(a[i] - b[i]) + Math.abs(a[i + 1] - b[i + 1]) + Math.abs(a[i + 2] - b[i + 2]) > 60) changed++;
    }
    return changed / (a.length / 4);
  }, { before: before.toString('base64'), after: after.toString('base64'), region });
}

(async () => {
  const errors = [], messages = [];
  const browser = await chromium.launch({ headless: true, channel: process.env.WEB_BROWSER_CHANNEL || undefined, args: JSON.parse(process.env.WEB_BROWSER_ARGS || '[]') });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
    page.setDefaultTimeout(180000);
    page.on('console', m => {
      const text = m.text(); messages.push(`${m.type()} ${text}`);
      if (m.type() === 'error' || /ERROR |panicked at|Validation Error|context lost/i.test(text)) errors.push(text);
      if (/AdapterInfo/.test(text)) console.log(text);
    });
    page.on('pageerror', e => errors.push(e.stack || e.message));
    if (process.env.WEB_DEBUG_GL_FINISH) await page.addInitScript(() => {
      const original = WebGL2RenderingContext.prototype.bufferSubData;
      WebGL2RenderingContext.prototype.bufferSubData = function(...args) {
        original.apply(this, args); this.finish();
      };
    });
    await page.goto(process.env.WEB_URL || 'http://127.0.0.1:8000');
    await page.waitForFunction(() => document.querySelector('#status').hidden || document.querySelector('#status').textContent.includes('Unable to start'));
    assert(await page.locator('#status').isHidden(), await page.locator('#status').innerText());
    await page.waitForTimeout(15000);
    const shot = name => page.screenshot({ path: `${output}/${name}.png`, timeout: 60000 });
    await shot('title');
    await page.mouse.click(130, 615);
    await page.waitForTimeout(4000);
    const deployed = await shot('deployed');
    await page.mouse.click(640, 360);
    await page.waitForTimeout(500);
    assert(await page.evaluate(() => !!document.pointerLockElement), 'Deploy did not enable pointer lock');
    await page.mouse.move(900, 380);
    await page.keyboard.down('w');
    await page.waitForTimeout(1800);
    await page.keyboard.up('w');
    const moved = await shot('moved');
    // Ignore HUD borders, compass, and crosshair. Rain alone must not pass this check.
    const movement = await changedPixels(page, deployed, moved, [0.15, 0.20, 0.70, 0.45]);
    assert(movement > 0.12, `World remained visually frozen after movement/look (${(movement * 100).toFixed(1)}% changed pixels)`);
    await page.mouse.down({ button: 'right' });
    await page.waitForTimeout(1000);
    const aimed = await shot('aimed');
    await page.mouse.up({ button: 'right' });
    const aiming = await changedPixels(page, moved, aimed, [0.55, 0.58, 0.35, 0.30]);
    assert(aiming > 0.03, 'Rifle did not visibly respond to aiming');
    await page.mouse.click(640, 360);
    await page.keyboard.press('r');
    await page.waitForTimeout(600);
    await shot('reload');
    await page.keyboard.press('Escape');
    await page.waitForTimeout(500);
    await shot('paused');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);
    await shot('resumed');
    assert.equal(errors.length, 0, errors.join('\n'));
    console.log(`BROWSER GAMEPLAY PASSED: world changed ${(movement * 100).toFixed(1)}%, aiming changed ${(aiming * 100).toFixed(1)}%`);
  } finally {
    fs.mkdirSync('tmp/logs', { recursive: true });
    fs.writeFileSync('tmp/logs/browser-smoke.log', messages.join('\n') + '\n' + errors.join('\n'));
    await browser.close();
  }
})().catch(e => { console.error(e); process.exitCode = 1; });
