struct Vertex { @builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32> }
struct Settings { params: vec4<f32>, lens: vec4<f32>, options: vec4<f32>, detail: vec4<f32>, sun: vec4<f32>, timing: vec4<f32> }
@group(0) @binding(0) var scene: texture_2d<f32>;
@group(0) @binding(1) var linear_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: Settings;
@group(0) @binding(3) var blurred: texture_2d<f32>;
@group(0) @binding(4) var history: texture_2d<f32>;
@group(0) @binding(5) var dirt_tex: texture_2d<f32>;
@group(0) @binding(6) var streak_tex: texture_2d<f32>;
@if(NATIVE_DEPTH) @group(0) @binding(7) var scene_depth: texture_depth_2d;
fn sample_scene(uv: vec2<f32>) -> vec3<f32> { return textureSampleLevel(scene,linear_sampler,uv,0.).rgb; }
fn luma(c: vec3<f32>) -> f32 { return dot(c,vec3(0.2126,0.7152,0.0722)); }
fn hash(p: vec2<f32>) -> f32 {
    var q=fract(vec3(p.x,p.y,p.x)*0.1031); q+=dot(q,q.yzx+33.33); return fract((q.x+q.y)*q.z);
}
fn srgb(c: vec3<f32>) -> vec3<f32> { return select(c*12.92,1.055*pow(max(c,vec3(0.)),vec3(1./2.4))-0.055,c>vec3(0.0031308)); }
fn linear(c: vec3<f32>) -> vec3<f32> { return select(c/12.92,pow(max((c+0.055)/1.055,vec3(0.)),vec3(2.4)),c>vec3(0.04045)); }
@fragment fn blur(v: Vertex) -> @location(0) vec4<f32> {
    let texel=1./vec2<f32>(textureDimensions(scene));
    var sum=vec3(0.); var weight=0.;
    for(var y=-1;y<=1;y++) { for(var x=-1;x<=1;x++) {
        let w=select(1.,2.,x==0)*select(1.,2.,y==0);
        sum+=sample_scene(v.uv+vec2<f32>(f32(x),f32(y))*texel*2.)*w; weight+=w;
    }}
    return vec4(sum/weight,1.);
}
@fragment fn meter(v: Vertex) -> @location(0) vec4<f32> {
    var sum=0.; var weights=0.;
    for(var y=0;y<5;y++) { for(var x=0;x<7;x++) {
        let uv=(vec2<f32>(f32(x),f32(y))+0.5)/vec2(7.,5.);
        let w=1.-0.75*smoothstep(0.1,0.55,length(uv-0.5));
        sum+=log(max(luma(sample_scene(uv)),0.0001))*w; weights+=w;
    }}
    let average=exp(sum/weights);
    let key=select(0.18,0.12,settings.detail.w>0.5);
    let desired=clamp(key/max(average,0.002),0.6,2.);
    let prev=textureSampleLevel(history,linear_sampler,vec2(0.5),0.);
    var exposure=desired;
    if prev.a>0.5 && settings.timing.y<0.5 { exposure=mix(prev.r*1.4+0.6,desired,1.-exp(-settings.timing.x/1.5)); }
    var visibility=settings.sun.z;
    @if(NATIVE_DEPTH) {
        var visible=0.; let size=vec2<f32>(textureDimensions(scene_depth));
        for(var i=0;i<12;i++) {
            let f=f32(i); let r=sqrt((f+0.5)/12.)*0.011; let angle=f*2.39996323;
            let uv=settings.sun.xy+vec2(cos(angle)/settings.sun.w,sin(angle))*r;
            if all(uv>=vec2(0.)) && all(uv<vec2(1.)) {
                // Bevy uses reversed Z: clear sky is zero, geometry is positive.
                visible+=select(0.,1.,textureLoad(scene_depth,vec2<i32>(uv*size),0)<=0.000001);
            }
        }
        visibility=visible/12.*settings.sun.z;
        if prev.a>0.5 && settings.timing.y<0.5 { visibility=mix(prev.g,visibility,1.-exp(-settings.timing.x*8.)); }
    }
    return vec4((exposure-0.6)/1.4,visibility,0.,1.);
}
@fragment fn dirt(v: Vertex) -> @location(0) vec4<f32> {
    var acc=0.;
    for(var i=0;i<70;i++) {
        let f=f32(i); let center=vec2(hash(vec2(f,1.7)),hash(vec2(f,9.2)));
        let size=mix(0.012,0.085,pow(hash(vec2(f,4.4)),3.));
        let r=length(v.uv-center)/size;
        acc+=(1.-smoothstep(0.86,1.,r))*(0.35+0.65*smoothstep(0.35,0.98,r))*mix(0.15,0.7,hash(vec2(f,6.1)));
    }
    let smudge=0.15*(0.5+0.5*sin(v.uv.x*31.+sin(v.uv.y*47.)));
    return vec4(vec3((acc*0.55+smudge)*(0.45+0.9*smoothstep(0.15,0.75,length(v.uv-0.5))))*vec3(1.,0.96,0.9),1.);
}
@fragment fn streak(v: Vertex) -> @location(0) vec4<f32> {
    var sum=vec3(0.); var weights=0.;
    let threshold=select(0.65,2.2,settings.detail.w>0.5);
    for(var i=-7;i<=7;i++) {
        let x=f32(i); let w=exp(-x*x/18.);
        sum+=max(sample_scene(v.uv+vec2(x*0.006,0.))-threshold,vec3(0.))*w; weights+=w;
    }
    return vec4(sum/weights,1.);
}
fn aces(c: vec3<f32>) -> vec3<f32> {
    let inset=mat3x3(vec3(0.59719,0.07600,0.02840),vec3(0.35458,0.90834,0.13383),vec3(0.04823,0.01566,0.83777));
    let outset=mat3x3(vec3(1.60475,-0.10208,-0.00327),vec3(-0.53108,1.10813,-0.07276),vec3(-0.07367,-0.00605,1.07602));
    let v=inset*(c*1.3); let a=v*(v+0.0245786)-0.000090537; let b=v*(0.983729*v+0.4329510)+0.238081;
    return clamp(outset*(a/b),vec3(0.),vec3(1.));
}
fn agx(c: vec3<f32>) -> vec3<f32> {
    let to2020=mat3x3(vec3(0.6274,0.0691,0.0164),vec3(0.3293,0.9195,0.0880),vec3(0.0433,0.0113,0.8956));
    let to_srgb=mat3x3(vec3(1.6605,-0.1246,-0.0182),vec3(-0.5876,1.1329,-0.1006),vec3(-0.0728,-0.0083,1.1187));
    let inset=mat3x3(vec3(0.856627153315983,0.137318972929847,0.11189821299995),vec3(0.0951212405381588,0.761241990602591,0.0767994186031903),vec3(0.0482516061458583,0.101439036467562,0.811302368396859));
    let outset=mat3x3(vec3(1.1271005818144368,-0.1413297634984383,-0.14132976349843826),vec3(-0.11060664309660323,1.157823702216272,-0.11060664309660294),vec3(-0.016493938717834573,-0.016493938717834257,1.2519364065950405));
    let x=clamp((log2(max(inset*(to2020*c),vec3(1e-10)))+12.47393)/16.5,vec3(0.),vec3(1.));
    let x2=x*x; let x4=x2*x2;
    var color=15.5*x4*x2-40.14*x4*x+31.96*x4-6.868*x2*x+0.4298*x2+0.1191*x-0.00232;
    let l=luma(color); color=pow(max(color,vec3(0.)),vec3(1.18)); color=l+(color-l)*1.22;
    return clamp(to_srgb*pow(max(outset*color,vec3(0.)),vec3(2.2)),vec3(0.),vec3(1.));
}
fn flare(uv: vec2<f32>) -> vec3<f32> {
    let aspect=vec2(settings.sun.w,1.); let p=(uv-0.5)*aspect; let s=(settings.sun.xy-0.5)*aspect;
    let d=p-s; let r=length(d); let angle=atan2(d.y,d.x);
    let spikes=pow(abs(cos(angle*3.+0.4)),24.)+0.6*pow(abs(cos(angle*4.-1.1)),40.);
    var c=vec3(1.,0.78,0.48)*(0.020/(r*r*9.+0.010)+0.10*exp(-r*3.2)+spikes*0.30*exp(-r*5.5));
    c+=vec3(0.30,0.75,0.55)*(1.-smoothstep(0.06,0.085,length(p+s*0.38)))*0.050;
    c+=vec3(0.85,0.45,0.25)*(1.-smoothstep(0.030,0.045,length(p+s*0.82)))*0.075;
    c+=vec3(0.35,0.50,0.95)*(1.-smoothstep(0.12,0.17,length(p+s*1.35)))*0.040;
    c+=vec3(0.80,0.70,0.35)*(1.-smoothstep(0.018,0.030,length(p-s*0.45)))*0.060;
    c+=vec3(0.45,0.55,0.85)*exp(-pow((length(p+s*0.15)-0.52)*16.,2.))*0.035*smoothstep(0.1,0.5,length(s));
    var visibility=settings.sun.z;
    @if(NATIVE_DEPTH) { visibility=textureSampleLevel(history,linear_sampler,vec2(0.5),0.).g*select(0.,1.,settings.sun.z>0.); }
    return c*visibility*settings.lens.w;
}
@fragment fn composite(v: Vertex) -> @location(0) vec4<f32> {
    let c=v.uv-0.5; let aspect=settings.sun.w;
    let r2=dot(c*vec2(aspect,1.),c*vec2(aspect,1.))/(0.25*(aspect*aspect+1.));
    let off=c*r2*(settings.options.z+select(0.,settings.params.y*0.006,settings.options.z>0.));
    var col=vec3(sample_scene(v.uv+off).r,sample_scene(v.uv).g,sample_scene(v.uv-off).b);
    let soft=textureSampleLevel(blurred,linear_sampler,v.uv,0.).rgb;
    col=mix(col,soft,settings.params.w*settings.options.y*0.75*smoothstep(0.22,0.85,r2));
    let dirt_color=textureSampleLevel(dirt_tex,linear_sampler,v.uv,0.).rgb;
    col=mix(col,soft,settings.lens.x)+soft*dirt_color*settings.lens.y;
    col+=textureSampleLevel(streak_tex,linear_sampler,v.uv,0.).rgb*vec3(0.45,0.62,1.)*settings.lens.z;
    col+=flare(v.uv)*(1.+dirt_color*1.5);
    if settings.options.x>0.5 { col*=textureSampleLevel(history,linear_sampler,vec2(0.5),0.).r*1.4+0.6; }
    if settings.detail.w>0.5 { col=mix(aces(col),agx(col),0.25); }
    col=clamp(col,vec3(0.),vec3(1.));
    var l=luma(col);
    col*=mix(vec3(0.80,1.01,1.13),vec3(1.07,1.,0.90),smoothstep(0.03,0.55,l));
    col=mix(col,col*col*(3.-2.*col),0.22);
    col=col*0.988+vec3(0.006,0.0085,0.011);
    l=luma(col);
    col=mix(vec3(l),col,(1.-0.65*settings.params.z)*(1.-0.5*settings.params.y));
    let vr=smoothstep(0.15-settings.params.w*0.08,1.05,r2);
    col*=1.-(0.3+settings.params.z*0.55+settings.params.w*0.22)*vr*vr*(3.-2.*vr);
    let edge=smoothstep(0.18,1.,r2);
    col=mix(col,vec3(0.55,0.02,0.01)*(0.35+l),clamp(settings.params.y*edge*0.85+settings.params.z*edge*edge*0.25*(0.75+0.25*sin(settings.params.x*5.)),0.,1.));
    return vec4(clamp(col,vec3(0.),vec3(1.)),1.);
}
@fragment fn finish(v: Vertex) -> @location(0) vec4<f32> {
    let texel=1./vec2<f32>(textureDimensions(scene));
    let m=srgb(sample_scene(v.uv));
    let nw=srgb(sample_scene(v.uv+vec2(-1.,-1.)*texel)); let ne=srgb(sample_scene(v.uv+vec2(1.,-1.)*texel));
    let sw=srgb(sample_scene(v.uv+vec2(-1.,1.)*texel)); let se=srgb(sample_scene(v.uv+vec2(1.,1.)*texel));
    let lm=luma(m); let lnw=luma(nw); let lne=luma(ne); let lsw=luma(sw); let lse=luma(se);
    let low=min(lm,min(min(lnw,lne),min(lsw,lse))); let high=max(lm,max(max(lnw,lne),max(lsw,lse))); let contrast=high-low;
    var col=m;
    if settings.detail.z>0.5 && contrast>max(0.0312,high*0.10) {
        var dir=vec2(-((lnw+lne)-(lsw+lse)),(lnw+lsw)-(lne+lse));
        let reduce=max((lnw+lne+lsw+lse)*0.03125,0.0078125);
        dir=clamp(dir/(min(abs(dir.x),abs(dir.y))+reduce),vec2(-8.),vec2(8.))*texel;
        let a=0.5*(srgb(sample_scene(v.uv+dir*(1./3.-0.5)))+srgb(sample_scene(v.uv+dir*(2./3.-0.5))));
        let b=a*0.5+0.25*(srgb(sample_scene(v.uv-dir*0.5))+srgb(sample_scene(v.uv+dir*0.5)));
        col=select(b,a,luma(b)<low || luma(b)>high);
    }
    col+=(m-(nw+ne+sw+se)*0.25)*settings.detail.y*(1.-smoothstep(0.04,0.22,contrast));
    let seed=v.position.xy+fract(settings.params.x*vec2(7.31,3.17))*vec2(311.7,173.3);
    let grain=hash(seed)+hash(seed+57.3)-1.; let l=luma(col);
    col+=grain*settings.detail.x*mix(1.,0.30,smoothstep(0.25,0.9,l))*(0.35+0.65*smoothstep(0.,0.08,l));
    col+=(hash(v.position.xy*1.37+11.)+hash(v.position.xy*0.71+3.)-1.)/255.;
    return vec4(linear(clamp(col,vec3(0.),vec3(1.))),1.);
}
