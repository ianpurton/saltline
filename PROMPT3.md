# Refactor Procedural Models and Improve Geometry Quality

Refactor the existing Saltline repository using Rust and Bevy 0.20.

The current implementation in `crates/models/src/lib.rs` builds most objects from scaled cubes, spheres and toruses. This was effective for creating the initial playable game, but the geometry is too primitive to reproduce the supplied Saltline references.

**Objective:** Introduce a reusable procedural hard-surface geometry library and give every 3D model its own Rust source file. Rebuild the Hound first to demonstrate a substantial improvement in geometry quality.

Do not rewrite the game, change engines or introduce Blender.

## 1. Target source tree

Refactor `crates/models` toward this structure:

```text
crates/models/
├── Cargo.toml
└── src/
    ├── lib.rs                 # Public model API and module exports
    │
    ├── geometry/              # REUSABLE PROCEDURAL GEOMETRY
    │   ├── mod.rs             # Geometry builder and shared utilities
    │   ├── mesh.rs            # Vertices, indices, normals, UVs, tangents
    │   ├── bevel.rs           # Beveled boxes, chamfered edges
    │   ├── extrusion.rs       # Extruded 2D polygon profiles
    │   ├── lathe.rs           # Revolved profiles, circular housings
    │   ├── tapered.rs         # Tapered and angular mechanical shapes
    │   ├── tube.rs            # Pipes, cables and curved tubes
    │   └── assembly.rs        # Component hierarchy, pivots, transforms
    │
    ├── materials/
    │   ├── mod.rs             # Shared material registry and caching
    │   └── industrial.rs      # Painted steel, rubber, alloys, glass
    │
    └── library/               # ONE FILE PER MODEL
        ├── mod.rs             # Model registry and exports
        ├── hound.rs           # Quadruped security robot
        ├── drone.rs           # Flying security drone
        ├── rifle.rs           # First-person assault rifle
        ├── container.rs       # Corrugated shipping container
        ├── crane.rs           # Quayside gantry crane
        ├── warehouse.rs       # Industrial warehouse
        ├── ship.rs            # Moored cargo vessel
        ├── boat.rs            # Extraction boat
        ├── relay.rs           # Communication relay terminal
        ├── barrel.rs          # Industrial barrel
        ├── fence.rs           # Security fencing
        └── props.rs           # Small environmental props
```

Preserve the existing public API where practical so `crates/game` and `crates/world` continue working.

Do not duplicate geometry utilities inside individual model files.

Models should express their construction through reusable geometry operations rather than manually calculating every vertex.

## 2. Geometry library

Implement reusable procedural mesh generation using Bevy's `Mesh` API.

Support these operations:

**Beveled box**
- Configurable dimensions.
- Configurable bevel radius and segment count.
- Flat or smooth shading where appropriate.
- Correct normals and UVs.

**Profile extrusion**
- Extrude a 2D polygon into a 3D solid.
- Generate front, back and side faces.
- Support angular armor panels and irregular cross-sections.

**Lathe**
- Revolve a profile around an axis.
- Support configurable radial segments.
- Suitable for joints, hubs, pistons and mechanical housings.

**Tapered geometry**
- Different dimensions at each end.
- Useful for angled armor, robot limbs and weapon components.

**Tubes**
- Generate cylindrical and curved tubes along a path.
- Suitable for hydraulic lines, cables and pipes.

**Assembly**
- Parent/child transformations.
- Named components.
- Explicit articulation pivots.
- Material assignments per component.

All meshes must have valid indices, normals and UVs. Generate tangents where required by normal-mapped materials.

Reuse generated meshes and materials where possible. Do not allocate identical meshes repeatedly.

Avoid unnecessary third-party geometry dependencies if the required operations can be implemented simply in Rust.

## 3. Rebuild the Hound

Use `assets/reference/hound.png` as the primary visual specification.

The current Hound is a blockout. Replace its geometry with a detailed mechanical assembly inspired by the supplied reference.

Prioritize:

1. Accurate overall silhouette and proportions.
2. Shaped armored chassis with beveled edges.
3. Four articulated mechanical legs.
4. Distinct upper and lower leg sections.
5. Circular hip and knee housings.
6. Pistons, hydraulic actuators and exposed linkages.
7. Layered armor panels with visible separation.
8. Sensor housings, upper equipment and antennae.
9. Mechanical feet with defined contact surfaces.
10. Secondary mechanical details and fasteners.

Use the new extrusion, bevel, lathe and assembly utilities.

Do not simply increase the number of scaled cubes.

Construct the legs using explicit joint hierarchies so hip and knee motion can be animated independently.

Preserve gameplay integration, enemy collision and AI behaviour.

## 4. Material improvements

Keep the existing PBR rendering pipeline.

Create appropriate materials for:

- Off-white painted armor.
- Dark graphite structural components.
- Exposed machined metal.
- Rubber and flexible components.
- Orange safety markings.
- Emissive optical sensors.

Materials should reinforce the mechanical design rather than conceal simple geometry with procedural noise.

Do not undertake a general lighting or post-processing rewrite.

## 5. Visual verification

Use the existing screenshot infrastructure.

Create or adapt an isolated Hound inspection view with:

- Side view.
- Front view.
- Three-quarter view.
- Rear view.

Capture screenshots at 1280×720 with consistent lighting and camera positions.

Compare the screenshots with `assets/reference/hound.png`.

Assess silhouette and proportions first, mechanical structure second, materials third.

If the geometry still looks predominantly like stacked boxes, continue refining it.

Do not declare the Hound complete solely because it renders successfully.

## 6. Testing

Preserve all existing gameplay functionality.

Use incremental debug compilation and the existing tests.

Verify:

- All five existing gameplay tests continue passing.
- The Hound spawns correctly in the game.
- Its animation remains functional.
- Its materials and geometry render correctly.
- The automatic mission replay still completes.
- No new runtime errors appear.

Add focused geometry tests for mesh validity and degenerate triangles.

Avoid release builds, unrelated refactoring and unnecessary infrastructure.

## 7. Scope and completion

This task has two deliverables:

**A. Structural refactor**

Each existing model has its own source file and uses the shared geometry/material APIs.

**B. High-quality Hound**

The Hound is rebuilt using the new geometry capabilities and demonstrates a substantial visual improvement against the supplied reference.

Do not rebuild all other models to the same fidelity during this task. They should be separated into files and remain functional.

Once the Hound is successful, the geometry library will be reused to improve the rifle, drone, containers and other assets in subsequent passes.

Work autonomously, inspect rendered results, correct defects and report the remaining visual differences.
