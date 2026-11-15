// Port of references/src/world/ground/{asphalt,water}.js; samples are linear data.
import bevy_pbr::render::{pbr_fragment::pbr_input_from_standard_material, mesh_view_bindings::view};
@if(PREPASS_PIPELINE)
import bevy_pbr::{prepass::io::{VertexOutput, FragmentOutput}, deferred::functions::deferred_output};
@else
import bevy_pbr::render::{forward_io::{VertexOutput, FragmentOutput}, pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing}};
struct WetUniform { settings:vec4<f32>, reflection_matrix:mat4x4<f32>, reflection:vec4<f32> };
@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> wet:WetUniform;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var mask_tex:texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var mask_sampler:sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(103) var marks_tex:texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(104) var marks_sampler:sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(105) var detail_tex:texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(106) var detail_sampler:sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(107) var normal_tex:texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(108) var normal_sampler:sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(109) var reflection_tex:texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(110) var reflection_sampler:sampler;
struct Surface { color:vec3<f32>, rough:f32, slope:vec2<f32>, pool:f32, soak:f32, film:f32 };
fn hash(p:vec2<f32>)->f32 {var q=fract(vec3(p.x,p.y,p.x)*vec3(0.1031,0.1030,0.0973));q+=dot(q,q.yzx+33.33);return fract((q.x+q.y)*q.z);}
fn hash3(p:vec2<f32>)->vec3<f32> {var q=fract(vec3(p.x,p.y,p.x)*vec3(0.1031,0.1030,0.0973));q+=dot(q,q.yzx+33.33);return fract((q.xxy+q.yzz)*q.zyx);}
fn noise(p:vec2<f32>)->f32 {let i=floor(p);let f=fract(p);let u=f*f*(3.0-2.0*f);return mix(mix(hash(i),hash(i+vec2(1.0,0.0)),u.x),mix(hash(i+vec2(0.0,1.0)),hash(i+vec2(1.0)),u.x),u.y);}
fn bicubic(tex:texture_2d<f32>,samp:sampler,uv:vec2<f32>,res:f32)->vec4<f32> {
 let p=uv*res-0.5;let i=floor(p);let f=p-i;let f2=f*f;let f3=f2*f;
 let w0=(1.0-3.0*f+3.0*f2-f3)/6.0;let w1=(4.0-6.0*f2+3.0*f3)/6.0;
 let w2=(1.0+3.0*f+3.0*f2-3.0*f3)/6.0;let w3=f3/6.0;
 let g0=w0+w1;let g1=w2+w3;let h0=(i+w1/g0-0.5)/res;let h1=(i+w3/g1+1.5)/res;
 return (textureSampleLevel(tex,samp,vec2(h0.x,h0.y),0.0)*g0.x+textureSampleLevel(tex,samp,vec2(h1.x,h0.y),0.0)*g1.x)*g0.y
 +(textureSampleLevel(tex,samp,vec2(h0.x,h1.y),0.0)*g0.x+textureSampleLevel(tex,samp,vec2(h1.x,h1.y),0.0)*g1.x)*g1.y;
}
fn ripple(p:vec2<f32>,t:f32,cell:f32,seed:f32)->vec2<f32> {
 let q=p/cell+seed*17.3;let id=floor(q);let f=fract(q);let h0=hash3(id+seed);
 let tt=t*(0.5+0.55*h0.x)+h0.y*9.0;let cyc=floor(tt);let life=fract(tt);
 let h=hash3(id+seed+cyc*7.31);let hc=hash3(id.yx+cyc*3.71+seed*5.0);
 let on=step(h.z,0.16+0.5*wet.settings.z);let rmax=mix(0.1,0.36,h.x*h.x);
 let c=mix(vec2(rmax+0.05),vec2(0.95-rmax),hc.xy);let dv=(f-c)*cell;let d=length(dv);
 let r=life*rmax*cell;let w=0.005+0.02*life;let x=(d-r)/w;let ring=x*exp(-x*x)*2.3;
 let amp=(1.0-life)*(1.0-life)*on*mix(0.3,1.0,h.y)*smoothstep(0.0,0.06,life);
 return dv/max(d,1e-4)*ring*amp;
}
fn apron(world:vec3<f32>)->Surface {
let settings=wet.settings;

  var gxz: vec2<f32> = world.xz;
  var gDist: f32 = length(view.world_position - world);
  // ---- unique large-scale masks (puddle height, dampness, oil, wear) ----
  var muv: vec2<f32> = (gxz - vec3(-44.0,-40.0,0.0125).xy) * vec3(-44.0,-40.0,0.0125).z;
  var mIn: f32 = smoothstep(0.0, 0.05, min(min(muv.x, 1.0 - muv.x), min(muv.y, 1.0 - muv.y)));
  var mk: vec4<f32> = mix(vec4(0.66, 0.5, 0.0, 0.5), bicubic(mask_tex, mask_sampler, muv, 1024.0), mIn);
  // ---- tiling detail ----
  var d1: vec4<f32> = textureSample(detail_tex, detail_sampler, gxz * 0.21);
  var d2: vec4<f32> = textureSample(detail_tex, detail_sampler, gxz * 1.37 + 0.37);
  var n1: vec4<f32> = textureSample(normal_tex, normal_sampler, gxz * 1.37 + 0.37);
  var n0: vec4<f32> = textureSample(normal_tex, normal_sampler, gxz * 0.21);
  var grain: f32 = d2.r;
  // ---- slabs: 6 m expansion joints with tar sealing ----
  var sj: vec2<f32> = gxz / 6.0;
  var sf: vec2<f32> = (0.5 - abs(fract(sj) - 0.5)) * 6.0;
  var jd: f32 = min(sf.x, sf.y) + (d1.a - 0.5) * 0.035;
  var jaa: f32 = fwidth(jd) + 1e-5;
  var tarW: f32 = 0.022 + 0.03 * d1.g;
  var tar: f32 = (1.0 - smoothstep(tarW - jaa, tarW + jaa, jd)) * clamp(tarW / jaa, 0.0, 1.0);
  var groove: f32 = (1.0 - smoothstep(0.004, 0.012 + jaa, jd)) * clamp(0.02 / jaa, 0.0, 1.0);
  var sid: vec2<f32> = floor(sj);
  var sh: f32 = fract(sin(dot(sid, vec2(127.1, 311.7))) * 43758.5453);
  var sh2: f32 = fract(sh * 7.31 + 0.13);
  // ---- cracks, some sealed with tar ----
  var crArea: f32 = smoothstep(0.6, 0.76, mk.a * 0.7 + sh * 0.45);
  var cfw: f32 = fwidth(d1.b) * 1.5 + 0.004;
  var crack: f32 = smoothstep(0.955 - cfw, 0.975, d1.b) * crArea;
  var seal: f32 = smoothstep(0.915 - cfw, 0.94, d1.b) * crArea * step(0.72, sh2);
  // ---- base albedo: worn asphalt in the yard, concrete slabs on the quay ----
  var conc: f32 = smoothstep(15.6, 16.6, gxz.x + (d1.g - 0.5) * 0.6);
  var alb: vec3<f32> = mix(vec3(0.125, 0.125, 0.13), vec3(0.22, 0.215, 0.2), conc);
  alb *= (0.8 + 0.4 * sh2) * (0.8 + 0.4 * d1.g) * (0.62 + 0.76 * grain);
  alb += vec3(0.11, 0.105, 0.095) * smoothstep(0.58, 0.78, d2.a) * (1.0 - conc * 0.6);    // pale aggregate
  var polish: f32 = smoothstep(0.38, 0.72, mk.a);                                            // traffic-polished lanes
  alb *= mix(1.0, 0.78, polish);
  alb = mix(alb, vec3(0.018, 0.018, 0.02), max(max(tar, seal * 0.4) * 0.94, crack * 0.9));
  alb *= 1.0 - 0.62 * mk.b;                                                               // oil
  // ---- painted markings (worn, chipped) + tyre rubber ----
  var kuv: vec2<f32> = (gxz - vec4(-40.0,-30.0,1.0/70.0,1.0/60.0).xy) * vec4(-40.0,-30.0,1.0/70.0,1.0/60.0).zw;
  var kIn: f32 = step(0.0, kuv.x) * step(kuv.x, 1.0) * step(0.0, kuv.y) * step(kuv.y, 1.0);
  var mp: vec3<f32> = textureSample(marks_tex, marks_sampler, kuv).rgb;
  var kpx: f32 = max(fwidth(kuv.x), fwidth(kuv.y)) * 2048.0;
  if (kpx < 1.0) { mp = mix(bicubic(marks_tex, marks_sampler, kuv, 2048.0).rgb, mp, smoothstep(0.5, 1.0, kpx)); }   // magnified: no texel blocks
  mp *= kIn;
  var kw: f32 = clamp(kpx * 0.5, 0.16, 0.5);
  // chips: world-space procedural noise down to sub-centimetre (never the texels of a map)
  var chipN: f32 = 0.5;
  if (max(mp.r, mp.g) > 0.02) {
    var cf2: f32 = 1.0 - smoothstep(6.0, 18.0, gDist);
var cf3: f32 = 1.0 - smoothstep(2.5, 7.0, gDist);
    chipN = noise(gxz * 17.0) * 0.45 + mix(0.5, noise(gxz * 57.0 + 11.0), cf2) * 0.33 + mix(0.5, noise(gxz * 181.0 + 5.0), cf3) * 0.22;
  }
  var chip: f32 = chipN * 0.62 + d1.a * 0.38;
  var wear: f32 = clamp(mk.a * 1.25 - 0.22 + mp.b * 0.6, 0.0, 1.0);
  var wt: f32 = mix(0.24, 0.55, wear);
  var keep: f32 = smoothstep(wt - 0.045, wt + 0.045, chip) * (1.0 - max(tar, crack));
  var eN: f32 = (chipN - 0.5) * 0.55 * (1.0 - kw * 2.0);
  var pY: f32 = smoothstep(0.5 - kw, 0.5 + kw, mp.r + eN) * keep;
  var pW: f32 = smoothstep(0.5 - kw, 0.5 + kw, mp.g + eN) * keep;
  alb *= 1.0 - 0.6 * mp.b * (1.0 - 0.5 * conc);
  var fade: f32 = 0.72 + 0.28 * d1.g;
  alb = mix(alb, vec3(0.52, 0.34, 0.035) * fade, pY * 0.94);
  alb = mix(alb, vec3(0.47, 0.47, 0.44) * fade, pW * 0.94);
  var paint: f32 = max(pY, pW);
  // ---- water: one continuous gradient  standing water -> soaked band -> damp -> drier patches ----
  // organic shoreline: world-space noise (40 / 15 / 5 cm) on the water-level comparison, independent of the mask res
  var sf2: f32 = 1.0 - smoothstep(25.0, 60.0, gDist);
var sf3: f32 = 1.0 - smoothstep(8.0, 22.0, gDist);
  var sn1: f32 = noise(gxz * 2.5 + 3.0);
var sn2: f32 = mix(0.5, noise(gxz * 6.7 + 17.0), sf2);
var sn3: f32 = mix(0.5, noise(gxz * 20.0 + 9.0), sf3);
  var sn: f32 = sn1 * 0.5 + sn2 * 0.3 + sn3 * 0.2;
  var depth: f32 = 0.5 - mk.r + (sn - 0.5) * 0.11 + (d1.g - 0.5) * 0.03 + (grain - 0.5) * 0.012 + groove * 0.04 + crack * 0.02 - paint * 0.01;
  var puddle: f32 = smoothstep(0.0, 0.035, depth) * settings.w;                 // mirror weight (~5 cm ramp)
  var deep: f32 = smoothstep(0.0, 0.22, depth);                           // 0 at the rim (bed shows) .. 1 a hand deep
  var bandW: f32 = 0.05 + 0.15 * sn1 * sn1 + 0.05 * d1.g;                  // soaked band, roughly 10-30 cm, varies along the shore
  var soak: f32 = smoothstep(-bandW, 0.0, depth) * settings.w;
  var fringe: f32 = smoothstep(-0.65, -0.02, depth);                      // damp halo, about a metre
  var dryP: f32 = 1.0 - smoothstep(0.2, 0.42, mk.g + fringe * 0.55 + (d1.g - 0.5) * 0.25 + (sn - 0.5) * 0.12);
  var damp: f32 = (1.0 - dryP) * settings.w;
  var wet: f32 = clamp(damp * 0.7 + soak * 0.3, 0.0, 1.0);
  var film: f32 = clamp(soak * (1.0 - smoothstep(0.35, 0.95, grain + (1.0 - soak) * 0.6)) + groove * 0.5, 0.0, 1.0) * (1.0 - paint * 0.4);
  alb *= mix(0.95, mix(0.7, 0.54, 1.0 - paint), damp);
  alb *= mix(1.0, 0.5, soak);
  // standing water: shallow rim shows the wet bed, deeper water goes murky and dark (never a black cut-out)
  alb = mix(alb, alb * mix(0.85, 0.4, deep) + vec3(0.004, 0.006, 0.0065) * deep, puddle);
  
  // ---- roughness: dry ~0.72 -> damp 0.36..0.56 -> soaked 0.1..0.2 -> water 0.055, all smooth ramps ----
  var rough: f32 = mix(0.74, 0.42, damp) + (grain - 0.5) * 0.24 + (d1.g - 0.5) * 0.16 + conc * 0.03 - paint * 0.05;
  rough -= polish * 0.07 + mk.b * 0.16 + max(tar, seal) * 0.1;
  rough = mix(rough, 0.2 + (grain - 0.5) * 0.14, soak * 0.9);
  rough = mix(rough, 0.13, film * 0.8);
  rough += smoothstep(10.0, 80.0, gDist) * 0.04;
  rough = mix(clamp(rough, 0.1, 1.0), 0.055, puddle);
  // ---- normal (world xz slope): aggregate grain + lumps break up the mirror outside puddles ----
  var nxz: vec2<f32> = (n1.rg - 0.5) * mix(2.1, 0.7, max(film, soak * 0.6)) + (n0.ba - 0.5) * 0.3;
  nxz *= 1.0 - puddle;
  var wsum: f32 = puddle + film * 0.2;
  var wv: vec2<f32> = textureSample(normal_tex, normal_sampler, gxz * 0.13 + settings.x * vec2(0.031, 0.017)).ba - 0.5;
  var wv2: vec2<f32> = textureSample(normal_tex, normal_sampler, gxz.yx * 0.31 - settings.x * vec2(0.043, 0.027)).ba - 0.5;
  if (wsum > 0.004) {
    var rf: f32 = 1.0 - smoothstep(10.0, 34.0, gDist);
    var rip: vec2<f32> = vec2(0.0);
    if (rf > 0.0) {
      var gp2: vec2<f32> = vec2(0.8 * gxz.x - 0.6 * gxz.y, 0.6 * gxz.x + 0.8 * gxz.y);
      var r2: vec2<f32> = ripple(gp2, settings.x * 0.87, 1.31, 2.0);
      rip = ripple(gxz, settings.x, 0.83, 1.0) + vec2(0.8 * r2.x + 0.6 * r2.y, -0.6 * r2.x + 0.8 * r2.y) * 0.9 + ripple(gxz.yx, settings.x * 1.13, 0.57, 3.0).yx * 0.6;
      rip *= rf * (0.3 + 0.7 * settings.z) * 0.3 * smoothstep(0.0, 0.08, depth);
    }
    nxz += (rip + (wv * 0.03 + wv2 * 0.018) * (0.4 + 0.6 * deep)) * wsum;
    rough += (1.0 - rf) * 0.03 * puddle * settings.z;
  }



 return Surface(alb, rough, nxz, puddle, soak, film);
}

