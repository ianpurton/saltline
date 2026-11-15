#!/usr/bin/env python3
"""Copy only the runtime static props and backdrop models, using its Rust path mapping."""
import re
import shutil
import sys
from pathlib import Path

root = Path(__file__).resolve().parent.parent
prop_paths = re.findall(r'"(models/[^"#]+\.glb)#Scene0"',
                        (root / 'crates/models/src/library/map_props.rs').read_text())
assert len(prop_paths) == 33 and len(set(prop_paths)) == 33, 'Incomplete static prop mapping'
scenery_paths = re.findall(r'"(models/[^"#]+\.glb)#Scene0"',
                          (root / 'crates/models/src/static_props.rs').read_text())
assert len(scenery_paths) == 5, 'Incomplete backdrop mapping'
paths = prop_paths + scenery_paths
assert len(set(paths)) == 38, 'Duplicate runtime model mapping'
destination = Path(sys.argv[1])
for relative in paths:
    source = root / 'assets' / relative
    assert source.read_bytes()[:4] == b'glTF', f'Invalid GLB: {source}'
    target = destination / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(source, target)
