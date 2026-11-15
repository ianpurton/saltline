#!/usr/bin/env python3
"""Copy only the runtime static catalogue, using its Rust path mapping."""
import re
import shutil
import sys
from pathlib import Path

root = Path(__file__).resolve().parent.parent
paths = re.findall(r'"(models/[^"#]+\.glb)#Scene0"',
                   (root / 'crates/models/src/library/map_props.rs').read_text())
assert len(paths) == 33 and len(set(paths)) == 33, 'Incomplete static prop mapping'
destination = Path(sys.argv[1])
for relative in paths:
    source = root / 'assets' / relative
    assert source.read_bytes()[:4] == b'glTF', f'Invalid GLB: {source}'
    target = destination / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(source, target)
