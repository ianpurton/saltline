# Saltline — Complete Visual Fidelity Pass

Improve the existing Rust/Bevy 0.20 Saltline game to reproduce the visual quality of the original Anthropic Saltline demonstration.

This is an existing, functional game. **Do not start a new project or rewrite the gameplay.**

Read `README.md`, `VISUAL_REVIEW.md` and inspect the existing implementation before making changes.

Use the reference screenshots in `assets/reference/` as the visual target.

## Objective

The current game is playable, but still looks like a stylized prototype rather than the detailed industrial FPS shown in the reference images.

Your task is to improve **every major visual subsystem**, not merely one model.

Work autonomously through multiple visual improvement iterations. Do not finish after one implementation pass if substantial reference differences remain.

## 1. Models and geometry

Inspect all existing models under `crates/models/src/library/`.

Improve the following:

- **Rifle:** High-detail weapon silhouette, layered receiver, barrel assembly, rail system, sights, stock, magazine, mechanical controls and material variation.
- **Hound:** Refine existing geometry, articulation, armor, sensors, mechanical joints and secondary details.
- **Drone:** Detailed chassis, propulsion assemblies, sensors, articulated components and mechanical surfaces.
- **Containers:** Accurate corrugation, door hardware, locking bars, corner castings, hinges, markings and weathering.
- **Cranes:** Structural trusses, beams, cables, hoists, trolleys, platforms and industrial details.
- **Warehouse:** Enclosed industrial architecture with detailed entrances, supports, windows, roof structures and interior dressing.
- **Cargo ship:** Recognizable hull, deck, superstructure, equipment and containers.
- **Relays:** Detailed industrial electronics, antennas, panels, lights and interactive status indicators.
- **Environmental props:** Pipes, bollards, pallets, fencing, machinery, cables, signs and loading equipment.

Use the existing reusable procedural geometry toolkit.

Prefer shaped, beveled, extruded and articulated geometry over basic scaled cubes.

Keep each model in its own Rust source file.

## 2. Materials

Improve the procedural PBR materials throughout the game.

Different surfaces must have distinct physical properties and visual characteristics.

Implement convincing painted steel, exposed metal, rust, concrete, wet asphalt, rubber, glass and industrial plastic.

Use appropriate albedo, normal, metallic and roughness variation.

Avoid uniform procedural noise across unrelated material types.

Add markings, decals and industrial identification details where they substantially improve realism.

## 3. Environment

Improve the port's density, composition and industrial authenticity.

Focus on:

- Varied container arrangements.
- More detailed industrial buildings.
- Believable crane and ship geometry.
- Loading equipment and scattered industrial objects.
- Road markings, drains and damaged surfaces.
- Enclosed and partially enclosed spaces.
- Strong foreground, midground and background composition.

Preserve gameplay paths, collision and mission objectives.

## 4. Rendering and atmosphere

Use Bevy's existing rendering capabilities before introducing new rendering systems.

Improve:

- Physically convincing directional and local lighting.
- Shadow quality and contact shadows.
- Wet surfaces and puddle reflections.
- Atmospheric depth and distance fog.
- Rain, surface splashes and water effects.
- Sky appearance and cloud coverage.
- HDR, tone mapping, exposure and color grading.
- Combat effects, sparks, muzzle flashes and explosions.

Match the cinematic appearance of the supplied reference images.

Do not introduce expensive rendering features without measuring their impact.

## 5. Visual inspection and iteration

Use the existing screenshot infrastructure.

Capture the port, warehouse, rifle, Hound, drone and relay from deterministic camera positions.

Compare each capture with the corresponding file under `assets/reference/`.

For each subsystem:

1. Identify its three most significant visual differences.
2. Implement improvements addressing those differences.
3. Capture a new screenshot.
4. Inspect the rendered result.
5. Correct obvious remaining defects.

Repeat where necessary.

Prioritize changes by their effect on the final game's appearance, not by ease of implementation.

Do not claim visual acceptance merely because a model has more geometry or a material has more texture variation.

## 6. Efficiency

Use incremental debug builds and existing project tools.

Avoid:

- Rewriting working gameplay.
- Repeated full builds when smaller checks suffice.
- Installing unnecessary dependencies.
- Creating new general-purpose frameworks.
- Reimplementing features Bevy already provides.
- Generating unused models, shaders or resources.
- Repeatedly inspecting unrelated files.

Reuse procedural geometry and material helpers.

Preserve existing game architecture and testing infrastructure.

The objective is high visual fidelity at low inference and execution cost—not the shortest possible implementation.

## 7. Verification

Run the existing headless gameplay tests.

Run the automated mission replay after major changes.

Check that procedural geometry, animation and collisions remain functional.

Capture actual screenshots of the finished game.

Record remaining visual differences honestly.

## 8. Completion

Do not stop after the first successful build, first screenshot or first improvement cycle.

Continue until all major visual systems have received a substantial fidelity pass and the game is recognizably closer to the supplied Saltline references.

Prioritize the most visible deficiencies before minor details.

If significant differences remain, continue addressing the highest-impact ones rather than simply documenting them.

At completion, provide:

- Before-and-after screenshots.
- A concise summary of visual improvements.
- Gameplay test results.
- Remaining differences from the reference.
- Total execution time and token usage, if available.
