const fs=require('node:fs');
const path=require('node:path');
const http=require('node:http');
const crypto=require('node:crypto');
const {chromium}=require('playwright');
const root=path.join(__dirname,'gallery'),out=path.join(__dirname,'assets/thumbnails');
const mime={'.html':'text/html','.js':'text/javascript','.json':'application/json'};
(async()=>{
 fs.mkdirSync(out,{recursive:true});
 const server=http.createServer((req,res)=>{
  const pathname=decodeURIComponent(new URL(req.url,'http://localhost').pathname);
  const file=path.resolve(root,'.'+(pathname==='/'?'/index.html':pathname));
  if(!file.startsWith(root+path.sep)){res.writeHead(403).end();return;}
  fs.readFile(file,(error,data)=>{if(error){res.writeHead(404).end();return;}res.setHeader('Content-Type',mime[path.extname(file)]||'application/octet-stream');res.end(data);});
 });
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));let browser;
 try {
  browser=await chromium.launch({headless:true,args:['--no-sandbox','--use-angle=swiftshader','--enable-unsafe-swiftshader']});
  const page=await browser.newPage({viewport:{width:1000,height:800},deviceScaleFactor:1});page.setDefaultTimeout(180000);
  const errors=[];page.on('pageerror',e=>errors.push(e.message));page.on('console',m=>{if(m.type()==='error'&&!m.text().includes('favicon'))errors.push(m.text());});
  await page.goto(`http://127.0.0.1:${server.address().port}/`);
  await page.waitForFunction(()=>window.galleryReady||window.galleryError);
  const error=await page.evaluate(()=>window.galleryError);if(error)throw Error(error);
  const entries=await page.evaluate(()=>assetGallery.entries);const results=[];
  const shots=[...entries,...['side','front','top'].map(view=>({...entries.find(e=>e.id==='hound'),view})),{...entries.find(e=>e.id==='pac9'),view:'side'}];
  for(const e of shots){
    const view=e.view||'three-quarter';
    const info=await page.evaluate(({id,view})=>assetGallery.show(id,view),{id:e.id,view});
    const shot=await page.evaluate(()=>{
      const canvas=document.querySelector('#preview'),context=new OffscreenCanvas(512,512).getContext('2d');context.drawImage(canvas,0,0);
      const pixels=context.getImageData(0,0,512,512).data;let count=0;
      for(let i=0;i<pixels.length;i+=4)if(Math.abs(pixels[i]-245)+Math.abs(pixels[i+1]-241)+Math.abs(pixels[i+2]-232)>40)count++;
      return {png:assetGallery.png().split(',')[1],coverage:count/(512*512)};
    });
    if(shot.coverage<.005)throw Error('Blank thumbnail: '+e.id);
    const filename=e.id+(e.view?'-'+e.view:'')+'.png';
    fs.writeFileSync(path.join(out,filename),Buffer.from(shot.png,'base64'));
    results.push({...e,...info,filename,view,coverage:shot.coverage,sourceSha256:crypto.createHash('sha256').update(fs.readFileSync(path.join(root,e.source))).digest('hex')});console.log('Rendered '+e.id);
  }
  // Verify gallery controls against the same model path used for captures.
  await page.locator('button[data-asset="hound"]').click();
  await page.waitForFunction(()=>document.querySelector('#status').textContent==='Hound / H-03');
  await page.locator('#wireframe').check();
  const wireframe=await page.evaluate(()=>document.querySelector('#wireframe').checked);
  if(!wireframe)throw Error('Wireframe control failed');
  await page.locator('#wireframe').uncheck();
  await page.locator('#reset').click();
  const downloadPromise=page.waitForEvent('download');await page.locator('#download').click();
  const download=await downloadPromise;if(download.suggestedFilename()!=='hound.png')throw Error('Download filename mismatch');
  if(errors.length)throw Error(errors.join('\n'));
  const inputs={};for(const file of ['shared/reference.js','registry.js','gallery.js'])inputs[file]=crypto.createHash('sha256').update(fs.readFileSync(path.join(root,file))).digest('hex');
  fs.writeFileSync(path.join(out,'render-inputs.json'),JSON.stringify(inputs,null,2)+'\n');
  fs.writeFileSync(path.join(out,'specification.json'),JSON.stringify(await page.evaluate(()=>assetGallery.specifications),null,2)+'\n');
  fs.writeFileSync(path.join(out,'manifest.json'),JSON.stringify(results,null,2)+'\n');
  console.log(`Verified ${results.length} nonblank 512×512 thumbnails with no browser errors.`);
 } finally {if(browser)await browser.close();await new Promise(resolve=>server.close(resolve));}
})().catch(e=>{console.error(e);process.exitCode=1;});
