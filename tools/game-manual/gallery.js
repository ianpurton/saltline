(async () => {
  const T=lt, canvas=document.querySelector('#preview'),status=document.querySelector('#status');
  const renderer=new T.WebGLRenderer({canvas,antialias:true,preserveDrawingBuffer:true,alpha:false});
  renderer.setPixelRatio(1);renderer.setSize(512,512,false);
  renderer.outputColorSpace=T.SRGBColorSpace;renderer.toneMapping=T.ACESFilmicToneMapping;renderer.toneMappingExposure=1.4;
  renderer.shadowMap.enabled=true;renderer.shadowMap.type=T.PCFSoftShadowMap;
  const scene=new T.Scene();scene.background=new T.Color('#f5f1e8');
  const camera=new T.PerspectiveCamera(32,1,.001,10000);
  scene.add(new T.HemisphereLight(0xffffff,0x797064,3));
  const key=new T.DirectionalLight(0xfff1dd,4);key.position.set(4,7,5);scene.add(key);
  const fill=new T.DirectionalLight(0xcbdfff,2);fill.position.set(-4,2,-3);scene.add(fill);
  const environment=new T.Scene();environment.background=new T.Color(.75,.75,.75);
  const pmrem=new T.PMREMGenerator(renderer);const env=pmrem.fromScene(environment);scene.environment=env.texture;
  const context=await SaltlineAssets.initialize(renderer);context.scene=scene;context.camera=camera;
  const entries=SaltlineAssets.entries();let object=null,current=null,view='three-quarter',center=new T.Vector3(),distance=3,yaw=.8,pitch=.3,wire=false;
  function render(){camera.position.set(center.x+Math.sin(yaw)*Math.cos(pitch)*distance,center.y+Math.sin(pitch)*distance,center.z+Math.cos(yaw)*Math.cos(pitch)*distance);camera.lookAt(center);renderer.render(scene,camera);}
  function frame(){
    object.updateMatrixWorld(true);
    // Precise vertex bounds include posed skinned vertices, not static bind-pose boxes.
    const box=new T.Box3().setFromObject(object,true),size=box.getSize(new T.Vector3());box.getCenter(center);
    const sphere=box.getBoundingSphere(new T.Sphere());distance=sphere.radius/Math.sin(camera.fov*Math.PI/360)*1.13;
    yaw=current.id==='pac9'?1.15:.8;pitch=.30;
    if(view==='side'){yaw=Math.PI/2;pitch=.05;}
    if(view==='front'){yaw=0;pitch=.05;}
    if(view==='top'){yaw=0;pitch=Math.PI/2-.001;}
    camera.near=Math.max(.0001,distance/1000);camera.far=Math.max(100,distance*10);camera.updateProjectionMatrix();
    return {size:size.toArray(),center:center.toArray()};
  }
  async function show(id,requestedView='three-quarter'){
    view=requestedView;
    const entry=entries.find(e=>e.id===id);if(!entry)throw Error('Unknown asset '+id);
    if(object)scene.remove(object);
    object=await entry.build(context);current=entry;scene.add(object);
    object.traverse(n=>{if(n.isMesh){n.frustumCulled=false;const ms=Array.isArray(n.material)?n.material:[n.material];for(const m of ms){if(m)m.wireframe=wire;}}});
    const bounds=frame();await renderer.compileAsync(scene,camera);render();
    await new Promise(resolve=>requestAnimationFrame(resolve));render();renderer.getContext().finish();
    status.textContent=entry.label;
    document.querySelector('#details').textContent=`${entry.source} · ${bounds.size.map(x=>x.toFixed(3)).join(' × ')} m`;
    for(const button of document.querySelectorAll('nav button'))button.setAttribute('aria-pressed',String(button.dataset.asset===id));
    return {id,source:entry.source,...bounds};
  }
  window.assetGallery={specifications:{hound:$v,gull:xm,props:Gi},entries:entries.map(({id,label,source,size})=>({id,label,source,size})),show,png:()=>canvas.toDataURL('image/png'),render};
  for(const e of entries){const b=document.createElement('button');b.textContent=e.label;b.dataset.asset=e.id;b.setAttribute('aria-pressed','false');b.onclick=()=>show(e.id).catch(fail);document.querySelector('#assets').append(b);}
  let drag=null;canvas.onpointerdown=e=>{drag=[e.clientX,e.clientY];canvas.setPointerCapture(e.pointerId)};
  canvas.onpointerup=()=>drag=null;canvas.onpointermove=e=>{if(drag){yaw-=(e.clientX-drag[0])*.01;pitch=Math.max(-1.3,Math.min(1.3,pitch+(e.clientY-drag[1])*.01));drag=[e.clientX,e.clientY];render();}};
  canvas.onwheel=e=>{e.preventDefault();distance*=Math.exp(e.deltaY*.001);render();};
  document.querySelector('#wireframe').onchange=e=>{wire=e.target.checked;if(object)object.traverse(n=>{if(n.isMesh)for(const m of (Array.isArray(n.material)?n.material:[n.material]))m.wireframe=wire;});render();};
  document.querySelector('#reset').onclick=()=>{frame();render();};
  document.querySelector('#download').onclick=()=>{const a=document.createElement('a');a.href=canvas.toDataURL('image/png');a.download=current.id+'.png';a.click();};
  await show(new URLSearchParams(location.search).get('asset')||'hound');window.galleryReady=true;
  function fail(error){status.classList.add('error');status.textContent=error.stack||String(error);window.galleryError=String(error);}
})().catch(error=>{document.querySelector('#status').textContent=error.stack||String(error);window.galleryError=String(error);console.error(error);});
