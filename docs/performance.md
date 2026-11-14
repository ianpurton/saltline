# Rendering and performance

SALTLINE targets a 1920×1080 native Linux output at 60 FPS. The 3D view can scale
between 50% and 100%; menus and HUD stay at output resolution. This target is a
measurement requirement, not a claim about every GPU.

## Run and compare

```sh
cargo run --release -p saltline-game
./scripts/benchmark.sh
./scripts/benchmark.sh --render-scale 0.8
./scripts/benchmark.sh --compare-overhead
./scripts/screenshot.sh
```

The benchmark runs three times, requires a hardware adapter, and fails if any
measured route segment exceeds 16.7 ms p95. It renders offscreen through Vulkan;
DISPLAY and Xvfb are unnecessary. It never silently substitutes software Vulkan.
Each scenic segment settles for 60 frames and measures 120 frames. The full
mission replay follows, exercising combat, all relays and extraction. Shader
warmup and startup are reported separately. Avoid builds, captures and other GPU
work during timing runs. Logs live in `tmp/logs/benchmark-{1,2,3}.log`; versioned JSON artifacts beside them
retain raw frame samples, metric observations, distributions, stage boundaries,
adapter/backend, quality settings, and instrumentation features. Missing stages,
invalid samples, incomplete missions, non-release builds, missing instrumentation,
software adapters, or absent adapter evidence
fail acceptance. Each scenic stage must contain exactly 120 measured samples.
The 16.7 ms p95 gate applies independently to all five stages in every run.
GPU query availability is reported separately from this frame-time gate.

Benchmarks hold render scale fixed unless `--adaptive` is supplied. A fixed scale
makes comparisons reproducible; use the same options for all compared runs.
Report the scale together with output resolution. Normal play starts at 100%,
reduces scale after two seconds with a rolling p95 above budget, and recovers only after five seconds
of headroom. `--render-scale` disables that adaptation.

## Quality controls

| Option | Default | Purpose |
| --- | --- | --- |
| `--quality low\|medium\|high` | medium | Shared title and gameplay profile |
| `--resolution WIDTHxHEIGHT` | 1920x1080 | Output resolution |
| `--render-scale 0.5…1.0` | adaptive in play | Fixed 3D resolution multiplier |
| `--ssr on\|off` | high only | Screen-space reflections |
| `--ao on\|off` | high only | Screen-space ambient occlusion |
| `--contact-shadows on\|off` | high only | Contact shadow pass |
| `--volumetric on\|off` | high only | Volumetric fog |
| `--shadow-size 0\|512\|1024\|2048\|4096` | 2048 medium, 4096 high | Shadow map resolution |
| `--taa on\|off` | on | Temporal antialiasing |
| `--bloom on\|off` | on | Restrained highlight bloom |
| `--profile` | off | CPU/GPU render-pass diagnostics |
| `--no-audio` | off | Disable audio output |

Medium retains deferred wet materials, environment reflections, distance fog,
TAA and bloom, with two shadow cascades. High uses four cascades and enables the
additional effects above. Browser rendering retains the simpler forward LDR
path; native-only screen-space effects and finishing are excluded.

## Implementation

- Static containers are assembled by material and cached by paint variant.
  Object boundaries remain independent for culling. Distant containers use a
  coarse mesh with hysteresis; shared paint maps retain corrugation and rust.
- Small fittings use simpler bevels. Large foreground rifle and Hound armor
  retain the detailed mesh. Articulated joints remain independent.
- The static sky is filtered during warmup. Both cameras then share the cached
  diffuse/specular textures; realtime filtering is removed. The panorama sun and
  directional light use the same direction.
- Rain uses one shader-driven mesh. Its clock freezes during pause; roof masking
  and camera-facing streaks run on the GPU.
- Combat cosmetics reuse 256 particles. Audio has a 24-voice transient cap and
  separate ambience levels. The deterministic procedural WAV palette is embedded;
  regenerate it with `python3 scripts/synthesize-audio.py`.
- Native finishing runs after Bevy tone mapping and before UI: split tone,
  contrast, lifted blacks, grain, vignette and damage response. It does not apply
  a second tone mapper or sRGB conversion.

## Diagnostics and tracing

