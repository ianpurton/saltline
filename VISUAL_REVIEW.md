# Visual rendering pass

The pass keeps the game/world/models dependency structure and gameplay intact.
All images below are actual Bevy window captures at 1280×720 through software
Vulkan/Xvfb, using the existing deterministic inspection camera positions.

| View | Before | After |
| --- | --- | --- |
| Port | [PNG](tmp/visual-review/before/port.png) | [PNG](tmp/screenshots/port.png) |
| Hound | [PNG](tmp/visual-review/before/hound.png) | [PNG](tmp/screenshots/hound.png) |
| Drone | [PNG](tmp/visual-review/before/drone.png) | [PNG](tmp/screenshots/drone.png) |
| First person | [PNG](tmp/visual-review/before/rifle.png) | [PNG](tmp/screenshots/rifle.png) |
| Relay | [PNG](tmp/visual-review/before/relay.png) | [PNG](tmp/screenshots/relay.png) |
| Warehouse | No original inspection camera | [PNG](tmp/screenshots/warehouse.png) |

Initial inspection against `assets/reference/` found dark flat surfaces, a missing
sky and reflective horizon, bare roofs, rectangular puddles and simple machinery.

Changes: deterministic tileable albedo, tangent-space normal and linear packed
roughness/metalness maps; nonmetallic asphalt, concrete and water; rusted paint;
stronger warm directional lighting with contact shadows; atmosphere-driven sky
and environment lighting, ACES tone mapping; irregular flat puddle meshes using
the existing screen-space reflections; expanded sea; two low-density fog layers;
container roof ribs, corner castings and doors; Hound panels, bolts, rails and
pistons; drone chassis details and landing skids; crane trolleys, spreaders and
webbing; pallets, crates, shed roof ribs, loaded racks and interior lights.

Intermediate captures in `tmp/visual-review/materials/` exposed excessive relief
and repeating weathering. Subsequent iterations reduced normal strength, replaced
sinusoidal patches with blended tileable noise, removed rust from bare machinery
metal and reduced mist density after inspecting the rendered results.

Validation: workspace build, five existing gameplay tests, `git diff --check`,
and rendered captures after the material/lighting stage and geometry/atmosphere
stage. Run `./scripts/screenshot.sh` to recreate the current captures.

Remaining reference differences: block-based silhouettes and square armour edges,
less dense scenery, no authored labels/decals, simple sky without storm clouds,
static water normals and sparse rain without splash particles. The shed remains
an open dock structure rather than the reference's enclosed warehouse. Reflections
are screen-space with atmospheric environment lighting for off-screen content;
they are not planar or ray-traced reflections. The low fog volumes have uniform
density rather than turbulent noise. No hardware frame-rate claim is made.

## PROMPT4 fidelity iterations

The new pass preserves the crate boundaries and simulation. Initial comparison
identified sparse scenery, open warehouse sheds, a box hull, pale weapon rails,
coarse shared weathering, and a featureless sky as the largest differences.
Per-system inspection notes are in [the working review](tmp/prompt4/review-notes.md).

| View | Before this pass | Final capture |
| --- | --- | --- |
| Port | [PNG](tmp/prompt4/before/port.png) | [PNG](tmp/screenshots/port.png) |
| Warehouse | [PNG](tmp/prompt4/before/warehouse.png) | [PNG](tmp/screenshots/warehouse.png) |
| First person | [PNG](tmp/prompt4/before/rifle.png) | [PNG](tmp/screenshots/rifle.png) |
| Rifle side | — | [PNG](tmp/screenshots/rifle-side.png) |
| Hound | [PNG](tmp/prompt4/before/hound.png) | [PNG](tmp/screenshots/hound.png) |
| Drone | [PNG](tmp/prompt4/before/drone.png) | [PNG](tmp/screenshots/drone.png) |
| Relay | [PNG](tmp/prompt4/before/relay.png) | [PNG](tmp/screenshots/relay.png) |

Geometry now includes chamfered weapon mechanisms, a framed stock and segmented
magazine; robot identification and service hardware; drone motor assemblies;
container locking rods, hinges and capacity plates; crane portal legs, access
ladders, machinery cabins and stayed booms; enclosed warehouse walls and loading
bays; a continuous extruded freighter hull with deck rails, funnel and mast;
relay vents, controls, foundation, antenna and cabling; forklift, drums, mooring
bollards, pipe stock, drains and fencing. The quay extends beneath the warehouses
and additional cargo, and warehouse collision bounds match the enclosing walls.

