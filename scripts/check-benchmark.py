#!/usr/bin/env python3
"""Validate versioned benchmark artifacts; missing evidence fails acceptance."""
import json
import math
import sys
from pathlib import Path

STAGES = ('title', 'corridor', 'waterfront', 'extraction', 'combat-and-mission')


def validate(data):
    errors = []
    if data.get('version') != 1 or data.get('mission_passed') is not True:
        errors.append('missing version or completed mission')
    adapter = data.get('adapter') or ''
    if not adapter or any(x in adapter.lower() for x in ('llvmpipe', 'swiftshader', 'device_type: cpu')):
        errors.append('missing hardware adapter')
    if data.get('resolution') != [1920, 1080]:
        errors.append('acceptance requires 1920x1080')
    if data.get('build_profile') != 'release':
        errors.append('acceptance requires a release build')
    if data.get('instrumentation'):
        paths = {o.get('path') for o in data.get('observations', [])}
        required = {'saltline/main_world_ms', 'saltline/render/submissions'} | {
            f'saltline/system/{name}_ms' for name in (
                'lib/simulate', 'main/controls', 'main/visuals', 'hud/update',
                'presentation/rigs', 'rain/update', 'effects/combat_effects')}
        if required - paths or not any(p and p.startswith('render/') and p.endswith('elapsed_cpu') for p in paths):
            errors.append('missing required instrumentation evidence')
    for stage in STAGES:
        samples = data.get('stages', {}).get(stage, {}).get('raw_ms', [])
        if not isinstance(samples, list) or len(samples) < (120 if stage != 'combat-and-mission' else 1):
            errors.append(f'{stage}: missing or insufficient samples')
            continue
        if stage != 'combat-and-mission' and len(samples) != 120:
            errors.append(f'{stage}: expected exactly 120 samples')
        if any(type(v) not in (int, float) or not math.isfinite(v) or v <= 0 for v in samples):
            errors.append(f'{stage}: invalid timing')
            continue
        p95 = sorted(samples)[(len(samples) - 1) * 95 // 100]
        if p95 > 16.7:
            errors.append(f'{stage}: p95 {p95:.3f} ms exceeds 16.7 ms')
    return errors


def main(paths):
    passed = True
    for path in paths:
        try:
            errors = validate(json.loads(Path(path).read_text()))
        except (OSError, ValueError, TypeError, AttributeError) as error:
            errors = [f'invalid artifact: {error}']
        for error in errors:
            print(f'{path}: {error}')
        passed &= not errors
    print('HARDWARE ACCEPTANCE: ' + ('PASS' if passed and paths else 'FAIL'))
    return 0 if passed and paths else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
