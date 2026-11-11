#!/usr/bin/env bash
# Capture two frames of the live startup menu (no gameplay or inspection bypass).
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p tmp/screenshots tmp/logs
cargo build -p saltline-game
# Xvfb has no DRI3 presentation, so use software Vulkan for this capture.
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json \
  xvfb-run -a -s '-screen 0 1280x720x24' \
  target/debug/saltline-game --title-capture > tmp/logs/title-screen.log 2>&1
! rg 'panicked|ERROR' tmp/logs/title-screen.log
