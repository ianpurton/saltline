#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p tmp/screenshots tmp/logs
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
cargo build -p saltline-game
for view in port rifle rifle-side hound drone relay warehouse; do
  xvfb-run -a -s '-screen 0 1280x720x24' target/debug/saltline-game --inspect "$view" > "tmp/logs/$view.log" 2>&1
done
