# SALTLINE construction manual

This folder contains authored Typst sources and the published `manual.pdf`.
The reference HTML supplies model geometry, materials and gameplay facts;
Typst supplies the manual's prose and layout. GLBs are independent exports,
never inputs or source authority for the manual.

## Build

From the repository root:

```sh
npm ci
npx playwright install chromium # once, if not installed
npm run manual:build
```

Requires Python 3 (stdlib only), Node.js, Typst and Liberation Sans/Mono fonts.
Once dependencies are installed, the build works offline. Build tools and
inspection adapters live in `tools/game-manual/`.

The command extracts original builders/helpers, renders 43 inspection views
(39 models plus four extra views), extracts gameplay/map facts, generates six
SVG diagrams and compiles the PDF. All intermediate files, staged Typst sources
and the interactive gallery live in ignored `target/game-manual/`. Only the
finished PDF is copied here. The rendering laboratory is no longer required.

For prose/layout changes, reuse a valid thumbnail cache:

```sh
npm run manual:build -- --reuse-thumbnails
```

Missing or stale caches produce an actionable error; run a full build to refresh.
`--check` performs the same build and verification as a normal build. A clean
build requires no committed PNGs, SVGs, JSON or extracted JavaScript.

## Inspect models

```sh
npm run manual:rip-assets
python3 -m http.server 8000 --bind 127.0.0.1 --directory target/game-manual/gallery
```

Open <http://127.0.0.1:8000> for orbit, wireframe and PNG download controls.
`npm run manual:thumbnails` refreshes the thumbnail cache alone.

Edit `references/index.html` for source geometry and gameplay changes,
`tools/game-manual/registry.js` for authored inspection poses/material adapters,
and the Typst files here for document edits. Generated builder files are
throwaway outputs: edits there may be overwritten. Geometry extraction is
cached by input hashes and excludes the FPS bootstrap.

Thumbnails retain the original catalogue's dry studio lighting and fixed poses.
Hound feet use the original IK solver; Gull effect overlays and PAC-9 hands are
excluded. These views explain construction, not full in-game rendering parity.

The manual specifies authored locations and rules; schematic routes are not
certified walkable paths. The original HTML remains maintenance evidence for
procedural rendering, animation and gameplay behavior.
