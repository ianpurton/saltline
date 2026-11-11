#!/usr/bin/env bash
set -euo pipefail

# Resolve project from this script's location.
ROOT="$(realpath "$(dirname "$0")/..")"
CONFIG="$ROOT/.devcontainer/devcontainer.json"

[[ -f "$CONFIG" ]] || {
    echo "Missing $CONFIG"
    exit 1
}

for cmd in docker devcontainer; do
    command -v "$cmd" >/dev/null || {
        echo "Missing executable: $cmd" >&2
        exit 1
    }
done

docker info >/dev/null
export BUILDKIT_PROGRESS=plain

echo "Building devcontainer for $ROOT (existing container stays until build succeeds)..."
devcontainer build --workspace-folder "$ROOT" --config "$CONFIG" --log-level debug

echo "Recreating and starting devcontainer..."
devcontainer up \
    --workspace-folder "$ROOT" \
    --config "$CONFIG" \
    --id-label "devcontainer.local_folder=$ROOT" \
    --id-label "devcontainer.config_file=$CONFIG" \
    --remove-existing-container \
    --log-level debug

echo "Done. Open the devcontainer in Zed to attach."
