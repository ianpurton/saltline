## 1. Game specification

### Setting

A compact industrial shipping port at dawn, during heavy rainfall.

The environment includes:

- Stacked shipping containers, warehouses and industrial buildings.
- Towering cargo cranes, floodlights, fencing and loading equipment.
- A docked cargo vessel and an extraction boat.
- Wet asphalt, puddles, reflective metal and concrete.
- Atmospheric fog, rain, industrial lighting and distant port scenery.

Create a believable environment with strong visual composition, environmental detail and a convincing sense of scale.

The level should be small enough to complete in approximately 5–10 minutes.

### Player

First-person soldier armed with a modern assault rifle.

Controls:

| Input | Action |
|---|---|
| WASD | Movement |
| Mouse | Look |
| Left mouse | Fire |
| Right mouse | Aim down sights |
| R | Reload |
| Shift | Sprint |
| C | Crouch |
| Space | Jump |
| E | Interact |
| Esc | Pause/menu |

Implement:

- Smooth first-person camera.
- Walking, sprinting, crouching and jumping.
- Weapon model visible in first-person.
- Weapon sway, recoil, aiming animation and muzzle flashes.
- Ammunition, magazine capacity and reloading.
- Player health, damage feedback and death.

### Enemies

Implement two enemy classes.

**Security drone**
- Flying autonomous surveillance/combat drone.
- Patrols predefined areas.
- Detects and tracks the player.
- Fires projectiles or energy weapons.
- Takes damage and explodes when destroyed.

**Hound**
- Four-legged robotic security unit.
- Procedurally modeled with articulated legs.
- Patrols container corridors.
- Detects and pursues the player.
- Attacks at close range.
- Has animated walking, running, attacking and death states.

Implement detection, pursuit, obstacle avoidance, damage, death and combat feedback.

### Mission

The automated port security system has become hostile.

The player's objectives are:

1. Enter the industrial port.
2. Locate and deactivate three security communication relays.
3. Survive encounters with robotic security forces.
4. Reach the extraction boat.
5. Complete the mission and display a victory screen.

Relays should be recognizable industrial terminals with lights, antennas and animated status indicators.

Press E within interaction range and with unobstructed line of sight to deactivate a relay. Repeated interaction is harmless. Extraction requires all three relays to be deactivated.

The HUD displays objective progress, ammunition, health and navigation information.

Include main menu, pause, death/restart and victory states.

Pause freezes gameplay while menus remain responsive. Resume clears stale input. Restart resets player health and ammunition, enemies, relays, projectiles, effects and mission progress.

## 2. Graphics and visual quality

Prioritize visual quality in this order:

1. Lighting, composition and atmosphere.
2. Detailed, recognizable procedural models.
3. Physically based materials and believable surfaces.
4. Wet ground, reflections and puddles.
5. Atmospheric fog, rain and particles.
6. Weapon and enemy animations.
7. Combat effects and UI polish.

Inspect every image in `assets/reference/` before implementation. Use these images as the visual quality target:

- `port.png`: layered industrial composition, crane silhouettes, container density, warm dawn, cool shadows, water and atmospheric depth.
- `rifle.png`: proportions, receiver, magazine, rails, sights, barrel and stock detail.
- `hound.png`: articulated silhouette, joints, feet, armor panels, sensors and contrasting materials.
- `warehouse.png`: structural depth, roof framing, corrugated surfaces, storage, practical lighting and surface wear.

Match their level of detail and material variation at gameplay viewing distances. Adapt lighting to heavy rain at dawn while preserving readability.

Target 60 FPS on hardware rendering. Record adapter, build profile and frame timings along a fixed gameplay route. Use software rendering for functional verification and captures.

## 3. Required source tree

Use this structure as a guide. Create additional modules only when their responsibilities justify them.

