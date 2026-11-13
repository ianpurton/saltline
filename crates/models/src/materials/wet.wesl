import bevy_pbr::{
    deferred::functions::deferred_output,
    render::pbr_fragment::pbr_input_from_standard_material,
    prepass::io::{VertexOutput, FragmentOutput},
};
@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> settings: vec4<f32>;
fn hash(p: vec2<f32>) -> f32 { return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453); }
fn noise(p: vec2<f32>) -> f32 {
    let a=floor(p); let f=fract(p); let u=f*f*(3.0-2.0*f);
    return mix(mix(hash(a),hash(a+vec2(1.0,0.0)),u.x),mix(hash(a+vec2(0.0,1.0)),hash(a+vec2(1.0)),u.x),u.y);
}
fn waves(p: vec2<f32>,t: f32) -> vec2<f32> {
    return vec2(sin(p.x*2.1+p.y*0.8+t*1.1),cos(p.y*2.7-p.x*0.4-t*0.8))*0.12
        +vec2(sin(p.x*7.2-p.y*4.0-t*1.7),cos(p.y*9.3+p.x*3.0+t*1.3))*0.055
        +vec2(sin(p.y*22.0+p.x*17.0+t*2.2),cos(p.x*25.0-p.y*11.0-t*2.0))*0.018;
}
@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var v=input;
    let p=input.world_position.xz;
    v.uv=p*0.5;
    var out=pbr_input_from_standard_material(v,is_front);
    if settings.y > 0.5 {
        let w=waves(p+vec2(noise(p*0.13),noise(p*0.17+3.0))*2.5,settings.x) / (1.0+length(fwidth(p))*3.0);
        out.N=normalize(vec3(w.x,1.0,w.y));
        out.material.base_color=vec4(0.012,0.028,0.033,1.0);
        out.material.perceptual_roughness=0.19;
    } else {
        let field=noise(p*0.22)*0.65+noise(p*0.83)*0.25+noise(p*2.4)*0.1;
        let pool=smoothstep(0.46,0.63,field);
        let ripple=waves(p*3.0,settings.x*1.8)*0.022;
        out.N=normalize(mix(out.N,vec3(ripple.x,1.0,ripple.y),pool*0.97));
        out.material.base_color=vec4(out.material.base_color.rgb*mix(0.65,0.32,pool),1.0);
        out.material.perceptual_roughness=mix(0.56,0.085,pool);
    }
    out.material.metallic=0.0;
    return deferred_output(v,out);
}
