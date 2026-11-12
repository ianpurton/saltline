# Rendering and performance

SALTLINE targets a 1920×1080 native Linux output at 60 FPS. The 3D view can scale
between 70% and 100%; menus and HUD stay at output resolution. This target is a
measurement requirement, not a claim about every GPU.

## Run and compare

```sh
cargo run --release -p saltline-game
./scripts/benchmark.sh
./scripts/benchmark.sh --render-scale 0.8
./scripts/screenshot.sh
```

The benchmark runs three times, requires a hardware adapter, and fails if any
measured route segment exceeds 16.7 ms p95. It renders offscreen through Vulkan;
DISPLAY and Xvfb are unnecessary. It never silently substitutes software Vulkan.
Each scenic segment settles for 60 frames and measures 120 frames. The full
mission replay follows, exercising combat, all relays and extraction. Shader
warmup and startup are reported separately. Avoid builds, captures and other GPU
work during timing runs. Logs live in `tmp/logs/benchmark-{1,2,3}.log`.

Benchmarks hold render scale fixed unless `--adaptive` is supplied. A fixed scale
makes comparisons reproducible; use the same options for all compared runs.
Report the scale together with output resolution. Normal play starts at 100%,
reduces scale after two seconds with a rolling p95 above budget, and recovers only after five seconds
of headroom. `--render-scale` disables that adaptation.

## Quality controls

| Option | Default | Purpose |
| --- | --- | --- |
| `--quality medium\|high` | medium | Shared title and gameplay profile |
| `--resolution WIDTHxHEIGHT` | 1920x1080 | Output resolution |
| `--render-scale 0.7…1.0` | adaptive in play | Fixed 3D resolution multiplier |
| `--ssr on\|off` | high only | Screen-space reflections |
| `--ao on\|off` | high only | Screen-space ambient occlusion |
| `--contact-shadows on\|off` | high only | Contact shadow pass |
| `--volumetric on\|off` | high only | Volumetric fog |
| `--shadow-size 512\|1024\|2048\|4096` | 2048 medium, 4096 high | Shadow map resolution |
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

Named system CPU samples are bounded to their most recent 120 updates.
The reported main-world CPU metric covers the main app schedules, not the render
thread. GPU diagnostics depend on adapter support. Draw submission counts describe
opaque multidraw batches plus transparent items before GPU culling, excluding
shadow and UI passes; they are not a count of executed GPU indirect draws.

## Visual inspection

Capture views include deployment, corridor, waterfront, extraction, rifle, ADS,
reload, damage, enemies, relays, warehouse and title. They use a fixed presentation
clock and wait for pipeline readiness. Captures go to `tmp/screenshots/` and are
inspection artifacts, not automatically approved baselines. To explicitly request
software captures, set `SALTLINE_SOFTWARE=1`.

Procedural sound playback requires an audio device. Headless runs disable audio;
waveform generation and state handling can be checked without one, but listening
still needs a device.