Native Linux play includes Bevy 0.20's built-in render debug overlay in both
development and release builds. Run `cargo run --release -p saltline-game`; no
extra Cargo features or debug build are required. The existing `render_dev_tools`
Bevy feature includes `RenderDebugOverlayPlugin` through `DefaultPlugins`.

Press **F1** to show the overlay and cycle supported render-buffer modes (depth,
normals, motion vectors and deferred material buffers); cycling past the last
supported mode hides it. Press **F2** to cycle opacity. Available modes depend on
the camera's existing passes; the overlay does not enable additional passes.
FPS is included in the timing panel described below. The buffer overlay starts hidden
and its plugin is disabled for WebAssembly, non-Linux targets, `--headless-render`,
`--benchmark`, `--inspect`, and `--title-capture` runs.

Native Linux play also shows Bevy's built-in `DiagnosticsOverlay` below the mission
HUD automatically, including in release builds. It displays FPS, frame time and CPU/GPU
timings in milliseconds for the deferred prepass, shadows, deferred lighting,
opaque geometry, TAA, and Saltline finishing pass. No `--profile` flag is needed.
The panel refreshes once per second. Other platforms retain the separate FPS counter.
`Missing` or `No sample` means the pass/query is unavailable or has not produced a
sample yet, rather than zero cost. GPU query support depends on the adapter/backend.
CPU timings measure render command preparation, not GPU execution; these selected
passes are not a total frame-time breakdown. The timing panel and its automatic
render diagnostics use the same platform, benchmark, and capture exclusions above.

`PerformancePlugin` in `saltline_game::performance` owns registration. `--profile`
and `--benchmark` enable the seven existing system measurements, built-in frame-time
and entity-count metrics, render diagnostics, and submission counting. Ordinary play
keeps custom system instrumentation disabled; native Linux play enables Bevy render
diagnostics for the timing panel. Adaptive resolution retains its original input.

| Path | Type, unit, and sampling |
| --- | --- |
| `saltline/system/{module}/{operation}_ms` | Duration in ms per invocation, including early returns; 120-sample display history |
| `saltline/main_world_ms` | Duration in ms from the First timing marker to the Last marker; excludes render-thread work and later reporting |
| `saltline/render/submissions` | Gauge, submissions; latest asynchronous render-world count sampled once per main update |
| `render/.../elapsed_cpu`, `render/.../elapsed_gpu` | Render-span durations in ms, supplied asynchronously by Bevy |
| Other built-in diagnostics | Bevy-defined units and histories; exported with their registered suffix |

System paths cover `lib/simulate`, `main/controls`, `main/visuals`, `hud/update`,
`presentation/rigs`, `rain/update`, and `effects/combat_effects`. The former public
`SystemTimings`/`CpuSpan` API has been replaced by `PerformanceSettings` and
`CpuMeasurement`; dashboards and exporters read `DiagnosticsStore`.

The benchmark keeps full-run distributions separately from bounded display histories.
It collects each measurement timestamp once after deferred diagnostics are applied,
and writes the artifact from Last after mission completion. Frame deltas are attributed
to the previous rendered view. Startup and each stage's settling interval are excluded.
CPU markers exclude startup/settling during benchmarks. Their scope differs from total
frame time: compare them as supporting evidence, not interchangeable timings.

Artifacts contain measurement and observation timestamps relative to benchmark startup.
Render observations use the stage at observation time; asynchronous query results near
stage boundaries are not claimed to match an exact main-world frame. Render diagnostics
still in flight at shutdown are not awaited. Unsupported GPU measurements remain absent;
availability never converts missing GPU cost into zero. Backend/device support determines
query availability. The custom finishing-pass span remains included.

Frame reports include median, p95, p99, maximum, and the worst rolling interval covering
at least one second of accumulated frame time (or the whole stage if shorter). Percentiles
use sorted index `floor((n - 1) * percentile / 100)`; median uses index `n / 2`.
System and render durations have separate distributions. Sustained frame-time analysis
should use each stage's chronological `raw_ms`, not the combined `all` log row.

Draw submission counts describe opaque multidraw batches plus transparent items before
GPU culling, excluding shadow and UI passes; they do not count executed indirect draws.

Coarse tracing spans use the original operation names. Enable one backend at a time:

```sh
cargo run --release -p saltline-game --features trace-chrome -- --profile
cargo run --release -p saltline-game --features trace-tracy -- --profile
```

