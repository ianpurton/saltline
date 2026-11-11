The game is functionally complete, but its graphics are substantially below the Saltline reference.

Do not add gameplay features or redesign the architecture.

Perform a visual-quality improvement pass using the reference images in `assets/reference/`.

First, capture screenshots from the existing game at viewpoints comparable to the references. Inspect them and identify the largest visual differences.

Prioritize:

1. Replace flat-looking container, asphalt, concrete and metal surfaces with detailed procedural PBR materials, including normal and roughness variation.
2. Improve lighting with more convincing shadows, ambient illumination, contrast and color grading.
3. Implement convincing wet-ground appearance, including puddles, surface variation and reflections where technically feasible.
4. Add geometry detail to containers, cranes, drones, Hounds and industrial props.
5. Improve atmospheric depth with layered fog, rain and environmental effects.
6. Refine the overall composition, avoiding large empty or uniformly lit surfaces.

Use Bevy's rendering features and WGSL shaders where necessary. Do not implement an entirely new renderer.

After each major change, capture and inspect an actual screenshot. Compare it against the Saltline references and correct the most obvious remaining differences.

Do not declare completion merely because the code compiles or the scene renders. Stop only when visual improvements have been demonstrated with before-and-after screenshots, and explain any remaining differences from the reference.
