# SALTLINE construction manual and executable assets

The editable asset authority is **`gallery/assets/*.js`**, together with the shared original geometry/material helpers and the gallery's pose/material adapters. Thumbnails execute those JavaScript files directly. There is no JSON model reconstruction.

## Build the manual

From the repository root:

```sh
npm ci
npx playwright install chromium # only if the Playwright browser is not installed
python3 docs/game-manual/build.py
```

Requires Node.js, Python 3, Typst (tested with 0.15.1), Liberation Sans/Mono and Playwright Chromium. The renderer uses Chromium's software WebGL backend for consistent headless captures. After dependencies/browser installation, the build works offline.

A normal build refreshes HTML-derived gameplay facts, renders 39 distinct assets plus four extra inspection views to 512×512 PNG, regenerates explanatory SVG diagrams and compiles `manual.pdf`. It **does not overwrite the extracted JavaScript**. `--check` runs the same build and validation. `--reuse-thumbnails` skips rendering and compiles with existing captures; use it for prose/layout edits, not after changing asset JavaScript.

```sh
python3 docs/game-manual/build.py --reuse-thumbnails
node docs/game-manual/render-thumbnails.cjs # thumbnails only
```

## Inspect and edit assets

```sh
python3 -m http.server 8000 --bind 127.0.0.1 --directory docs/game-manual/gallery
```

Open <http://127.0.0.1:8000>. Select an asset; drag to orbit and scroll to zoom. Wireframe, reset-camera and PNG download controls are available. The gallery uses local classic scripts and can also be opened as `gallery/index.html`; HTTP is useful for automated capture.

- `gallery/assets/hound.js`, `gull.js`, `pac9.js`: original procedural geometry builders.
- Other files in `gallery/assets/`: named original prop builders; three container types share `containers.js`.
- `gallery/shared/reference.js`: bundled Three.js, original primitive/assembly/skinning/material helpers and prop registrations. Support declarations are retained to preserve dependencies; the FPS bootstrap, DOM hooks and animation loop are excluded.
- `gallery/registry.js`: readable asset names, material adapters and static inspection poses.
- `gallery/gallery.js`: renderer, lighting, camera framing and inspection controls.

The extracted builders retain their original minified symbols. Helper references resolve through classic-script globals. They are not standalone ES modules: keep the gallery's generated script order when reusing them. Asset scripts load before shared registry initialization.

The hound uses the original IK solver to place its feet. Gull rotor-blur discs/searchlight cone, weapon hands, and transient effects are excluded from static asset thumbnails. PAC-9 uses the original `w_` geometry with studio material adapters rather than the full animated first-person viewmodel shader stack. Surfaces are dry and studio-lit; this is intentionally different from the game's wet sunset environment.

## Refresh from the original HTML

This is an explicit import operation that **overwrites extracted builders and the generated gallery HTML**, so preserve local asset edits before running it:

```sh
node docs/game-manual/rip-assets.cjs
python3 docs/game-manual/build.py
```

The importer uses Acorn to locate original top-level builder declarations and removes them from the shared bundle, so each extracted builder has one authoritative definition. `gallery/provenance.json` records the original HTML fingerprint and builder symbols. A normal build never invokes this importer.

## Document files and evidence

- `manual.typ`, `theme.typ`: editable content and A4 field-manual layout.
- `extract.cjs`: HTML-derived gameplay tuning, map positions and source anchors; no asset geometry.
- `data/reference.json`: derived gameplay/map facts, original-source fingerprint and anchors.
- `assets/thumbnails/*.png`: actual rendered models, individually framed rather than mutually to scale.
- `assets/thumbnails/manifest.json`: derived dimensions, views, source paths, builder hashes and nonblank-pixel coverage.
- `assets/thumbnails/specification.json`: dimension/rig metadata read from the running asset JavaScript.
- `assets/*.svg`: map, warehouse section, mission flow, rig hierarchy, gait explanation and HUD schematic. These explain relationships; they do not construct assets.

The map labels stack count and height and rotates the authored footprints; detailed tier and prop schedules in the PDF supply offsets and dressing. Dashed links indicate mission locations, not certified walking routes. Gameplay reconstruction uses the PDF; the HTML is maintenance evidence. Asset changes use the extracted JavaScript. JSON and PNG outputs are regenerated reports, not editable model definitions.

Validation renders every registered asset, fails on missing builders/blank captures/browser errors, checks PNG dimensions and frame coverage, parses SVGs and compiles the PDF. This does not certify gameplay equivalence.

## Design evidence and maintenance

The PDF is the standalone design specification. `details.typ` supplies detailed combat, motion and placement sheets. Source navigation stays here and in `data/reference.json` (anchors and fingerprint).

Maintenance anchor map: player `Pi`, `Mt`, `x_`; mission `Yi`, `mM`; weapon `dm`, combat `zv`; hound `iM`, gull `Q_`, shared perception `Em` and tuning `Ui`; level assembly `C1`, yard `hR`, gate `fR`, warehouse `ab`, quay `cb`, stack helper `ob`; presentation `hM`, `pM`, `u_`, materials `Gg`, weather `O1`.

The extractor records yard assembly with inert methods and scans static warehouse/quay placement calls with Acorn. It never executes the game. Container schedule argument five is jitter strength, **not elevation**: all stacks begin at zero and tier heights accumulate (2.59 m standard, 2.9 m HC). Nominal catalogue bounds may differ slightly from these assembly heights.

Reconstruction review covers shot distribution, recoil retention, reload transfer timing, melee bypass of ranged ownership, sight/hearing delays, blocked-wave retries, water rescue, scoring, stacking and elevations. Cosmetic texture noise and individual particle random samples are intentionally nondeterministic.
