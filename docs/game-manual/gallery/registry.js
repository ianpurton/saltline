// Named adapters around original builders. These JS files are the asset authority.
// Classic scripts share the original minified helper bindings; no JSON reconstruction.
window.SaltlineAssets = {
  async initialize(renderer) {
    const context = {renderer, params:{tex:'256',wet:'0'}, quality:{tex:256}, rng:il(7), events:Dy(), time:{t:0,dt:0,frame:0}};
    context.mats = await Gg(context);
    context.props = Yg(context);
    return context;
  },
  entries() {
    return [
      {id:'hound', label:'Hound / H-03', source:'assets/hound.js', build: context => {
        const materials = {
          shell:context.mats.get('armor_white',{vertexColors:true}),
          metal:context.mats.get('steel_dark',{vertexColors:true}),
          glow:new lt.MeshBasicMaterial({vertexColors:true}),
          lens:new lt.MeshBasicMaterial({color:new lt.Color(8,4,.7)}),
        };
        const rig=pm(Z_(),materials);
        rig.bones.body.position.y=$v.stand;
        for(let j=0;j<4;j++) {
          const pose={}; const side=$v.side[j];
          q_(pose,side*$v.footX-side*$v.hipX,$v.footR*.6-$v.stand-$v.hipY,$v.footZ[j]-$v.fore[j]*$v.hipZ,side,$v.l0,$v.L1,$v.L2-$v.footR*.6);
          rig.bones['abd'+$v.legs[j]].rotation.z=pose.abd;
          rig.bones['thigh'+$v.legs[j]].rotation.x=pose.hip;
          rig.bones['shin'+$v.legs[j]].rotation.x=pose.knee;
        }
        rig.group.updateMatrixWorld(true);rig.mesh.computeBoundingBox();
        return rig.group;
      }},
      {id:'gull',label:'Gull',source:'assets/gull.js',build:context=>{
        const rig=pm(j_(),{
          shell:context.mats.get('armor_white',{vertexColors:true}),metal:context.mats.get('steel_dark',{vertexColors:true}),
          glow:new lt.MeshBasicMaterial({vertexColors:true}),lens:new lt.MeshBasicMaterial({color:new lt.Color(8,4,.7)}),
        });
        // Rotor discs and the long searchlight cone are motion/effect overlays,
        // intentionally omitted from the stationary construction thumbnail.
        rig.group.updateMatrixWorld(true);rig.mesh.computeBoundingBox();return rig.group;
      }},
      {id:'pac9',label:'PAC-9',source:'assets/pac9.js',build:context=>{
        const parts=w_();const group=new lt.Group();
        const mats={
          ano:new lt.MeshStandardMaterial({color:0x25272b,metalness:.45,roughness:.4,vertexColors:true}),
          gun:new lt.MeshStandardMaterial({color:0x303033,metalness:.7,roughness:.5,vertexColors:true}),
          poly:new lt.MeshStandardMaterial({color:0x39393a,roughness:.62,vertexColors:true}),
          plain:new lt.MeshStandardMaterial({color:0xffffff,roughness:.92,vertexColors:true}),
          bolt:context.mats.get('steel_dark',{vertexColors:true}),mag:context.mats.get('polymer_tan',{vertexColors:true}),round:context.mats.get('brass',{vertexColors:true}),
        };
        for(const [key,mat] of Object.entries(mats)) {if(parts[key])group.add(new lt.Mesh(parts[key],mat));}
        return group;
      }},
      ...Object.entries(Gi).map(([id,size])=>({id,label:id.replaceAll('_',' '),size:size.slice(0,3),source:'assets/'+(id.startsWith('container')?'containers':id)+'.js',build:context=>{
        // Fail rather than letting Yg silently return a bounding-box substitute.
        if(!Xg[id])throw Error('Missing registered original builder: '+id);
        const object=Xg[id](context,{seed:7,doors:0});
        if(!object)throw Error('Builder returned no geometry: '+id);
        return object;
      }})),
    ];
  },
};