fn wave_slope(p:vec2<f32>,t:f32,detail:bool)->vec2<f32> {
 var sum=vec2(0.0);
 var waves=array<vec4<f32>,7>(vec4(-0.8,0.6,1.31,0.02),vec4(-0.95,-0.31,0.83,0.014),vec4(-0.35,-0.94,0.57,0.01),vec4(-0.6,0.8,0.39,0.007),vec4(-1.0,0.05,0.27,0.005),vec4(0.3,0.95,0.47,0.006),vec4(-0.86,-0.51,0.21,0.0035));
 var count=7;
 if !detail {
 waves[0]=vec4(-0.92,0.39,9.5,0.06);waves[1]=vec4(-0.75,-0.66,5.7,0.04);waves[2]=vec4(-0.99,0.12,3.3,0.026);waves[3]=vec4(-0.5,0.87,2.0,0.016);count=4;
 }
 for(var i=0;i<count;i++) {let w=waves[i];let k=6.28318530718/w.z;let dir=normalize(w.xy);sum+=dir*w.w*k*cos(k*dot(dir,p)-sqrt(9.81*k)*t);}
 return sum;
}
fn ocean(world:vec3<f32>)->Surface {
 var slope=vec2(0.0);var rough=0.1;
 let amplitude=1.0-smoothstep(50.0,220.0,distance(world.xz,vec2(30.0,0.0)));

  var sxz: vec2<f32> = world.xz;
  var sd: f32 = length(view.world_position - world);
  var slopeA: vec2<f32> = vec2(0.0);
var slopeB: vec2<f32> = vec2(0.0);
slopeA = wave_slope(sxz, wet.settings.x, false);
slopeB = wave_slope(sxz, wet.settings.x, true);
  var nearB: f32 = 1.0 - smoothstep(25.0, 110.0, sd);
  var t1: vec2<f32> = textureSample(normal_tex, normal_sampler, sxz * 0.113 + wet.settings.x * vec2(-0.021, 0.009)).ba - 0.5;
  var t2: vec2<f32> = textureSample(normal_tex, normal_sampler, sxz * 0.031 + wet.settings.x * vec2(-0.007, -0.004)).ba - 0.5;
  var t3: vec2<f32> = textureSample(normal_tex, normal_sampler, sxz.yx * 0.39 + wet.settings.x * vec2(0.031, -0.027)).ba - 0.5;
  slope = slopeA * amplitude + slopeB * nearB + t1 * 0.2 + t2 * 0.24 + t3 * 0.1 * nearB;
  // foam / scum along the quay wall (x = 30) and around the moored hull x[33,49] z[-60,-9]
  var dq: f32 = sxz.x - 30.0;
  var hq: vec2<f32> = abs(sxz - vec2(41.0, -34.5)) - vec2(4.0, 21.5);
  var dh: f32 = length(max(hq, vec2(0.0))) + min(max(hq.x, hq.y), 0.0) - 4.0;
  var de: f32 = min(dq, dh);
  var foam_noise: vec4<f32> = textureSample(detail_tex, detail_sampler, sxz * 0.41 + wet.settings.x * vec2(-0.012, 0.007));
  var fn2: vec4<f32> = textureSample(detail_tex, detail_sampler, sxz * 0.093 + wet.settings.x * vec2(0.004, 0.003));
  var band: f32 = 1.0 - smoothstep(0.0, 0.2 + fn2.g * fn2.g * 2.2, de);
  var foam: f32 = clamp(band * smoothstep(0.5, 0.68, foam_noise.a * 0.55 + foam_noise.r * 0.25 + fn2.g * 0.3 + band * 0.2), 0.0, 1.0) * step(-0.3, de);
  var scum: f32 = (1.0 - smoothstep(0.0, 5.0, de)) * smoothstep(0.4, 0.7, fn2.g);
  var alb: vec3<f32> = vec3(0.0045, 0.013, 0.017);
  alb = mix(alb, vec3(0.03, 0.04, 0.034), scum * 0.6);
  alb = mix(alb, vec3(0.3, 0.32, 0.3), foam * 0.7);

  // wind structure: broad cat's-paw patches (ruffled, darker, duller) between calmer slicks, plus long swell shading
  var fn3: vec4<f32> = textureSample(detail_tex, detail_sampler, sxz * 0.0123 + wet.settings.x * vec2(0.0016, 0.0011));
  var gust: f32 = smoothstep(0.38, 0.62, fn2.g * 0.45 + fn3.g * 0.75 - 0.1);
  var swell: f32 = sin(dot(sxz, vec2(-0.118, 0.05)) - wet.settings.x * 0.55 + fn3.a * 5.0) * 0.5 + 0.5;
  slope = slope * mix(0.4, 1.35, gust) + vec2(-0.92, 0.39) * (swell - 0.5) * 0.07;

  alb *= mix(1.0,0.6,gust);
  rough = mix(mix(0.1, 0.32, smoothstep(12.0, 380.0, sd)) * mix(0.62, 1.5, gust), 0.65, max(foam, scum * 0.3));
  slope *= 1.0 - foam * 0.6;


return Surface(alb,rough,slope,0.0,0.0,0.0);
}
// A filtered planar sample avoids a second sharp mirror on damp asphalt.
fn planar(uv:vec2<f32>,rough:f32,pool:f32)->vec3<f32> {
 let texel=1.0/vec2<f32>(textureDimensions(reflection_tex));
 let radius=mix(rough*rough*40.0,1.2,pool);
 let d=texel*radius;
 var color=textureSampleLevel(reflection_tex,reflection_sampler,clamp(uv,vec2(0.002),vec2(0.998)),0.0).rgb*0.4;
 for(var i=0;i<4;i++) {
 let angle=f32(i)*1.5707963+0.785398;
 let delta=vec2(cos(angle),sin(angle))*d;
 color+=textureSampleLevel(reflection_tex,reflection_sampler,clamp(uv+delta,vec2(0.002),vec2(0.998)),0.0).rgb*0.15;
 }
 return min(color,vec3(48.0));
}
@fragment fn fragment(input:VertexOutput,@builtin(front_facing) is_front:bool)->FragmentOutput {
 var pbr=pbr_input_from_standard_material(input,is_front);
 var surface:Surface;
 if wet.settings.y > 0.5 {surface=ocean(input.world_position.xyz);} else {surface=apron(input.world_position.xyz);}
 pbr.material.base_color=vec4(surface.color,1.0);
 pbr.material.perceptual_roughness=surface.rough;
 pbr.material.metallic=0.0;
 pbr.N=normalize(vec3(-surface.slope.x,1.0,-surface.slope.y));
 @if(PREPASS_PIPELINE) {return deferred_output(input,pbr);}
 @if(!PREPASS_PIPELINE) {
 var out:FragmentOutput;
 out.color=apply_pbr_lighting(pbr);
 if wet.reflection.x > 0.5 && wet.settings.y < 0.5 {
 let rc=wet.reflection_matrix*vec4(input.world_position.x,0.0,input.world_position.z,1.0);
 let uv=vec2(rc.x/rc.w*0.5+0.5,0.5-rc.y/rc.w*0.5);
 let v=normalize(view.world_position-input.world_position.xyz);
 let fresnel=0.075+0.925*pow(1.0-clamp(dot(v,pbr.N),0.0,1.0),5.0);
 let in_view=step(0.0,uv.x)*step(uv.x,1.0)*step(0.0,uv.y)*step(uv.y,1.0)*step(0.001,rc.w);
 let weight=mix(1.0-smoothstep(0.55,0.9,surface.rough),1.0,max(surface.pool,surface.soak))*in_view*fresnel;
 let reflected=planar(uv+surface.slope*vec2(0.015,0.04),surface.rough,surface.pool);
 out.color=vec4(mix(out.color.rgb,reflected,weight),1.0);
 }
 out.color=main_pass_post_lighting_processing(pbr,out.color);
 return out;
 }
}
