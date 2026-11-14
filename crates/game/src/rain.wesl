import bevy_pbr::render::{forward_io::{Vertex,decompress_vertex},view_transformations::position_world_to_clip};
struct RainOut {
    @builtin(position) position:vec4<f32>,
    @location(0) world:vec3<f32>,
    @location(1) uv:vec2<f32>,
};
struct RainUniform {motion:vec4<f32>,right:vec4<f32>};
@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> settings:RainUniform;
fn wrap(v:f32,n:f32)->f32 {return v-floor(v/n)*n;}
@vertex fn vertex(input:Vertex)->RainOut {
    let motion=settings.motion;
    let v=decompress_vertex(input,input.instance_index);
    var p=v.position;
    p.x=wrap(p.x-motion.x*1.8-motion.y+22.,44.)+motion.y-22.;
    p.z=wrap(p.z+motion.x*0.6-motion.z+22.,44.)+motion.z-22.;
    p.y=wrap(p.y-motion.x*16.,18.);
    p+=settings.right.xyz*((v.uv.x-0.5)*0.012)+vec3((v.uv.y-0.5)*0.04,(v.uv.y-0.5)*0.33,0.);
    var out:RainOut;
    out.world=p;out.uv=v.uv;out.position=position_world_to_clip(p);
    return out;
}
@fragment fn fragment(input:RainOut)->@location(0) vec4<f32> {
    let motion=settings.motion;
    let p=input.world;
    let roof=p.x > -16. && p.x < 12. && p.z > -28. && p.z < -11. && p.y < 7.;
    if roof || p.y<0.05 {discard;}
    let fade=1.-smoothstep(14.,22.,length(p.xz-motion.yz));
    let tip=sin(input.uv.y*3.14159);
    return vec4(0.48,0.59,0.68,0.20*fade*tip);
}