```text
.
├── .devcontainer/
│   ├── devcontainer.json        # Existing Linux development environment
│   └── Dockerfile               # Rust, Vulkan, Mesa, Xvfb, xdotool
│
├── Cargo.toml                   # Cargo workspace
├── Cargo.lock                   # Dependency lockfile
├── README.md                    # Build, controls, testing, architecture
│
├── crates/
│   │
│   ├── game/                    # BEVY APPLICATION + GAMEPLAY
│   │   ├── Cargo.toml
│   │   ├── tests/
│   │   │   ├── combat.rs        # Hit detection and damage
│   │   │   ├── enemies.rs       # AI behaviour
│   │   │   ├── objectives.rs    # Mission completion and state transitions
│   │   │   └── navigation.rs    # Movement and collision
│   │   └── src/
│   │       ├── lib.rs           # Reusable gameplay plugins
│   │       ├── main.rs          # App setup, plugins, startup
│   │       ├── states.rs        # Menu, Playing, Paused, Dead, Victory
│   │       ├── player.rs        # FPS movement, camera, health
│   │       ├── input.rs         # Keyboard/mouse input
│   │       ├── weapons.rs       # Rifle, recoil, ADS, reload
│   │       ├── combat.rs        # Damage, projectiles, hit detection
│   │       ├── enemies.rs       # Drone/Hound AI and behaviour
│   │       ├── navigation.rs    # Patrol, pursuit, obstacle avoidance
│   │       ├── physics.rs       # Movement and collision
│   │       ├── objectives.rs    # Three relays and extraction
│   │       ├── ui.rs            # Health, ammo, compass, objective HUD
│   │       └── audio.rs         # Sound playback and spatial audio
│   │
│   ├── models/                  # PROCEDURAL 3D MODEL LIBRARY
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs           # Model registry, generation, seeds
│   │       ├── geometry.rs      # Mesh builders and primitives
│   │       ├── materials.rs     # Bevy PBR materials
│   │       ├── animation.rs     # Procedural rigs and animation
│   │       ├── export.rs        # Optional GLB exporter
│   │       └── library/
│   │           ├── mod.rs
│   │           ├── *rifle.rs    # Each model in its own file
│   │           └── props.rs     # Pipes, crates, lights, cables
│   │
│   └── world/                   # LEVEL + ENVIRONMENT GENERATION
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── level.rs         # Deterministic port layout
│           ├── terrain.rs       # Asphalt, concrete and puddles
│           ├── lighting.rs      # Sun, floodlights and practical lights
│           ├── atmosphere.rs    # Fog, sky and rain
│           ├── effects.rs       # Smoke, sparks, explosions
│           └── spawning.rs      # World object and enemy placement
│
├── assets/                      # FILE-BASED RESOURCES
│   ├── shaders/
│   │   ├── wet_ground.wgsl      # Wet asphalt material
│   │   ├── water.wgsl           # Water animation
│   │   ├── rain.wgsl            # Rain visual effect if needed
│   │   └── hologram.wgsl        # Relay display material
│   │
│   ├── audio/
│   │   ├── rifle.wav            # Gunshot
│   │   ├── reload.wav           # Reload sound
│   │   ├── rain.ogg             # Rain ambience
│   │   ├── drone.wav            # Drone engine
│   │   └── explosion.wav        # Explosion
│   │
│   ├── fonts/
│   │   └── hud.ttf              # UI font
│   │
│   └── reference/
│       └── *.png                # Art direction references
│
├── tests/
│   └── baselines/
│       ├── port.png             # Approved scene screenshot
│       ├── rifle.png            # Approved weapon screenshot
│       └── hound.png            # Approved robot screenshot
│
├── scripts/
│   ├── smoke-test.sh            # Launch and exercise game
│   ├── screenshot.sh            # Capture deterministic viewpoints
│   └── benchmark.sh             # Performance test
│
└── tmp/                         # GENERATED, GIT-IGNORED
    ├── screenshots/
    │   ├── port.png             # Current level capture
    │   ├── rifle.png            # Weapon inspection
    │   └── hound.png            # Enemy inspection
    ├── exports/
    │   └── hound.glb            # Optional Blender-compatible export
    └── logs/
        └── game.log             # Runtime diagnostics
```

Files listed under `assets/` are examples, not mandatory empty placeholders. Create only resources actually used by the game.

The Rust code is the authoritative source of geometry and animations.

Use dependencies `game → world → models` and `game → models`. Models owns geometry, materials and visual rigs; world owns layout and environment; game owns gameplay and AI. World generation exposes player, enemy, relay and extraction placements; game attaches gameplay behavior.

## 4. Efficient implementation strategy

**Minimize total inference cost and elapsed development time without sacrificing the final result.**

Follow these principles:

