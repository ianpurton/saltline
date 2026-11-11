#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p tmp/logs
cargo build -p saltline-game
vulkaninfo --summary > tmp/logs/adapter.txt 2>&1
if [[ -n ${DISPLAY:-} ]]; then
  target/debug/saltline-game --mission-test > tmp/logs/mission.log 2>&1
else
  export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
  xvfb-run -a -s '-screen 0 1280x720x24' target/debug/saltline-game --mission-test > tmp/logs/mission.log 2>&1
fi
rg 'MISSION DRIVER PASSED|FRAME TIMINGS|AdapterInfo' tmp/logs/mission.log
