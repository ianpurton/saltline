#!/usr/bin/env python3
"""Download CC0 source maps and normalize their encoding for embedded Bevy assets.
Requires Pillow. Sources are retained in tmp/material-sources for reproducibility.
"""
from pathlib import Path
import urllib.request, hashlib, json
from PIL import Image
root=Path(__file__).resolve().parents[1]
cache=root/'tmp/material-sources'; cache.mkdir(parents=True,exist_ok=True)
out=root/'assets/materials'; out.mkdir(parents=True,exist_ok=True)
manifest={}
for asset,label in [('asphalt_03','asphalt'),('concrete_floor_02','concrete')]:
 for kind,name in [('diff','color'),('nor_gl','normal'),('arm','arm')]:
  url=f'https://dl.polyhaven.org/file/ph-assets/Textures/png/2k/{asset}/{asset}_{kind}_2k.png'
  source=cache/f'{asset}_{kind}.png'
  if not source.exists():
   urllib.request.urlretrieve(url,source)
  dest=out/f'{label}-{name}.png'
  im=Image.open(source).convert('RGB'); im.save(dest,optimize=True)
  manifest[dest.name]={'source':url,'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'sha256':hashlib.sha256(dest.read_bytes()).hexdigest()}
  print(dest.name, dest.stat().st_size,flush=True)
(root/'assets/materials/manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
url='https://dl.polyhaven.org/file/ph-assets/HDRIs/hdr/2k/industrial_sunset_02_puresky_2k.hdr'
dest=root/'assets/sky/industrial_sunset_02_puresky_2k.hdr'
if not dest.exists(): urllib.request.urlretrieve(url,dest)
print('sky',dest.stat().st_size,flush=True)
