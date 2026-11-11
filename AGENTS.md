You are running in a Linux devcontainer set up for Bevy development with GPU access.

## GPU checks

The command sandbox hides /dev/dri and the supplementary render-device group. Sandboxed vulkaninfo --summary may therefore report only the CPU renderer (llvmpipe) even when devcontainer GPU passthrough is working.
