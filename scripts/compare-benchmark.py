"""Paired, descriptive overhead measurements; no assumed overhead threshold."""
import json
import sys
sys.dont_write_bytecode = True
from pathlib import Path
from statistics import median
from importlib.util import spec_from_file_location, module_from_spec

spec = spec_from_file_location('checker', Path(__file__).with_name('check-benchmark.py'))
checker = module_from_spec(spec)
spec.loader.exec_module(checker)
rows = []
for run in range(1, 4):
    enabled = json.loads(Path(f'tmp/logs/benchmark-{run}.json').read_text())
    disabled = json.loads(Path(f'tmp/logs/benchmark-baseline-{run}.json').read_text())
    # Baselines may exceed budgets, but must contain valid hardware evidence.
    for data in (enabled, disabled):
        errors = [e for e in checker.validate(data) if 'exceeds 16.7' not in e]
        if errors:
            raise SystemExit(errors)
    if not enabled['instrumentation'] or disabled['instrumentation']:
        raise SystemExit('comparison requires instrumentation enabled versus disabled')
    for stage in checker.STAGES:
        a = enabled['stages'][stage]['stats']['p95_ms']
        b = disabled['stages'][stage]['stats']['p95_ms']
        rows.append(dict(run=run, stage=stage, enabled_p95_ms=a, disabled_p95_ms=b, delta_ms=a-b, delta_percent=(a/b-1)*100))
result = dict(version=1, pairs=rows, median_delta_ms=median(r['delta_ms'] for r in rows),
              note='Sequential paired runs; thermal and system noise can affect differences. No overhead pass/fail threshold.')
Path('tmp/logs/benchmark-overhead.json').write_text(json.dumps(result, indent=2))
print(f"Instrumentation median paired p95 delta: {result['median_delta_ms']:.3f} ms")
