#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p tmp/screenshots tmp/logs
# Xvfb has no DRI3 presentation support; match screenshot.sh.
export VK_ICD_FILENAMES="${VK_ICD_FILENAMES:-/usr/share/vulkan/icd.d/lvp_icd.json}"
cargo build -p saltline-models --example hound_inspect
for view in side front three-quarter rear; do
  xvfb-run -a -s '-screen 0 1280x720x24' target/debug/examples/hound_inspect "$view" > "tmp/logs/hound-$view.log" 2>&1
done
