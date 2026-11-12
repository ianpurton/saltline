#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p tmp/logs
compare=0
options=()
for arg in "$@"; do
  if [[ "$arg" == --compare-overhead ]]; then compare=1; else options+=("$arg"); fi
done
cargo build --release -p saltline-game
vulkaninfo --summary > tmp/logs/adapter.txt 2>&1
artifacts=()
for run in 1 2 3; do
  logfile="tmp/logs/benchmark-${run}.log"
  artifact="tmp/logs/benchmark-${run}.json"
  rm -f "$artifact"
  target/release/saltline-game --headless-render --benchmark --no-audio --resolution 1920x1080 "${options[@]}" --benchmark-output "$artifact" > "$logfile" 2>&1
  rg 'AdapterInfo|STATIC CONTAINERS|BENCHMARK|MISSION DRIVER PASSED' "$logfile"
  artifacts+=("$artifact")
  if [[ "$compare" == 1 ]]; then
    baseline="tmp/logs/benchmark-baseline-${run}.json"
    rm -f "$baseline"
    target/release/saltline-game --headless-render --benchmark --no-audio --resolution 1920x1080 "${options[@]}" --no-instrumentation --benchmark-output "$baseline" > "tmp/logs/benchmark-baseline-${run}.log" 2>&1
  fi
done
# Report acceptance after the optional comparison so failed budgets still retain evidence.
if [[ "$compare" == 1 ]]; then
  python3 scripts/compare-benchmark.py
fi
python3 scripts/check-benchmark.py "${artifacts[@]}"
