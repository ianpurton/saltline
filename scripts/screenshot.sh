#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p tmp/screenshots tmp/logs
# Hardware offscreen captures by default; request software explicitly if needed.
if [[ ${SALTLINE_SOFTWARE:-0} == 1 ]]; then
  export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
fi
cargo build -p saltline-game
for view in port deployment corridor elevated waterfront extraction rifle rifle-side aim reload hound drone relay warehouse damage; do
  target/debug/saltline-game --headless-render --inspect "$view" --resolution 1280x720 "$@" > "tmp/logs/$view.log" 2>&1
done
target/debug/saltline-game --headless-render --title-capture --resolution 1280x720 "$@" > tmp/logs/title.log 2>&1
