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