Materials distinguish fine asphalt aggregate, directional wood, hazard paint,
graphite weapon parts and worn robot paint. Lighting uses the existing PBR,
contact shadows, fog, SSR and tone mapping, with a baked directional cloud
cubemap and Bevy's filtered environment map. Rain has lightweight expanding
surface rings; destroyed enemies emit short-lived sparks with gravity and bounce.

Rendered iterations corrected the stepped prototype bow to a continuous hull,
reduced rail brightness, added a sight pedestal, removed first-person hands from
the side inspection view, and fixed the relay pulse overwriting its beacon height.
Inspection captures hide the crosshair and freeze ripple/beacon animation phases.

Remaining differences: this is still below reference fidelity. The cloud field
is baked, the water normals remain static, geometry and texture wear remain
procedural and relatively clean, and identification strokes are not full authored
typographic decals. Warehouse roofs lack the reference's full pitched structure,
mezzanine and stairs. Port density, distant skyline and loading machinery remain
less varied. Drone rotors are static, sparks are simple geometry, and there is no
smoke/fire explosion simulation. SSR retains its normal off-screen limitations.
The new sky environment replaces the former atmosphere LUT sky; distance and
local volumetric fog remain active. No GPU performance acceptance is claimed.

Verification for this pass: incremental debug build passed; all five headless
gameplay tests passed, and both procedural mesh/articulation tests passed.
Two rendered mission replays passed (10 kills each); the later run reached
victory in 33.87 simulation seconds. Its 144 measured frames had median 774.76 ms
and p95 1133.96 ms on llvmpipe while inspection captures also ran. Those shared
software-renderer measurements cannot establish hardware performance or isolate
the cost of this pass. Logs are in `tmp/prompt4/`. No new rendering dependency
or runtime cloud ray marcher was added.

The warehouse inspection camera was moved inside the open east loading bay:
its previous position now faces a solid end wall, and a first replacement was
obstructed by the new perimeter fence. Consequently that before/after pair is
not pixel-aligned. Ocean UV scale is now independent of puddle UV scale, avoiding
kilometre-wide stretched normal-map bands. Final capture paths are working
artifacts, not approved visual regression baselines.

Completion checks were rerun after the ocean material and inspection-camera fixes:
five gameplay tests and two model tests passed again; incremental build and
`git diff --check` passed. Final warehouse capture was visually inspected from
inside the loading bay. Its broad surface mottling and flat roof remain below
the warehouse reference, as do the warehouse light/fog transitions. Total elapsed
execution time was approximately 17 minutes (including software capture waits).
Token usage is not exposed by this session.

## Animated title screen (PROMPT5)

Implemented in `crates/game/src/title.rs` using the existing procedural port.
Compared `tmp/screenshots/title-70.png` and `title-170.png` against
`assets/reference/splash-screen.png`. Refined the foreground framing, title size
and weight, gradient scrim, rain contrast, and UI camera assignment after render
review. The live scene retains the project's procedural geometry/material style;
it does not reproduce the reference's photographic surface detail.

Verified with Xvfb/software Vulkan at 1280×720:
- Startup displays the title before gameplay; captured two distinct camera poses.
- Mouse Settings opens the motion option panel; Esc returns.
- Mouse Controls opens the complete input list; Esc returns.
- Mouse Deploy switches to the gameplay camera, HUD, rifle, and active enemies.
- Captures: `tmp/screenshots/title-settings.png`, `title-controls.png`,
  `title-deployed.png`.
- `cargo test --workspace`: all seven tests pass. `git diff --check` passes.

`scripts/title-screen.sh` reproduces the two startup captures. Xvfb requires
software presentation here; normal launches retain hardware renderer selection.

### Missing menu text fix

Replaced asynchronous filesystem loading of Oswald with `include_bytes!` and a
font added directly to `Assets<Font>`. Removed the build-machine asset directory
from application configuration. Verified the rebuilt executable from
`/tmp/saltline-font-check` with `BEVY_ASSET_ROOT` pointing to a nonexistent folder:
the title, button labels, summary and legend all render. Capture:
`tmp/screenshots/title-embedded-font.png`. All seven workspace tests pass.
