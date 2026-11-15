# Models

`Kit::new` allocates the shared mesh and material palettes once. Procedural model
instances clone handles. Static catalogue props and backdrop models use cached
GLB scenes (see below). Active procedural model builders live under `library`;
world placement, warehouse collision and other world data stay in `saltline-world`.

`geometry` provides face-local indexed mesh construction, rounded boxes (flat or
smooth shading), ear-clipped polygon extrusion, Y-axis lathe profiles, tapered
rectangular solids, and capped tubes with transported frames. Generated meshes
include normals, UVs and tangents. Extrusion profiles must be simple,
counterclockwise and nondegenerate; lathe profiles run from bottom to top, using
zero-radius endpoints when caps are needed. Tube paths must have distinct
successive points. Dimensions and segment counts must be positive.

`geometry::assembly` spawns named components and independent articulation pivots.
The Hound uses `Leg` on hips and `Knee` on their children. Armor and actuators are
attached to the appropriate pivot, so they follow gameplay animation. The static
inspection example deliberately freezes articulation for repeatable comparison.

Run `scripts/hound-inspect.sh` for neutral, isolated 1280×720 side, front,
three-quarter and rear screenshots under `tmp/screenshots/hound-*.png`. The
script uses the same screenshot observer and software Vulkan/Xvfb approach as
`scripts/screenshot.sh`. Run `cargo test --workspace` for gameplay, mesh attribute,
triangle degeneracy, hierarchy and handle reuse checks.

Native builders cover containers, drones, the Hound, relay indicators and the
rifle. Static catalogue placements and backdrop models use the GLB scenes
described below.

## Reference comparison

The four inspection views were rendered and reviewed against
`assets/reference/hound.png`. The side silhouette now has the reference's long
armored body, raised sensor pod, rack and antenna, circular hip/knee joints, and
rearward upper/forward lower leg bends. Three-quarter views expose separate armor,
actuators, hoses and structural members instead of a single block per limb. Front
and rear views confirm four distinct articulated legs and a clear underbody gap.
An initial inspection exposed thin shins and oversized rectangular pads; the
second pass added shaped shin plates, safety cuffs, clipped foot profiles, joint
fasteners and side optics.

Remaining differences: the paint is clean (no chipped paint or stenciled unit
markings), feet are broader, the head uses paired front optics plus side optics,
and the reference's densely packed equipment and asymmetrical mechanical detail
are simplified. These are visible differences, not a claim of exact reproduction.
The other models retain their original blockout fidelity.

## Static GLB props

Install `static_props::StaticPropsPlugin` after Bevy's asset/GLTF plugins when
using `library::map_props::build`. The builder returns the same local root and
queues a catalogue scene on it. The plugin caches scene handles across instances;
`StaticPropsReady` becomes true once dependencies and scene instances are ready.
Load failures identify the asset path and stop the application.

All 33 `PropKind` models use `assets/models/*.glb` with embedded PBR materials.
`PropOptions` remains source-compatible, but color, seed, forks, fence posts and
sign choices no longer affect the fixed exported visuals. `PropKind::size` still
defines world collision; GLB bounds do not replace colliders. Containers, enemy
rigs, weapons, relay indicators and shared utilities retain their
existing builders. Ship, boat, quay crane and warehouse visuals use scenery GLBs. Exported materials do not reproduce custom wetness.

Render a headless overview of every static prop with:

```sh
cargo run -p saltline-models --example static_props_inspect
```

The output is `tmp/static-props/catalogue.png`. Tiles run in `PropKind::ALL`
order, six per row, from the far left across each row. Models are scaled to fit
for this overview only; in-game instances keep their authored transforms.

Known collision/visual differences: imported equipment can extend beyond the
legacy collider envelopes. In particular, the relay is about 2.57 × 3.18 × 1.17 m
versus its 1.2 × 2.4 × 0.8 m cabinet collider; the booth roof, fence feet and light
fixtures also overhang. Stair rails rise to about 3.62 m above the ground while
collision follows the existing steps/platform. Collision is intentionally unchanged.

Native debug and release builds use the same asset discovery: an explicit
`BEVY_ASSET_ROOT/assets` override, then executable-adjacent `assets`, then working
directory `assets`, then the build-time workspace `assets`. An explicit override
is respected even if missing. Local release runs from the repository need no
environment override. Release distributions include `assets/models` next to the
executable; Linux CI and web packaging copy the 38 runtime models with
`scripts/package-static-props.py`. Web hosting serves `assets/models` alongside
`index.html` and does not use native discovery.


Backdrop roots use `StaticModelPath(SceneryKind::Ship.asset_path())` (also `Boat`,
`Crane`, `Warehouse` and `Skyline`). They share the scene cache, loading failure handling and
`StaticPropsReady` gate with catalogue props. Original placement and collision
remain in `saltline-world`. The imported `ship-radar`, `boat-hull` and
`crane-spreader` nodes retain their pivots for later animation; no new procedural
motion is enabled by this replacement.


Inspect the new backdrop models in the game with `--inspect ship`, `--inspect boat`
or `--inspect crane` (add `--headless-render` to save a capture without a window).

The skyline is exported at world identity from the original `Mb` builder with
`node scripts/export-assets.cjs skyline`. Its single batched mesh replaces the
placeholder distant scenery and never casts or receives shadows. The exporter
also writes `assets/models/skyline.json`: HDR lamp colors, octahedron sizes and
blink parameters, plus the 12 seeded gull orbits from `wb`. The game embeds this
metadata and animates two batched meshes using `PresentationClock`; pause and
inspection captures therefore use the same timing rules as other presentation.
The existing detailed `gull.glb` is a separate catalogue model.
