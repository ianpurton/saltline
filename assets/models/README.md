# Exported SALTLINE catalogue

These 39 self-contained GLBs are generated from the original procedural model
builders extracted into ignored `target/game-manual/gallery`. The exporter
automatically prepares these inputs from `references/index.html` when needed;
it does not rebuild the manual or overwrite authored sources.

From the repository root:

```sh
npm ci
npx playwright install chromium # once, if needed
npm run assets:export            # all 39
npm run assets:export -- hound container20 pac9
```

After installation, export works offline. Three.js and its exporter/loader are
pinned to revision 180. Chromium uses software WebGL for consistent readback.
Seed 7, dry surfaces, and the existing 256-quality catalogue materials are used.
Original dimensions, local origins, mesh attributes, and enemy skeletons survive;
material groups become separate glTF primitives. No gallery floor or lights are
included. GLBs contain embedded PNG textures; no external texture files are needed.

`manifest.json` records input hashes, GLB hashes, geometry and posed bounds before
and after reload, joint deformation checks, texture readback evidence, preview
coverage, validator warnings, and per-model conversion limitations. Selected
exports preserve other manifest entries and their own input hashes. Reloaded
previews are written to `target/asset-export/<id>.png`.

Source tint modes 1 and 2 combine texture RGB and alpha with material tint. This
combination is baked into base color in linear space before PNG encoding. Vertex
colors remain glTF multipliers. Out-of-range vertex colors and roughness are
clamped, and HDR unlit base colors normalized, with limitations recorded. Raw GPU
texture row zero is v=0; canvas textures retain their original flip convention.

This is a standard PBR approximation, not exact rendering parity. Custom macro
variation, wetness, triplanar projection, shader normal adjustments, and
subpixel fence dithering are not carried as executable shaders. Triplanar
materials use existing mesh UVs. Inspect the per-model limitations before reuse.
The catalogue PAC-9 uses studio material adaptations and excludes hands.
Procedural animations, AI, collision, gameplay, effects, and the assembled level
are not encoded. Enemy skeletons are posed but have no animation clips.

Load scenes in Bevy 0.20 with `WorldAssetRoot` and `models/hound.glb#Scene0`.
A standalone smoke viewer is available:

```sh
cargo run -p saltline-models --example glb_inspect -- hound
# For Xvfb (which lacks DRI3 presentation):
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json \
  xvfb-run -a cargo run -p saltline-models --example glb_inspect -- container20
```

The viewer exits after loading and saving `target/asset-export/bevy.png`.
The game uses the 33 static catalogue GLBs through `StaticPropsPlugin`; containers,
enemies and the weapon still use their existing procedural models.
