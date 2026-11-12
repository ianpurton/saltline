struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};
@group(0) @binding(0) var scene:texture_2d<f32>;
@group(0) @binding(1) var scene_sampler:sampler;
struct Settings {params:vec4<f32>};
@group(0) @binding(2) var<uniform> settings:Settings;
@fragment fn fragment(input:FullscreenVertexOutput)->@location(0) vec4<f32> {
    var col=textureSample(scene,scene_sampler,input.uv).rgb;
    let l=dot(col,vec3(0.2126,0.7152,0.0722));
    col*=mix(vec3(0.80,1.01,1.13),vec3(1.07,1.,0.90),smoothstep(0.03,0.55,l));
    col=mix(col,col*col*(3.-2.*col),0.22);
    col=col*0.988+vec3(0.006,0.0085,0.011);
    col=mix(vec3(dot(col,vec3(0.2126,0.7152,0.0722))),col,1.-0.65*settings.params.z-0.35*settings.params.y);
    let p=(input.uv-0.5)*2.;
    let r2=dot(p,p)*0.5;
    let edge=smoothstep(0.15,1.05,r2);
    col*=1.-(0.20+settings.params.z*0.4+settings.params.w*0.15)*edge*edge;
    col=mix(col,vec3(0.55,0.02,0.01)*(0.35+l),clamp(settings.params.y*edge*0.65,0.,0.8));
    let grain=fract(sin(dot(input.position.xy+settings.params.x,vec2(12.9898,78.233)))*43758.5453)-0.5;
    col+=grain*0.004;
    return vec4(clamp(col,vec3(0.),vec3(1.)),1.);
}
