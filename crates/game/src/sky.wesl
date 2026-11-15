// Port of references/src/render/sky.js. All colors and texture samples are linear.
import bevy_pbr::render::{forward_io::{Vertex,decompress_vertex},mesh_view_bindings::view};
struct SkySettings { sun: vec4<f32>, params: vec4<f32> };
@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> settings: SkySettings;
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var noise_texture: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var noise_sampler: sampler;
struct SkyOut {
    @builtin(position) position: vec4<f32>,
    @location(0) direction: vec3<f32>,
};
@vertex fn vertex(input: Vertex) -> SkyOut {
    let vertex = decompress_vertex(input, input.instance_index);
    // w=0 removes view translation. Bevy uses reversed Z: 0 is infinitely far.
    let clip = view.clip_from_world * vec4(vertex.position, 0.0);
    var out: SkyOut;
    out.position = vec4(clip.xy, 0.0, clip.w);
    out.direction = vertex.position;
    return out;
}
fn n2(p: vec2<f32>) -> f32 {
    let i = floor(p);
    var f = fract(p);
    f = f*f*(vec2(3.0)-2.0*f);
    return textureSampleLevel(noise_texture, noise_sampler, (i+f+vec2(0.5))/256.0, 0.0).r;
}
fn fbm5(point: vec2<f32>) -> f32 {
    var p=point; var a=0.5; var s=0.0;
    for(var i=0;i<5;i++){s+=a*n2(p);p=mat2x2(vec2(1.6,1.2),vec2(-1.2,1.6))*p+vec2(17.3);a*=0.5;}
    return s/0.96875;
}
fn fbm3(point: vec2<f32>) -> f32 {
    var p=point; var a=0.5; var s=0.0;
    for(var i=0;i<3;i++){s+=a*n2(p);p=mat2x2(vec2(1.6,1.2),vec2(-1.2,1.6))*p+vec2(17.3);a*=0.5;}
    return s/0.875;
}
fn skyToSun(d:vec3<f32>) -> f32 {
    return clamp(dot(normalize(d.xz+vec2(1e-5)),normalize(settings.sun.xz))*0.5+0.5,0.0,1.0);
}
fn horizonColor(d:vec3<f32>) -> vec3<f32> {
    let ts=skyToSun(d);
    let away=vec3(0.16,0.22,0.3);
    var c=mix(away,away*1.25+vec3(0.05,0.03,0.02),smoothstep(0.2,0.75,ts));
    c=mix(c,vec3(0.95,0.55,0.28)*vec3(1.9,1.55,1.2),pow(ts,6.0));
    return mix(c,vec3(1.0,0.60,0.24)*3.4,pow(ts,34.0));
}
@fragment fn fragment(input: SkyOut) -> @location(0) vec4<f32> {

  var d = normalize(input.direction);
  var h = max(d.y, 0.0);
  var mu = dot(d, settings.sun.xyz);
  var s = max(mu, 0.0);
  var ts = skyToSun(d);

  // ---- clear sky behind / below the deck ----
  var hc = horizonColor(d);
  var midC = mix(vec3(0.15, 0.24, 0.36), vec3(0.72, 0.50, 0.34), pow(ts, 3.0));   // pale band above the fire
  var clearC = mix(hc, midC, smoothstep(0.0, 0.17, h));
  clearC = mix(clearC, vec3(0.085, 0.145, 0.24), smoothstep(0.10, 0.5, h));
  // aureole (mie glow round the sun)
  clearC += vec3(1.0, 0.72, 0.42) * vec3(1.0, 0.86, 0.68) * (pow(s, 8.0) * 0.14 + pow(s, 64.0) * 0.55 + pow(s, 600.0) * 2.2 + pow(s, 6000.0) * 14.0);

  // ---- cloud deck ----
  var cuv = d.xz / (h + 0.10) * vec2(0.62, 0.95);          // rolls of cloud run roughly east-west
  var wind = vec2(-0.012, 0.004) * settings.params.x;
  var q = cuv + wind;
  var warp = fbm3(q * 0.45 + vec2(7.1));
  var qq = q + vec2((warp - 0.5) * 1.3);
  var dens = fbm5(qq);
  // rippled underside (ridged detail), fading with distance so the horizon does not alias
  var ridg = 1.0 - abs(2.0 * fbm3(qq * 3.1 + vec2(5.0)) - 1.0);
  dens = dens * 0.86 + (ridg - 0.6) * 0.16 * smoothstep(0.03, 0.25, h);
  var big = n2(q * 0.19 + vec2(3.7));
  var body = dens * 0.74 + big * 0.36;
  var cov = smoothstep(0.31, 0.50, body);
  // the deck ends above the eastern horizon: clear, burning gap toward the sun; almost down to the sea elsewhere
  var edge = mix(0.030, 0.285, pow(ts, 1.7));
  var rag = (n2(q * 0.9 + vec2(11.0)) - 0.5) * 0.16 + (dens - 0.5) * 0.10;
  var deck = smoothstep(edge - 0.035, edge + 0.11, h + rag * (0.35 + ts));
  cov *= deck;
  // the sun always burns through a ragged hole in the gap, never sits behind solid cloud
  var hole = smoothstep(0.972, 0.9985, mu + (dens - 0.5) * 0.012);
  cov *= 1.0 - 0.93 * hole;
  // thin scud drifting in the gap (veils the sun a little, catches fire)
  var scud = smoothstep(0.55, 0.85, fbm3(q * vec2(0.55, 1.9) + vec2(31.0))) * (1.0 - deck) * smoothstep(0.02, 0.10, h) * 0.55 * (1.0 - 0.8 * hole);

  // lighting: density gradient toward the sun = self shadowing of the underside relief
  var sd = normalize(settings.sun.xyz.xz);
  var densS = fbm5(qq + sd * 0.22);
  var relief = clamp((dens - densS) * 4.5 + 0.5, 0.0, 1.0);
  var thick = smoothstep(0.40, 0.72, body);
  var cool = mix(vec3(0.155, 0.195, 0.255), vec3(0.030, 0.042, 0.064), thick);       // slate, darker where the deck is thick
  cool *= 0.55 + 0.9 * relief * relief;
  cool += vec3(0.07, 0.10, 0.15) * pow(1.0 - thick, 3.0);                             // thin spots let cold light through
  // warm light raking the underside from the gap: strongest near the sun and near the deck edge
  var nearSun = pow(ts, 5.0) * exp(-max(h - edge, 0.0) * 4.5) * 0.9 + pow(s, 7.0) * 0.9;
  var warm = (vec3(0.95, 0.55, 0.28) * vec3(1.30, 0.66, 0.40)) * nearSun * (0.05 + 2.2 * relief * relief * relief) * 1.15;
  // burning rim where cloud thins at the edge of the gap
  var rim = cov * (1.0 - cov) * 4.0;
  warm += vec3(1.0, 0.72, 0.42) * rim * pow(ts, 4.0) * (0.5 + 1.6 * pow(s, 6.0)) * (1.0 - 0.85 * hole);
  var cloudC = cool + warm;
  // aerial perspective: far cloud melts into the horizon colour
  cloudC = mix(cloudC, hc * 0.55 + cloudC * 0.45, exp(-h * 11.0));

  var col = mix(clearC, cloudC, cov * 0.985);
  var scudC = mix(vec3(0.10, 0.11, 0.14), vec3(0.95, 0.55, 0.28) * 1.5, pow(ts, 3.0)) + vec3(1.0, 0.72, 0.42) * pow(s, 24.0) * 2.0;
  col = mix(col, scudC, scud);

  // sun disk (not in the environment capture: the directional light is the sun there)
  var veil = (1.0 - cov * 0.92) * (1.0 - scud * 0.7);
  var disk = smoothstep(0.999905, 0.999955, mu);
  col += vec3(1.0, 0.80, 0.52) * disk * 110.0 * veil * (1.0 - settings.params.y);

  // below the horizon: sea haze / ground bounce (mostly hidden by geometry)
  if (d.y < 0.0) {
    var t = clamp(-d.y * 5.0, 0.0, 1.0);
    var low = mix(vec3(0.16, 0.22, 0.3) * 0.42, vec3(0.060, 0.064, 0.070), settings.params.y);
    low += vec3(0.95, 0.55, 0.28) * 0.55 * pow(ts, 6.0) * exp(d.y * 7.0);
    col = mix(hc + vec3(1.0, 0.72, 0.42) * pow(s, 48.0) * 1.5, low, smoothstep(0.0, 1.0, t));
  }
  // keep the PMREM free of fireflies
  col = mix(col, min(col, vec3(6.0)), settings.params.y);
  return vec4(max(col, vec3(0.0)), 1.0);
}