- Inspect the existing devcontainer and installed tools before changing the environment.
- Declare Bevy `0.20` in workspace Cargo dependencies and commit `Cargo.lock`. Use the installed Bevy skills.
- Use existing Bevy rendering features, ECS systems and standard Rust crates.
- Prefer simple, proven implementations over custom frameworks.
- Reuse geometry builders, material definitions and procedural components.
- Use deterministic random seeds for level and model generation.
- Keep implementations modular to reduce repeated context consumption.
- Do not repeatedly read unrelated files.
- Do not generate lengthy progress reports.
- Avoid unnecessary dependencies, installations and rebuilds.
- Use `cargo check` for compilation feedback when sufficient.
- Use incremental debug builds for executable testing.
- Do not run `cargo build --release` during development.
- Do not spend time implementing platform support beyond native Linux.
- Do not create a custom editor, asset pipeline or engine abstraction.
- Do not implement optional features before the complete gameplay loop works.

Build reusable geometric primitives and material helpers once, then reuse them throughout the environment.

Spend the available effort on game quality, not infrastructure.

## 5. Autonomous testing

Follow the installed `bevy-testing` skill.

### Gameplay verification

Write lightweight Rust tests in `crates/game/tests/` using minimal Bevy `App` instances, controlled time and inputs, fixed seeds, scheduled `App::update()` calls and observable state assertions. Bound asynchronous progress checks by a step limit or deadline.

Use deterministic simulation steps to verify:

- Player movement and collision.
- Weapon firing, damage and reloading.
- Enemy detection, pursuit and attacks.
- Relay interaction and mission progression.
- Death, restart and victory conditions.
- Pause/resume and complete mission reset.
- Input handling across zero, one and multiple fixed ticks per frame.

Avoid launching the renderer for tests that only require ECS state.

### Visual inspection

Implement a simple inspection mode within the game. Do not build a separate editor.

Support deterministic camera views of:

- The full port.
- The first-person rifle.
- The Hound robot.
- The security drone.
- The relay terminal.
- Representative materials.

Capture 1280×720 PNG images under `tmp/screenshots/` using Bevy screenshot capture. Wait for capture completion before inspecting files.

Inspect screenshots and correct visible problems, including:

- Missing or intersecting geometry.
- Incorrect proportions.
- Poor lighting and contrast.
- Unconvincing materials.
- Floating or misplaced objects.
- Unreadable UI.
- Incorrect camera composition.

Use fixed camera locations and random seeds for repeatable inspection.

Compare inspection and player-height gameplay views against the references. Fix the three largest visible shortcomings and repeat until the visual target is met. Inspect successive frames of walking, attacking, recoil and reloading. Update regression baselines through explicit review.

### Gameplay smoke testing

Use Xvfb and xdotool to verify launch, menus, mouse capture, movement, aiming, firing, reloading and pause/resume.

Run a deterministic full-mission driver through the same movement, combat and interaction actions used by the player, under normal gameplay rules. Assert victory; fail on timeout or unmet assertions and retain diagnostic logs.

Verify that the player can:

- Move and aim.
- Fire and reload.
- Damage and destroy enemies.
- Deactivate all three relays.
- Reach extraction.
- Complete the mission.

Inspect runtime logs for errors and panics.

## 6. Development sequence

Use the following order to reduce rework.

**Phase 1 — Playable foundation**

Implement the Bevy application, first-person movement, rifle mechanics, enemy behaviour, relays and mission progression using simple placeholder geometry.

Verify the complete mission before visual refinement.

**Phase 2 — Procedural assets**

Replace placeholders with detailed procedural geometry.

Implement the rifle, drones, Hounds, containers, buildings, cranes, ship and environmental props.

Validate model geometry and inspect screenshots.

**Phase 3 — Visual presentation**

Develop lighting, materials, wet surfaces, atmosphere, rain and combat effects.

Work from large-scale composition to fine detail. Inspect actual rendered screenshots rather than judging only from source code.

**Phase 4 — Polish and verification**

Improve animation, weapon feedback, enemy behaviour, audio, interface and visual coherence.

Run gameplay tests and complete a full automated mission playthrough.

## 7. Completion criteria

The project is complete when:

- The native Linux game launches reliably.
- The player can complete the entire mission.
- Both enemy types behave correctly.
- All three relays can be deactivated.
- Extraction triggers the victory screen.
- The world contains recognizable, detailed industrial assets.
- Lighting, atmosphere, wet surfaces and effects create a convincing cinematic presentation.
- Gameplay tests pass.
- A complete automated gameplay smoke test succeeds.
- Representative screenshots have been captured and visually inspected.
- Visual quality matches the reference images in composition, model detail and material variation.
- Weapon and enemy animations have been visually verified.

The game must be more than a technical prototype. It should feel like a coherent, polished, short FPS experience.

**Do not stop at the first working version.** Continue visual refinement and functional verification until the stated acceptance criteria are met, while avoiding unnecessary engineering work.
