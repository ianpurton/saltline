// Evaluate the port's shader math against float pixels from the reference GLSL.
export async function validateSky(samples, noise, sun) {
  const adapter=await navigator.gpu?.requestAdapter({forceFallbackAdapter:true});
  if(!adapter)throw Error('WebGPU software adapter unavailable for numerical sky validation');
  const device=await adapter.requestDevice();
  let source=await (await fetch('/crates/game/src/sky.wesl')).text();
  source=source.replace(/^import .*;$/m,'').replaceAll('constants::MATERIAL_BIND_GROUP','0')
    .replace(/@vertex fn vertex[\s\S]*?\nfn n2/,'fn n2')
    .replace('@fragment fn fragment(input: SkyOut) -> @location(0) vec4<f32>','fn sky_color(direction:vec3<f32>) -> vec4<f32>')
    .replace('normalize(input.direction)','normalize(direction)');
  source+=`\n@group(0) @binding(3) var<storage,read_write> result:array<vec4<f32>>;
    @compute @workgroup_size(1) fn test(){result[0]=sky_color(vec3(TEST_DIRECTION));}`;
  const texture=device.createTexture({size:[256,256],format:'rgba8unorm',usage:GPUTextureUsage.TEXTURE_BINDING|GPUTextureUsage.COPY_DST});
  device.queue.writeTexture({texture},noise,{bytesPerRow:1024},[256,256]);
  const sampler=device.createSampler({addressModeU:'repeat',addressModeV:'repeat',magFilter:'linear',minFilter:'linear'});
  const uniform=device.createBuffer({size:32,usage:GPUBufferUsage.UNIFORM|GPUBufferUsage.COPY_DST});
  const result=device.createBuffer({size:16,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_SRC});
  const staging=device.createBuffer({size:16,usage:GPUBufferUsage.MAP_READ|GPUBufferUsage.COPY_DST});
  let maxError=0;
  for(const sample of samples) {
    const theta=((sample.x+.5)/1024-.5)*Math.PI*2,phi=(1-(sample.y+.5)/512)*Math.PI;
    const direction=[Math.sin(phi)*Math.cos(theta),Math.cos(phi),Math.sin(phi)*Math.sin(theta)];
    const module=device.createShaderModule({code:source.replace('TEST_DIRECTION',direction.join(','))});
    const compilation=await module.getCompilationInfo();
    const errors=compilation.messages.filter(m=>m.type==='error');if(errors.length)throw Error(errors.map(e=>e.message).join('\n'));
    const pipeline=await device.createComputePipelineAsync({layout:'auto',compute:{module,entryPoint:'test'}});
    const bind=device.createBindGroup({layout:pipeline.getBindGroupLayout(0),entries:[
      {binding:0,resource:{buffer:uniform}},{binding:1,resource:texture.createView()},
      {binding:2,resource:sampler},{binding:3,resource:{buffer:result}}]});
    device.queue.writeBuffer(uniform,0,new Float32Array([...sun,0,sample.time,1,1,0]));
    const encoder=device.createCommandEncoder(),pass=encoder.beginComputePass();pass.setPipeline(pipeline);pass.setBindGroup(0,bind);pass.dispatchWorkgroups(1);pass.end();encoder.copyBufferToBuffer(result,0,staging,0,16);device.queue.submit([encoder.finish()]);
    await staging.mapAsync(GPUMapMode.READ);const values=Array.from(new Float32Array(staging.getMappedRange()));staging.unmap();
    for(let c=0;c<3;c++)maxError=Math.max(maxError,Math.abs(values[c]-sample.color[c]));
  }
  device.destroy();
  if(maxError>.006)throw Error(`Sky shader differs from reference: ${maxError}`);
  return {samples:samples.length,maxAbsoluteError:maxError};
}
