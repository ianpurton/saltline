#!/usr/bin/env python3
"""Rebuild source data, vector illustrations and the Typst PDF. Stdlib only."""
from pathlib import Path
import subprocess, json, math, html, argparse, xml.etree.ElementTree as ET
ROOT=Path(__file__).resolve().parent
INK='#27343c'; BLUE='#50758a'; AMBER='#b77916'; RUST='#a94c30'; PALE='#f5f1e8'
def run(*args): subprocess.run(args,cwd=ROOT,check=True)
class SVG:
 def __init__(self,w,h): self.w=w;self.h=h;self.s=[]
 def line(self,x,y,X,Y,color=BLUE,width=1,dash=False): self.s.append(f'<line x1="{x:.2f}" y1="{y:.2f}" x2="{X:.2f}" y2="{Y:.2f}" stroke="{color}" stroke-width="{width}"'+(' stroke-dasharray="4 3"' if dash else '')+'/>')
 def rect(self,x,y,w,h,fill='none',stroke=BLUE): self.s.append(f'<rect x="{x:.2f}" y="{y:.2f}" width="{w:.2f}" height="{h:.2f}" fill="{fill}" stroke="{stroke}"/>')
 def text(self,x,y,t,size=11,color=INK): self.s.append(f'<text x="{x:.2f}" y="{y:.2f}" font-family="Liberation Sans" font-size="{size}" fill="{color}">{html.escape(str(t))}</text>')
 def circle(self,x,y,r=3,color=AMBER): self.s.append(f'<circle cx="{x:.2f}" cy="{y:.2f}" r="{r}" fill="{color}"/>')
 def save(self,name):
  p=ROOT/'assets'/name;p.write_text(f'<svg xmlns="http://www.w3.org/2000/svg" width="{self.w}" height="{self.h}" viewBox="0 0 {self.w} {self.h}">'+''.join(self.s)+'</svg>');ET.parse(p)
def map_svg(data):
 d=SVG(820,640);sx=8;sy=8
 def p(x,z):return 75+(x+40)*sx,55+(z+30)*sy
 for x in range(-40,41,10):
  d.line(*p(x,-30),*p(x,30),'#ddd8ce',.7);d.text(p(x,-30)[0]-9,42,str(x),10)
 for z in range(-30,31,10):
  d.line(*p(-40,z),*p(40,z),'#ddd8ce',.7);d.text(42,p(-40,z)[1]+4,str(z),10)
 d.rect(*p(30,-30),80,480,'#e2ebed');d.text(*p(31,-25),'SEA',12)
 for x,z,w,h,name in [(-40,-30,16,60,'GATE'),(-24,-10,40,40,'YARD'),(-24,-30,40,20,'SHED ZONE'),(16,-30,24,60,'QUAY')]:
  X,Y=p(x,z);d.rect(X,Y,w*sx,h*sy);d.text(X+5,Y+15,name,11)
 sh=data['objects']['no'];X,Y=p(sh['X0'],sh['Z0']);d.rect(X,Y,(sh['X1']-sh['X0'])*sx,(sh['Z1']-sh['Z0'])*sy,'#e9e4d9');d.text(X+5,Y+30,'WAREHOUSE / RIDGE 9.5 m',10)
 for st in data['stacks']:
  length=6.058 if st['type']=='20' else 12.192;X,Y=p(st['x']-length/2,st['z']-1.219);d.rect(X,Y,length*sx,2.438*sy,'#dce3e6')
 for sp in data['spawns']:
  x,y=p(sp['pos'][0],sp['pos'][2]);
  if sp['type']=='air': d.rect(x-2,y-2,4,4,'none',RUST)
  else:d.circle(x,y,2,RUST)
 for k,label in [('player','START'),('relay_a','A / YARD'),('relay_b','B / SHED'),('relay_c','C / QUAY'),('extraction','EXTRACT')]:
  x,y=p(data['locations'][k][0],data['locations'][k][2]);d.circle(x,y,5);d.text(x+8,y-7,label,11)
 # Proposed route is explanatory, not a route enforced by the game.
 route=[data['locations'][k] for k in ['player','relay_a','relay_b','relay_c','extraction']]
 for a,b in zip(route,route[1:]):d.line(*p(a[0],a[2]),*p(b[0],b[2]),AMBER,1.3,True)
 d.text(75,565,'X → east / Z ↓ south / metres / Y is elevation',12)
 d.text(75,585,'● Ground spawn   □ Air spawn   ● Amber: mission location',11)
 d.text(75,605,'Container rectangles: first-tier footprints; stack heights, rotated stacks and props omitted.',11)
 d.text(75,625,'Dashed connections explain mission flow; they are not verified walkable routes.',11)
 d.save('level-map.svg')
