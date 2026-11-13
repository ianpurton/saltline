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
data.stacks=[];
const yard=fn('hR');
for(const m of yard.matchAll(/\bt\(([-\d.]+),([-\d.]+),\[\[(20|40|"hc"),"([a-z]+)"/g))data.stacks.push({x:+m[1],z:+m[2],type:m[3].replaceAll('"',''),color:m[4]});
data.locations={player:[-32,0,6],relay_a:[1.5,0,7.5],relay_b:[-2,data.objects.no.FLOOR,-19.5],relay_c:[23,0,0],extraction:[29,0,data.objects.lb.LADDER_Z]};
// Fail if the source-derived coordinates above no longer match their anchors.
for(const anchor of ['pos:mt(-32,0,6)','pos:mt(1.5,0,7.5)','relayPos:new W(-2,y,-19.5)','relayPos:new W(23,0,0)','extraction:new W(29,0,h)'])if(!source.includes(anchor))throw Error('Location changed: '+anchor);
if(data.spawns.length<30||data.stacks.length<20)throw Error('Extraction unexpectedly sparse');
fs.mkdirSync(path.join(out,'data'),{recursive:true});fs.writeFileSync(path.join(out,'data/reference.json'),JSON.stringify(data,null,2)+'\n');
console.log(`Extracted gameplay facts, ${data.spawns.length} spawns, ${data.stacks.length} stack footprints; no asset geometry.`);
