#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
xvfb-run -a -s '-screen 0 1280x720x24' bash -c '
set -euo pipefail
target/debug/saltline-game > tmp/logs/window-smoke.log 2>&1 &
pid=$!
trap "kill $pid 2>/dev/null || true" EXIT
window=$(xdotool search --sync --name "SALTLINE / port" | head -1)
xdotool windowfocus "$window"
xdotool key Return
sleep 2
xdotool keydown w
sleep 1
xdotool keyup w
xdotool mousemove_relative -- 30 10
xdotool mousedown 3
sleep 1
xdotool mouseup 3
xdotool mousedown 1
sleep 1
xdotool mouseup 1
xdotool key r
sleep 2
xdotool key Escape
sleep 1
xdotool key Return
sleep 1
kill -0 "$pid"
'
! rg 'panicked|ERROR' tmp/logs/window-smoke.log