Chrome tracing writes Bevy's trace artifact for a compatible trace viewer; Tracy needs
a connected Tracy profiler. For source-level profiling, set
`CARGO_PROFILE_RELEASE_DEBUG=1` and record that build setting with the capture. Capture
the shipping resolution, renderer, content, and power mode after warming up. Detailed
traces add overhead; use normal feature builds for acceptance. No standalone `tracing`
dependency is needed: Bevy re-exports it.

`./scripts/benchmark.sh --compare-overhead` alternates three instrumented runs with
three `--no-instrumentation` baselines at identical settings. It writes paired stage
p95 differences and their median to `tmp/logs/benchmark-overhead.json`. Baselines retain
the route's frame/CPU clocks and artifact export, while disabling system measurements,
render queries, built-in diagnostic plugins, and submission counting. Thermal/system
noise can produce negative differences; no overhead threshold is assumed. Acceptance
still evaluates the instrumented runs against the unchanged frame-time budget.

For a single run, use `--benchmark-output PATH` (default `tmp/logs/benchmark.json`).
Validate saved artifacts with `python3 scripts/check-benchmark.py PATH...`.
Native Linux is the acceptance platform; browser compilation remains supported, but
these results establish no browser or Steam Deck performance budget.

## Visual inspection

Capture views include deployment, corridor, waterfront, extraction, rifle, ADS,
reload, damage, enemies, relays, warehouse and title. They use a fixed presentation
clock and wait for pipeline readiness. Captures go to `tmp/screenshots/` and are
inspection artifacts, not automatically approved baselines. To explicitly request
software captures, set `SALTLINE_SOFTWARE=1`.

Procedural sound playback requires an audio device. Headless runs disable audio;
waveform generation and state handling can be checked without one, but listening
still needs a device.

## Graphics menu (native Linux)

Select **Graphics** on the home screen (or press **G**). During play, press
**Esc**, select **Graphics** (or press **G**), and adjust settings while the
scene continues rendering behind the paused game. Use the mouse arrows or
**Up/Down** to select and **Left/Right** to change a value. **Enter/Space** activates
the selected control; **Esc/Back** returns to the home screen or pause page. Choose Resume or
press Esc again to return to play. The timing panel stays visible to the left.

Changes apply immediately and save automatically. Low, Medium, and High presets
provide starting points; individual adjustments select Custom. Reset Defaults
restores Medium. Settings include render scale (50–100%), adaptive resolution,
shadows (Off, 512, 1024, 2048, 4096), reflections, ambient occlusion, contact shadows,
volumetric fog, bloom, TAA, rain, wet-surface shading and container detail.
Manual scale adjustment is inactive while adaptation is enabled; its row shows the
current scale. Adaptation can now reduce native Linux rendering to 50%.

| Setting | Low | Medium / default | High |
| --- | --- | --- | --- |
| Initial scale | 70% | 100% | 100% |
| Adaptive resolution | On | On | On |
| Shadow size | 512 | 2048 | 4096 |
| Reflections, AO, contact shadows, volumetric fog | Off | Off | On |
| Bloom, TAA, rain | On | On | On |
| Wet surfaces | Simple | Detailed | Detailed |
| Container detail | Low | Standard | Standard |

Simple wet surfaces retain basic PBR shading without procedural wet noise/waves.
Low container detail uses existing coarse container meshes at all distances;
Standard restores distance-based detail. Neither changes collisions or mission
logic. Changes can briefly compile shaders; compare timings after they settle.
These options do not guarantee a particular frame rate.

Preferences use versioned JSON at `$XDG_CONFIG_HOME/saltline/graphics.json`, or
`$HOME/.config/saltline/graphics.json` when XDG_CONFIG_HOME is unset. Invalid or
unsupported preferences fall back to defaults with a log warning. Save failures
leave changes active and show a message in the menu. Precedence is defaults,
saved preferences, then explicit command-line options. `--quality low|medium|high`
selects a preset before individual overrides. Benchmarks, mission tests, offscreen
runs and automated captures ignore saved preferences and do not create the menu;
their defaults and CLI options determine quality. Browser play retains its existing UI.

The game now renders the [original berth layout](map.md). Existing benchmark stage
identifiers are retained, but their viewpoints and world geometry have changed;
compare timings only against builds of the same map. The `corridor` stage now
samples the authored container yard. The renderer and 16.7 ms acceptance threshold
are unchanged.
