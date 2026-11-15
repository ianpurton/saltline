import bevy_pbr::render::{forward_io::{Vertex,decompress_vertex},view_transformations::position_world_to_clip};
struct RainOut {
    @builtin(position) position:vec4<f32>,
    @location(0) world:vec3<f32>,
    @location(1) uv:vec2<f32>,
};
struct RainUniform {motion:vec4<f32>,right:vec4<f32>};
@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> settings:RainUniform;
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var mask_tex:texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var mask_sampler:sampler;
fn hash2(p:vec2<f32>)->vec2<f32> {var q=fract(vec3(p.x,p.y,p.x)*vec3(0.1031,0.1030,0.0973));q+=dot(q,q.yzx+33.33);return fract((q.xx+q.yz)*q.zy);}
fn wrap(v:f32,n:f32)->f32 {return v-floor(v/n)*n;}
@vertex fn vertex(input:Vertex)->RainOut {
    let motion=settings.motion;
    let v=decompress_vertex(input,input.instance_index);
    var p=v.position;
    p.x=wrap(p.x-motion.x*1.8-motion.y+22.,44.)+motion.y-22.;
    p.z=wrap(p.z+motion.x*0.6-motion.z+22.,44.)+motion.z-22.;
    p.y=wrap(p.y-motion.x*16.,18.);
    if motion.w>0.5 {
        let seed=hash2(v.position.xz+v.position.y);
        let period=0.26+0.14*seed.x;
        let t=motion.x/period+seed.y;
        let cycle=floor(t);let life=fract(t);
        let rr=hash2(v.position.xz+cycle*7.31)-0.5;
        p=vec3(floor(motion.y*0.5)*2.0+rr.x*30.0,0.0,floor(motion.z*0.5)*2.0+rr.y*30.0);
        let uv=(p.xz-vec2(-44.0,-40.0))/80.0;
        let pool=step(textureSampleLevel(mask_tex,mask_sampler,uv,0.0).r,0.49);
        let k=fract(seed.x*5.3+cycle*0.17);
        let roof=p.x> -16.0 && p.x<12.0 && p.z> -28.0 && p.z< -11.0;
        let kill=(pool<0.5 && k>0.3) || k>0.2+0.8*settings.right.w || p.x>29.7 || roof;
        let size=mix(0.04,0.08,seed.y)*mix(0.65,1.0,pool);
        let width=size*(0.7+1.5*life);let height=size*(0.4+1.3*sin(life*3.14159));
        let delta=p.xz-motion.yz;
        let side=normalize(vec3(delta.y,0.0,-delta.x)+vec3(1e-5,0.0,0.0));
        p+=side*(width*(v.uv.x-0.5)*2.0)+vec3(0.0,height*v.uv.y+0.004,0.0);
        var splash:RainOut;
        splash.world=vec3(p.x,life,p.z);splash.uv=v.uv;
        splash.position=position_world_to_clip(p);
        if kill {splash.position=vec4(2.0,2.0,2.0,1.0);}
        return splash;
    }
    p+=settings.right.xyz*((v.uv.x-0.5)*0.012)+vec3((v.uv.y-0.5)*0.04,(v.uv.y-0.5)*0.33,0.);
    var out:RainOut;
    out.world=p;out.uv=v.uv;out.position=position_world_to_clip(p);
    return out;
}
@fragment fn fragment(input:RainOut)->@location(0) vec4<f32> {
    let motion=settings.motion;
    let p=input.world;
    if motion.w>0.5 {
        let x=(input.uv.x-0.5)*2.0;let y=input.uv.y;
        let dome=1.0-smoothstep(0.25,1.0,length(vec2(x*1.15,y*1.5)));
        let crown=dome*(0.35+0.65*smoothstep(0.25,0.75,length(vec2(x,y*1.4))));
        let core=(1.0-smoothstep(0.0,0.5,length(vec2(x*1.6,(y-0.25)*1.6))))*(1.0-p.y);
        let fade=1.0-smoothstep(11.0,15.0,length(p.xz-motion.yz));
        let alpha=clamp(crown*0.5+core*0.8,0.0,1.0)*(1.0-p.y)*fade*0.6;
        if alpha<0.003 {discard;}
        return vec4(0.48,0.59,0.68,alpha);
    }
    let roof=p.x > -16. && p.x < 12. && p.z > -28. && p.z < -11. && p.y < 7.;
    if roof || p.y<0.05 {discard;}
    let fade=1.-smoothstep(14.,22.,length(p.xz-motion.yz));
    let tip=sin(input.uv.y*3.14159);
    return vec4(0.48,0.59,0.68,0.20*fade*tip);
}
