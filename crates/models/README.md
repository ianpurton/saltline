# Procedural models

`Kit::new` allocates the shared mesh and material palettes once. All model
instances clone handles. Existing `Kit` entry points and material indices remain
available; environmental assemblies live under `library` and world placement and
collision data stay in `saltline-world`.

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

Barrel and fence modules reserve the requested source layout; the original game
contained no barrel or fence assembly to migrate.

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
