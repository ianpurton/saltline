# Cinematic postprocessing

The title and gameplay cameras share the reference-inspired pipeline. Reflection
cameras and UI are excluded. Native uses scene-linear HDR, then a 75% ACES / 25%
AgX curve. WebGL2 retains material tone mapping and applies bounded LDR brightness
adaptation, so its highlight recovery cannot match the native HDR path.

The fragment-only passes meter the unmodified scene, build a reduced-resolution
blur, optionally generate horizontal streaks, composite lens effects and ADS
blur, grade the image, and finish with edge-aware sharpening, grain and dithering.
WebGL2 also applies FXAA. Native retains TAA. Bloom is owned by this pipeline;
do not add a second Bevy Bloom component to these cameras.

Native sun visibility samples reversed-Z prepass depth. WebGL2 uses the authored
world collision boxes, which approximate visual occlusion and exclude decorative
objects without colliders. Visibility and exposure have separate temporal smoothing.
Exposure histories are per camera and reset on activation, mission reset, resize,
quality-resolution changes, and re-enabling adaptation.

The native graphics menu exposes eye adaptation, lens effects, ADS blur,
chromatic aberration, film grain and sharpening. Existing preference files gain
defaults automatically. For deterministic native inspection and benchmarking:

```
--adaptation on|off --lens on|off --ads-blur on|off
--aberration on|off --grain on|off --sharpen on|off --bloom on|off
```

All controls are also represented in the shared `RenderQuality.finish` settings
used by the browser build. Its existing UI does not expose a graphics menu.
High quality uses half-resolution blur/streak targets; medium and low use quarter
resolution. Low disables lens dirt, streaks and flare by default. Disabled auxiliary
effects skip their passes; the dirt map is generated once when first needed.

GPU diagnostic spans are `cinematic_blur`, `cinematic_meter`, `cinematic_dirt`,
`cinematic_streak`, `cinematic_composite`, and `cinematic_finish`. The blur span has
two invocations per rendered camera. Capture and benchmark artifacts under `tmp/`
are validation evidence, not approved image baselines.

## Validation (2026-11-17)

- Native build and WebGL2 WASM build pass; 30 game binary unit tests pass.
- Radeon 780M Vulkan captures cover medium ADS, low damage at 4:3, high waterfront,
  and the same waterfront with optional effects disabled. No GPU validation errors.
- Chromium WebGL2 renders the title screen with the finishing pipeline and UI.
  Software rendering is slow; browser frame rate is not a hardware performance result.
- Native medium 1280×720 development benchmark: median frame 10.320 ms, p95
  23.609 ms. GPU medians: blur 0.112 ms per recorded invocation, metering 0.014 ms,
  streak 0.023 ms, composite 0.213 ms, finish 0.210 ms. This is not a release budget.
- The workspace suite reaches an existing failure in `firing_damage_reload`;
  the simulation library, world code, and integration test are unchanged from HEAD.
  Five other gameplay integration tests pass. Full preset performance and physical
  browser-device coverage remain unverified.