def diagrams(data):
 # Logical rig diagram, not an asset reconstruction.
 d=SVG(760,190)
 labels=[('ROOT',30,30),('BODY',140,30),('NECK',270,30),('HEAD',390,30),('TURRET',270,80),('GUN',390,80),('ABD ×4',270,135),('THIGH',390,135),('SHIN',510,135)]
 for t,x,y in labels:d.rect(x,y,90,26,PALE);d.text(x+6,y+17,t,11)
 for x,y,X,Y in [(120,43,140,43),(230,43,270,43),(360,43,390,43),(230,43,250,43),(250,43,250,148),(250,93,270,93),(360,93,390,93),(250,148,270,148),(360,148,390,148),(480,148,510,148)]:d.line(x,y,X,Y)
 d.text(30,184,'Abduction Z • thigh/knee X • turret Y • gun X • separate antenna and beacon children',11)
 d.save('hound-rig.svg')
 d=SVG(760,180);sh=data['objects']['no']
 # Warehouse cross-section in Z/Y, source dimensions.
 def p(z,y):return 65+(z-sh['Z0'])*23,145-y*12
 for a,b in [((sh['Z0'],0),(sh['Z1'],0)),((sh['Z0'],0),(sh['Z0'],sh['EAVE'])),((sh['Z1'],0),(sh['Z1'],sh['EAVE'])),((sh['Z0'],sh['EAVE']),(sh['ZR'],sh['RIDGE'])),((sh['ZR'],sh['RIDGE']),(sh['Z1'],sh['EAVE']))]:d.line(*p(*a),*p(*b),BLUE,2)
 d.text(490,38,'WAREHOUSE / Z–Y SECTION',12)
 d.text(490,65,'Depth: 17 m / X span: 28 m',11)
 d.text(490,87,'Eaves: 7 m / ridge: 9.5 m',11)
 d.text(490,109,'Floor: Y 0.03 m / roof ridge: Z −19.5',11)
 d.text(65,173,'Structural envelope from no; doors, skylights, rafters and interior props omitted.',10)
 d.save('warehouse-section.svg')
 d=SVG(760,130)
 names=['TITLE','INTRO','PLAYING','EXTRACTION','COMPLETE']
 for i,t in enumerate(names):
  x=10+i*150;d.rect(x,20,130,32,PALE);d.text(x+8,41,t,12)
  if i<4:d.line(x+130,36,x+150,36,AMBER,2)
 d.text(310,80,'↕ PAUSED',12);d.text(310,105,'DEAD → RETRY → INTRO',12,RUST)
 d.save('mission.svg')
 d=SVG(760,160)
 for i in range(4):
  x=20+i*185;d.text(x,18,f'POSE {i+1}',11)
  d.rect(x+25,40,110,30)
  for j in range(4):
   hip=x+35+(j%2)*85; y=65; swing=(j in ([0,3] if i%2==0 else [1,2])); foot=hip+(15 if swing else -5)
   d.line(hip,y,hip+12,102);d.line(hip+12,102,foot,130-(18 if swing else 0));d.circle(foot,130-(18 if swing else 0),3,RUST if swing else BLUE)
  d.line(x+10,132,x+155,132,INK)
 d.text(20,155,'Schematic diagonal stepping: FL + RR alternate with FR + RL. Swing arcs lift; stance feet stay planted.',11)
 d.save('gait.svg')
 d=SVG(760,200)
 for i,(label,x,y) in enumerate([('HEADING / COMPASS',260,15),('OBJECTIVE / RELAYS',12,40),('CROSSHAIR',320,90),('HEALTH',12,160),('AMMO / RESERVE',555,160),('INTERACT / HOLD',280,130)]):
  d.rect(x,y,170 if i!=2 else 85,25);d.text(x+5,y+17,label,10)
 d.text(12,195,'Layout schematic; normalized to the reference HUD hierarchy, not pixel-exact.',10)
 d.save('hud.svg')
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');parser.add_argument('--reuse-thumbnails',action='store_true',help='Compile using existing thumbnails without launching Chromium');args=parser.parse_args()
 run('node','extract.cjs');data=json.loads((ROOT/'data/reference.json').read_text())
 if not args.reuse_thumbnails: run('node','render-thumbnails.cjs')
 manifest=json.loads((ROOT/'assets/thumbnails/manifest.json').read_text())
 for entry in manifest:
  assert (ROOT/'assets/thumbnails'/entry['filename']).is_file(),entry['filename']
 map_svg(data);diagrams(data)
 for asset in (ROOT/'assets').glob('*.svg'):ET.parse(asset)
 run('typst','compile','--root',str(ROOT),'manual.typ','manual.pdf')
 # Text extraction is also available via Typst query, without a PDF dependency.
 result=subprocess.run(['typst','query','--root',str(ROOT),'manual.typ','heading'],cwd=ROOT,check=True,capture_output=True,text=True)
 headings=json.loads(result.stdout);assert len(headings)>=20,'Missing document sections'
 print(f'Validated {len(list((ROOT/"assets").glob("*.svg")))} SVGs and {len(headings)} headings; PDF compiled.')
if __name__=='__main__':main()
