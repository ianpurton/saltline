# SALTLINE


![Saltline gameplay screenshot](.github/assets/saltline.png)

A native Linux procedural port FPS built with Bevy 0.20.

Run `cargo run -p saltline-game`. Enter deploys; WASD moves, mouse looks,
left mouse fires, right mouse aims, R reloads, Shift sprints, C crouches,
Space jumps, E deactivates nearby relays/extracts, Esc pauses.
Disable all three relays along the central dock road, then reach the boat.
Enter restarts after death or victory.

`cargo test -p saltline-game --test gameplay` runs renderer-free gameplay checks.
`./scripts/screenshot.sh` captures deterministic 1280×720 inspection views.
Generated captures and logs live under `tmp/` and are not approved baselines.

The dependency direction is game → world → models, with game also using models.
Models owns cached meshes/materials and procedural assemblies; world supplies the
port layout and obstacle bounds; game owns simulation, controls and presentation.

Current implementation uses axis-separated collision and sampled visibility checks.
Performance and reference-quality acceptance must be assessed on the rendered build;
no hardware performance claim is made without measurements.

Verification performed in this devcontainer:

- Workspace check and incremental debug build pass (Bevy 0.20.1).
- Five headless tests pass, including collision, damage/reload, detection/death,
  obstruction, and mission progression.
- The rendered action-driven mission replay destroyed 10 enemies and reached victory.
- Xvfb/xdotool exercised deployment, movement, look, aim, fire, reload and pause/resume.
- Port, rifle, Hound, drone and relay PNGs were captured and visually inspected.

`./scripts/window-smoke.sh` repeats window input checks. `./scripts/benchmark.sh`
runs the mission replay and reports median/p95 frame times. With a real `DISPLAY`
it uses the default adapter; without one it selects software Vulkan and Xvfb.
AMD Vulkan is available here, but presenting through Xvfb fails for that driver
because DRI3 is unavailable. Software rendering was used for functional captures.

Latest software Vulkan replay (optimized incremental debug profile): 268 frames,
median 124.44 ms, p95 156.84 ms, 30.84 simulation seconds, all 10 enemies destroyed,
victory asserted. These software timings were collected while captures also ran;
they are functional diagnostics, not a hardware acceptance benchmark.

The startup title screen uses the live procedural port with a separate looping
cinematic camera. Click **Deploy** (or press Enter) to start, **Settings** to toggle
camera motion with M, or **Controls** for the input legend. Esc closes subpanels.
The foreground title stacks are presentation only and are hidden during gameplay.

Run `scripts/title-screen.sh` to save two startup screenshots in
`tmp/screenshots/title-{70,170}.png`. This uses software Vulkan under Xvfb;
normal launches retain the selected hardware renderer. The Oswald font is embedded in the executable, so menu text does not depend on
the launch directory or an external assets folder. Its SIL Open Font License is
in `assets/fonts/OFL.txt`.

For GitHub CI, browser builds, releases, and Wavedash setup, see
[the release guide](docs/releases.md).
