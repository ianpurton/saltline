import {validateSky} from './validate.js';
import {createNoiseTexture,Hy} from '/references/src/render/sky.js';
import {WebGLRenderer,WebGLRenderTarget,FloatType,RGBAFormat,Scene,Mesh,PlaneGeometry,Camera,ShaderMaterial,Vector3} from '/references/src/vendor/three-r180.js';
try {
  const noise=createNoiseTexture();
  let noiseHash=2166136261;
  for(const byte of noise.image.data)noiseHash=Math.imul(noiseHash^byte,16777619)>>>0;
  const reference=Hy(noise);
  const width=1024,height=512;
  const renderer=new WebGLRenderer();renderer.setSize(width,height);
  const uniforms=reference.material.uniforms;
  uniforms.uSunDir.value=new Vector3(.48,.225,-.86).normalize();
  uniforms.uEnv.value=1;
  // equirectangular panorama: top is +Y, atan2(z,x) maps horizontally.
  const fragmentShader=reference.material.fragmentShader.replace('varying vec3 vDir;','varying vec2 vUv;')
    .replace('vec3 d = normalize(vDir);','float theta=(vUv.x-0.5)*6.28318530718; float phi=(1.0-vUv.y)*3.14159265359; vec3 d=vec3(sin(phi)*cos(theta),cos(phi),sin(phi)*sin(theta));');
  const material=new ShaderMaterial({uniforms,toneMapped:false,depthTest:false,depthWrite:false,
    vertexShader:'varying vec2 vUv; void main(){vUv=uv;gl_Position=vec4(position.xy,0.,1.);}',fragmentShader});
  const scene=new Scene();scene.add(new Mesh(new PlaneGeometry(2,2),material));
  const target=new WebGLRenderTarget(width,height,{type:FloatType,format:RGBAFormat,depthBuffer:false});
  renderer.setRenderTarget(target);renderer.render(scene,new Camera());
  const pixels=new Float32Array(width*height*4);renderer.readRenderTargetPixels(target,0,0,width,height,pixels);
  // Fixed-time directional samples, retained as numerical shader-port evidence.
  const samples=[];
  for(const time of [0,1,30]) {
    uniforms.uTime.value=time;renderer.render(scene,new Camera());
    const data=new Float32Array(width*height*4);renderer.readRenderTargetPixels(target,0,0,width,height,data);
    for(const [x,y] of [[100,400],[600,320],[750,260],[500,180]])samples.push({time,x,y,color:Array.from(data.slice((y*width+x)*4,(y*width+x)*4+3))});
  }
  let binary='';const bytes=new Uint8Array(pixels.buffer);
  for(let i=0;i<bytes.length;i+=32768)binary+=String.fromCharCode(...bytes.subarray(i,i+32768));
  const validation=await validateSky(samples,noise.image.data,uniforms.uSunDir.value.toArray());
  window.skyResult={width,height,pixels:btoa(binary),noiseHash,samples,validation};
} catch(error) {window.skyError=error.stack;}
