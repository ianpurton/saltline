// Read only the reference source. Evaluate isolated declarative definitions,
// never the bundled game, DOM code, or renderer.
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');
const crypto = require('node:crypto');
const out = __dirname;
const file = path.resolve(out, '../../references/index.html');
const source = fs.readFileSync(file, 'utf8');
const anchors = {};
// Dedicated scanner; handles strings containing braces.
function block(start) {
 let n=0,q=null,esc=false;
 for(let p=start;p<source.length;p++) {const c=source[p];if(q){if(esc)esc=false;else if(c==='\\')esc=true;else if(c===q)q=null;continue;}if('"\'`'.includes(c)){q=c;continue;}if(c==='{')n++;if(c==='}'&&--n===0)return source.slice(start,p+1);}
 throw Error('Unterminated object');
}
function record(name, anchor, p) {anchors[name]={anchor,line:source.slice(0,p).split('\n').length};}
const context=vm.createContext({Math,ts:Math.PI/2});
function object(name) {
 const re=new RegExp('(?<![\\w$])'+name.replace(/[$]/g,'\\$')+'=\\{');
 const m=re.exec(source);if(!m)throw Error('Missing '+name);
 const p=m.index+m[0].length-1, code=block(p);
 record(name,m[0],m.index);
 const value=vm.runInContext('('+code+')',context,{timeout:1000});context[name]=value;return value;
}
function fn(name) {const a=source.indexOf('function '+name+'(');if(a<0)throw Error(name);record(name,'function '+name+'(',a);const p=source.indexOf('{',a);return source.slice(a,p)+block(p);}
const data={source:'references/index.html',sha256:crypto.createHash('sha256').update(source).digest('hex'),objects:{},anchors};
for(const key of ['Pi','Mt','Yi','Kv','Zv','no','lb','Pr'])data.objects[key]=object(key);
data.weapon={};for(const key of ['Gv','B_','O_','H_','V_','W_','G_']){const m=new RegExp('\\b'+key+'=([\\d.]+)').exec(source);if(!m)throw Error(key);data.weapon[key]=Number(m[1]);record(key,m[0],m.index);}
for(const name of ['C1','hR','ab','cb','d3','x_','dm','iM','Q_','Em','mM','hM','pM','Gg','O1','Cf','u_','Z_']){
 const a=source.indexOf('function '+name+'(');if(a<0)throw Error(name);record(name,'function '+name+'(',a);
}
const level=fn('C1');
data.spawns=[];
for(const m of level.matchAll(/\b([gx])\(([-\d.]+),([-\d.]+),(?:([-\d.]+),)?"(yard|shed|quay)"\)/g)) {
 const air=m[1]==='x';data.spawns.push({type:air?'air':'ground',pos:air?[+m[2],+m[3],+m[4]]:[+m[2],0,+m[3]],zone:m[5]});
}
const coverMatch=/\b_=\[([\s\S]*?)\],M=i\.physics\.nav/.exec(level);
if(!coverMatch)throw Error('Missing cover schedule');
data.coverPoints=[...coverMatch[1].matchAll(/mt\(([-\d.]+),([-\d.]+),([-\d.]+)\)/g)].map(m=>m.slice(1).map(Number));
if(data.coverPoints.length!==55)throw Error('Cover schedule changed');
data.stacks=[];
const yard=fn('hR');
for(const m of yard.matchAll(/\bt\(([-\d.]+),([-\d.]+),\[\[(20|40|"hc"),"([a-z]+)"/g))data.stacks.push({x:+m[1],z:+m[2],type:m[3].replaceAll('"',''),color:m[4]});
data.locations={player:[-32,0,6],relay_a:[1.5,0,7.5],relay_b:[-2,data.objects.no.FLOOR,-19.5],relay_c:[23,0,0],extraction:[29,0,data.objects.lb.LADDER_Z]};
// Fail if the source-derived coordinates above no longer match their anchors.
for(const anchor of ['pos:mt(-32,0,6)','pos:mt(1.5,0,7.5)','relayPos:new W(-2,y,-19.5)','relayPos:new W(23,0,0)','extraction:new W(29,0,h)'])if(!source.includes(anchor))throw Error('Location changed: '+anchor);
if(data.spawns.length<30||data.stacks.length<20)throw Error('Extraction unexpectedly sparse');
// Evaluate the isolated yard declaration against recording-only assembly methods.
// No renderer, DOM or game bootstrap is evaluated.
data.assembly={stacks:[],placements:[]};
const recorder={place:(...args)=>data.assembly.placements.push(args),stack:(...args)=>data.assembly.stacks.push(args),stairs:(...args)=>data.assembly.placements.push(['stairs',...args]),catwalk:(...args)=>data.assembly.placements.push(['catwalk',...args]),rod:()=>{},tube:()=>{},col:()=>{},label:()=>{},decal:()=>{}};
const noop=()=>{};
vm.runInNewContext('('+fn('hR')+')(recorder)',{recorder,Math,ts:Math.PI/2,R1:.535,no:data.objects.no,Ef:noop,e0:noop,Qa:noop,lv:noop,Lr:noop,Ka:noop,Sf:noop},{timeout:1000});
// Record static warehouse/quay placements without executing their geometry routines.
const acorn=require('acorn');
for(const [name,alias,baseY] of [['ab','Y',data.objects.no.FLOOR],['cb','c',0]]) {
 const code=fn(name), tree=acorn.parse(code,{ecmaVersion:'latest'});
 function visit(node) {
  if(!node||typeof node!=='object')return;
  if(node.type==='CallExpression') {
   const callee=code.slice(node.callee.start,node.callee.end);
   if(['shelf_rack','bollard_mooring'].includes(node.arguments[0]?.value))return;
   if((callee===alias && node.arguments[0]?.type==='Literal' && typeof node.arguments[0].value==='string') || callee==='i.stack') {
    try {
     const args=node.arguments.map(a=>vm.runInNewContext('('+code.slice(a.start,a.end)+')',{Math,V:Math.PI/2,y:baseY}));
     if(callee==='i.stack') data.assembly.stacks.push(args);
     else {if(args.length<6)args[5]=baseY;data.assembly.placements.push(args);}
    } catch(error) {throw Error('Unresolved static placement: '+code.slice(node.start,node.end));}
   }
  }
  for(const value of Object.values(node))if(Array.isArray(value))value.forEach(visit);else if(value&&typeof value==='object')visit(value);
 }
 visit(tree);
}
for(const x of [-8.4,-5.6,-2.8,2.8,5.6,8.4])data.assembly.placements.push(['shelf_rack',x,-27.13,0,{},.03]);
for(const x of [2.8,5.6,8.4])data.assembly.placements.push(['shelf_rack',x,-23.45,0,{},.03]);
const gateRecorder={...recorder,place:(...args)=>{data.assembly.placements.push(args);return {scale:{x:1}}},box:noop,slab:noop,cyl:noop};
vm.runInNewContext('('+fn('fR')+')(recorder)',{recorder:gateRecorder,Math,ts:Math.PI/2},{timeout:1000});
fs.mkdirSync(path.join(out,'data'),{recursive:true});fs.writeFileSync(path.join(out,'data/reference.json'),JSON.stringify(data,null,2)+'\n');
console.log(`Extracted gameplay facts, ${data.spawns.length} spawns, ${data.stacks.length} stack footprints; no asset geometry.`);
