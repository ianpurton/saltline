// Render the actual reference shader to a linear HDR panorama, without its sun disk.
const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const root = path.resolve(__dirname, '..');
(async () => {
  const server = http.createServer((req,res) => {
    const url = new URL(req.url,'http://localhost');
    if(url.pathname==='/') {res.setHeader('Content-Type','text/html');res.end('<script type="module" src="/scripts/sky-export/browser.js"></script>');return;}
    const file=path.resolve(root,'.'+decodeURIComponent(url.pathname));
    if(!file.startsWith(root+path.sep)){res.writeHead(403).end();return;}
    fs.readFile(file,(error,data)=>{if(error){res.writeHead(404).end();return;}
      res.setHeader('Content-Type',file.endsWith('.js')?'text/javascript':'application/octet-stream');res.end(data);});
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  let browser;
  try {
    browser=await chromium.launch({headless:true,args:['--no-sandbox','--use-angle=swiftshader','--enable-unsafe-swiftshader','--enable-unsafe-webgpu','--use-vulkan=swiftshader']});
    const page=await browser.newPage(); page.setDefaultTimeout(180000);
    await page.goto(`http://127.0.0.1:${server.address().port}/`);
    await page.waitForFunction(()=>window.skyResult||window.skyError);
    const error=await page.evaluate(()=>window.skyError);if(error)throw Error(error);
    const {width,height,pixels,samples,noiseHash,validation}=await page.evaluate(()=>window.skyResult);
    const floatBytes=Buffer.from(pixels,'base64');
    const floats=new Float32Array(floatBytes.buffer,floatBytes.byteOffset,floatBytes.length/4);
    const parts=[Buffer.from(`#?RADIANCE\nFORMAT=32-bit_rle_rgbe\n\n-Y ${height} +X ${width}\n`)];
    let min=Infinity,max=0;
    for(let y=0;y<height;y++) {
      const channels=Array.from({length:4},()=>Buffer.alloc(width));
      for(let x=0;x<width;x++) {
        const offset=((height-1-y)*width+x)*4;
        const rgb=[floats[offset],floats[offset+1],floats[offset+2]];
        if(rgb.some(v=>!Number.isFinite(v)||v<0))throw Error('Invalid HDR pixel');
        const peak=Math.max(...rgb); min=Math.min(min,...rgb);max=Math.max(max,peak);
        const exponent=peak<1e-32?0:Math.floor(Math.log2(peak))+1;
        const scale=peak<1e-32?0:256/2**exponent;
        for(let c=0;c<3;c++)channels[c][x]=Math.floor(rgb[c]*scale);
        channels[3][x]=peak<1e-32?0:exponent+128;
      }
      parts.push(Buffer.from([2,2,width>>8,width&255]));
      for(const channel of channels)for(let x=0;x<width;x+=128){const chunk=channel.subarray(x,x+128);parts.push(Buffer.from([chunk.length]),chunk);}
    }
    if(max>6.001)throw Error('Environment capture must be clamped to 6');
    const destination=path.join(root,'assets/sky/reference-clouds.hdr');
    fs.writeFileSync(destination,Buffer.concat(parts));
    const inputs=['references/src/render/sky.js','references/src/core/config.js','scripts/export-sky.cjs','scripts/sky-export/browser.js','scripts/sky-export/validate.js','crates/game/src/sky.wesl'];
    const sha=data=>crypto.createHash('sha256').update(data).digest('hex');
    fs.writeFileSync(destination.replace('.hdr','.json'),JSON.stringify({width,height,time:0,environment:true,sunDirection:[.48,.225,-.86],min,max,noiseHash,samples,validation,sha256:sha(fs.readFileSync(destination)),inputs:Object.fromEntries(inputs.map(f=>[f,sha(fs.readFileSync(path.join(root,f)))]))},null,2)+'\n');
    console.log(`Exported ${width}x${height} reference environment, range ${min}..${max}`);
  } finally {if(browser)await browser.close();await new Promise(resolve=>server.close(resolve));}
})().catch(error=>{console.error(error);process.exitCode=1;});
