#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p tmp/logs
cargo build --release -p saltline-game
vulkaninfo --summary > tmp/logs/adapter.txt 2>&1
# Offscreen hardware rendering works without DISPLAY or Xvfb. Never silently select llvmpipe.
for run in 1 2 3; do
  logfile="tmp/logs/benchmark-${run}.log"
  target/release/saltline-game --headless-render --benchmark --no-audio --resolution 1920x1080 "$@" > "$logfile" 2>&1
  rg 'AdapterInfo|STATIC CONTAINERS|BENCHMARK|MISSION DRIVER PASSED' "$logfile"
  if rg -qi 'AdapterInfo.*(llvmpipe|swiftshader|device_type: Cpu)' "$logfile"; then
    echo 'Software renderer: these timings cannot satisfy hardware acceptance.' >&2
    exit 1
  fi
done
python3 - <<'PY'
from pathlib import Path
import re
passed=True
for p in sorted(Path('tmp/logs').glob('benchmark-[123].log')):
    s=p.read_text()
    rows=re.findall(r'BENCHMARK stage=(\S+) samples=\d+ median_ms=[\d.]+ p95_ms=([\d.]+)',s)
    for stage,value in rows:
        if stage not in ('main-world-cpu','all') and float(value)>16.7:
            print(f'{p.name}: {stage} p95 {value} ms exceeds 16.7 ms')
            passed=False
    if not rows or 'MISSION DRIVER PASSED' not in s: passed=False
print('HARDWARE ACCEPTANCE: '+('PASS' if passed else 'FAIL'))
raise SystemExit(0 if passed else 1)
PY
