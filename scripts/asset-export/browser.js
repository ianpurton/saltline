import {GLTFExporter} from '/node_modules/three/examples/jsm/exporters/GLTFExporter.js';
import {GLTFLoader} from '/node_modules/three/examples/jsm/loaders/GLTFLoader.js';
import * as THREE from 'three';

const T = lt;
let randomState = 7;
Math.random = () => {randomState = (Math.imul(randomState, 1664525) + 1013904223) >>> 0; return randomState / 4294967296;};
const renderer = new T.WebGLRenderer({antialias: true, preserveDrawingBuffer: true});
renderer.setSize(512, 512);
const textureCache = new Map();
let usedTextures = new Set();
function portableTexture(source, slot) {
  if (!source) return null;
  usedTextures.add(source);
  if (textureCache.has(source)) return textureCache.get(source);
  const {width, height} = source.image;
  const canvas = document.createElement('canvas'); canvas.width = width; canvas.height = height;
  let pixels, flipY = source.flipY;
  if (source.image.getContext) {
    canvas.getContext('2d').drawImage(source.image, 0, 0);
    pixels = canvas.getContext('2d').getImageData(0, 0, width, height).data;
  } else {
    const gl = renderer.getContext(), previous = gl.getParameter(gl.FRAMEBUFFER_BINDING), fb = gl.createFramebuffer();
    gl.bindFramebuffer(gl.FRAMEBUFFER, fb);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, renderer.properties.get(source).__webglTexture, 0);
    try {
      if (gl.checkFramebufferStatus(gl.FRAMEBUFFER) !== gl.FRAMEBUFFER_COMPLETE) throw Error('Unreadable texture ' + source.name);
      pixels = new Uint8Array(width * height * 4);
      gl.readPixels(0, 0, width, height, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
      if (gl.getError() !== gl.NO_ERROR) throw Error('Texture readback error');
    } finally {gl.bindFramebuffer(gl.FRAMEBUFFER, previous); gl.deleteFramebuffer(fb);}
    canvas.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(pixels), width, height), 0, 0);
    flipY = false; // Raw row zero is v=0, also row zero in the embedded PNG.
  }
  let nonzero = 0, varied = 0;
  for (let i = 0; i < pixels.length; i += 4) {
    if (pixels[i] || pixels[i + 1] || pixels[i + 2]) nonzero++;
    if (pixels[i] !== pixels[0] || pixels[i + 1] !== pixels[1] || pixels[i + 2] !== pixels[2] || pixels[i + 3] !== pixels[3]) varied++;
  }
  if (!nonzero && !pixels.some((v, i) => i % 4 === 3 && v)) throw Error('Empty texture ' + source.name); 
  const texture = new THREE.CanvasTexture(canvas);
  for (const key of ['name', 'colorSpace', 'wrapS', 'wrapT', 'magFilter', 'minFilter', 'channel']) texture[key] = source[key];
  texture.flipY = flipY;
  texture.repeat.copy(source.repeat); texture.offset.copy(source.offset); texture.center.copy(source.center); texture.rotation = source.rotation;
  textureCache.set(source, texture);
  texture.userData.evidence = {name: source.name, width, height, nonzero, varied};
  return texture;
}
function stats(root) {
  root.updateMatrixWorld(true);
  let meshes = 0, primitives = 0, vertices = 0, triangles = 0, joints = 0;
  root.traverse(n => {
    if (n.isBone) joints++;
    if (n.isMesh) {meshes++; primitives += Array.isArray(n.material) ? n.geometry.groups.length : 1; vertices += n.geometry.attributes.position.count; triangles += (n.geometry.index?.count ?? n.geometry.attributes.position.count) / 3;}
  });
  const box = new T.Box3().setFromObject(root, true);
  return {meshes, primitives, vertices, triangles, joints, min: box.min.toArray(), max: box.max.toArray()};
}
function checkRig(root) {
  let checked = 0;
  root.traverse(n => {
    if (!n.isSkinnedMesh) return;
    const bone = n.skeleton.bones.find(b => b.children.length) || n.skeleton.bones[0];
    const before = [];
    for (let i = 0; i < n.geometry.attributes.position.count; i++) before.push(n.getVertexPosition(i, new T.Vector3()).toArray());
    const old = bone.rotation.x; bone.rotation.x += 0.2; root.updateMatrixWorld(true); n.skeleton.update();
    const moved = before.some((p, i) => n.getVertexPosition(i, new T.Vector3()).distanceTo(new T.Vector3(...p)) > 1e-6);
    bone.rotation.x = old; root.updateMatrixWorld(true); n.skeleton.update();
    if (!moved) throw Error('Skeleton does not deform mesh'); checked++;
  });
  return checked;
}
try {
  if (T.REVISION !== THREE.REVISION) throw Error('Three.js revision mismatch');
  const context = await SaltlineAssets.initialize(renderer);
  const entries = SaltlineAssets.entries();
  window.exportIds = entries.map(e => e.id);
  window.exportAsset = async id => {
    const entry = entries.find(e => e.id === id);
    randomState = 7; usedTextures = new Set();
    const object = await entry.build(context); object.name = id; object.updateMatrixWorld(true);
    const original = stats(object), limitations = new Set();
    const articulationNodes = entry.articulationNodes || [];
    if (articulationNodes.length) limitations.add('Articulation nodes preserved; procedural motion and blinking lamps are not animation clips.');
    for (const name of articulationNodes) if (!object.getObjectByName(name)) throw Error(id + ': missing articulation node ' + name);
    const materials = new Map();
    object.traverse(n => {
      if (!n.isMesh) return;
      n.geometry = n.geometry.clone();
      if (n.geometry.hasAttribute('blink')) {
        n.geometry.deleteAttribute('blink');
        limitations.add('Blink shader timing attribute omitted; lamps use static unlit colors.');
      }
      const colors = n.geometry.attributes.color;
      if (colors) for (let i = 0; i < colors.array.length; i++) {
        if (colors.array[i] < 0 || colors.array[i] > 1) limitations.add('Vertex colors clamped to glTF 0..1 range.');
        colors.array[i] = Math.max(0, Math.min(1, colors.array[i]));
      }
      const adapt = source => {
        if (materials.has(source)) return materials.get(source);
        if (source.isShaderMaterial) throw Error('Unsupported ShaderMaterial: ' + source.name);
        const m = source.clone();
        m.userData = {}; m.onBeforeCompile = () => {}; m.customProgramCacheKey = () => ''; m.defines = {};
        if (source.userData.sl) limitations.add('Macro variation, wetness and shader normal adjustments use standard PBR approximations.');
        if (source.defines?.SL_TRIPLANAR !== undefined) limitations.add('Triplanar projection uses existing mesh UVs.');
        for (const key of ['map', 'normalMap', 'roughnessMap', 'metalnessMap', 'aoMap', 'emissiveMap', 'alphaMap']) m[key] = portableTexture(source[key], key);
        const tint = source.defines?.SL_TINT;
        if (m.map && (tint === 1 || tint === 2)) {
          const canvas = document.createElement('canvas'); canvas.width = m.map.image.width; canvas.height = m.map.image.height;
          const ctx = canvas.getContext('2d'); ctx.drawImage(m.map.image, 0, 0);
          const data = ctx.getImageData(0, 0, canvas.width, canvas.height);
          const c = new THREE.Color();
          for (let i = 0; i < data.data.length; i += 4) {
            c.setRGB(data.data[i]/255, data.data[i+1]/255, data.data[i+2]/255, THREE.SRGBColorSpace);
            const a = data.data[i+3]/255;
            if (tint === 1) c.add(source.color.clone().multiplyScalar(a));
            else { const l = Math.max(.55, Math.min(1.35, (c.r+c.g+c.b)/3/.19)); c.lerp(source.color.clone().multiplyScalar(l), a*.96); }
            c.convertLinearToSRGB();
            data.data[i] = Math.round(c.r*255); data.data[i+1] = Math.round(c.g*255); data.data[i+2] = Math.round(c.b*255); data.data[i+3] = 255;
          }
          ctx.putImageData(data, 0, 0);
          m.map = m.map.clone(); m.map.image = canvas; m.color.set(0xffffff);
          limitations.add('Source tint modes 1/2 baked into base color; vertex colors retained as glTF multipliers.');
        }
        for (const key of ['roughness', 'metalness', 'opacity']) if (m[key] !== undefined) {
          if (m[key] < 0 || m[key] > 1) limitations.add(key + ' clamped to glTF 0..1 range.');
          m[key] = Math.max(0, Math.min(1, m[key]));
        }
        if (Math.max(m.color.r, m.color.g, m.color.b) > 1) {
          limitations.add('HDR unlit color normalized to glTF base-color range.');
          m.color.multiplyScalar(1 / Math.max(m.color.r, m.color.g, m.color.b));
        }
        materials.set(source, m); return m;
      };
      n.material = Array.isArray(n.material) ? n.material.map(adapt) : adapt(n.material);
      n.userData = {};
    });
    const buffer = await new GLTFExporter().parseAsync(object, {binary: true, onlyVisible: false});
    const loaded = await new GLTFLoader().parseAsync(buffer, '');
    const roundtrip = stats(loaded.scene);
    for (const name of articulationNodes) if (!loaded.scene.getObjectByName(name)) throw Error(id + ': lost articulation node ' + name);
    for (const key of ['primitives', 'triangles', 'joints']) if (original[key] !== roundtrip[key]) throw Error(`${id}: ${key} changed`);
    for (const key of ['min', 'max']) original[key].forEach((v, i) => {if (Math.abs(v - roundtrip[key][i]) > Math.max(1e-4, Math.abs(v) * 1e-5)) throw Error(`${id}: bounds changed`);});
    const rigChecks = checkRig(loaded.scene);
    const scene = new THREE.Scene(); scene.background = new THREE.Color('#f5f1e8'); scene.add(loaded.scene);
    scene.add(new THREE.HemisphereLight(0xffffff, 0x797064, 3));
    const light = new THREE.DirectionalLight(0xffffff, 4); light.position.set(4, 7, 5); scene.add(light);
    const center = new THREE.Vector3().addVectors(new THREE.Vector3(...roundtrip.min), new THREE.Vector3(...roundtrip.max)).multiplyScalar(.5);
    const radius = new THREE.Vector3(...roundtrip.max).distanceTo(new THREE.Vector3(...roundtrip.min)) / 2;
    const camera = new THREE.PerspectiveCamera(32, 1, Math.max(radius / 1000, .0001), radius * 30);
    camera.position.copy(center).add(new THREE.Vector3(1, .6, 1).normalize().multiplyScalar(radius / Math.sin(16 * Math.PI / 180) * 1.15)); camera.lookAt(center);
    renderer.setRenderTarget(null); renderer.render(scene, camera);
    const png = renderer.domElement.toDataURL().split(',')[1];
    const previewCanvas = document.createElement('canvas'); previewCanvas.width = previewCanvas.height = 512;
    const previewContext = previewCanvas.getContext('2d'); previewContext.drawImage(renderer.domElement, 0, 0);
    const previewPixels = previewContext.getImageData(0, 0, 512, 512).data;
    let coverage = 0;
    for (let i = 0; i < previewPixels.length; i += 4) if (Math.abs(previewPixels[i] - previewPixels[0]) + Math.abs(previewPixels[i+1] - previewPixels[1]) + Math.abs(previewPixels[i+2] - previewPixels[2]) > 30) coverage++;
    coverage /= 512 * 512;
    if (coverage < .005) throw Error(id + ': blank reload preview');
    const bytes = new Uint8Array(buffer); let binary = '';
    for (let i = 0; i < bytes.length; i += 32768) binary += String.fromCharCode(...bytes.subarray(i, i + 32768));
    const result = {base64: btoa(binary), preview: png, coverage, articulationNodes, source: entry.source, original, roundtrip, rigChecks, materials: materials.size, limitations: [...limitations], textures: [...usedTextures].map(t => textureCache.get(t).userData.evidence)};
    loaded.scene.traverse(n => {if (n.isMesh) n.geometry.dispose();});
    return result;
  };
  window.exportReady = true;
} catch (e) {window.exportError = e.stack;}
