#import "theme.typ": *
#let d = json("data/reference.json")
#let o = d.objects
#let asset-spec = json("assets/thumbnails/specification.json")
#let thumbnails = json("assets/thumbnails/manifest.json")
#let props = thumbnails.filter(e => e.id != "hound" and e.id != "gull" and e.id != "pac9")
#let asset-card(entry) = block(stroke: .3pt + rgb("d3cec4"), inset: 3pt, width: 100%)[
 #image("assets/thumbnails/" + entry.filename, width: 100%, height: 45mm, fit: "contain")
 #text(size: 8.5pt, weight: "bold", entry.label)
 #linebreak()
 #text(size: 7pt)[#entry.size.map(x => str(calc.round(x, digits: 2))).join(" × ") m / nominal]
 #linebreak()
 #text(size: 6.5pt, fill: blue, entry.source)
]
#show: manual

#sheet("01", "Build the berth", "Purpose, evidence and mission contract", [
#note[*SALTLINE* is a single-player industrial-port FPS. Cut three uplink relays, survive the responding machines, and reach the boat. This manual describes the HTML reference, not the current Rust port.]
#plate("mission.svg", [Mission sequence. Extraction is a condition inside PLAYING, not a separate source state.])
#cols[
== How to read this manual
*Exact* facts come from extracted definitions or inspected source branches. *Schematic* plates explain shape, placement or timing and deliberately omit detail. *Interpretation* means implementation advice, not an original game rule.

Use the PDF to understand intent. Use `gallery/assets/*.js` for executable asset construction and `gallery/shared/reference.js` for original geometry and material dependencies. JSON contains derived facts and rendering reports, not model geometry. Resolve gameplay uncertainties against `references/index.html`.

== Mission contract
Deploy at the gate. All three relays begin live and may be cut in any order. Hold interaction while near and facing a live relay. Cutting a relay schedules reinforcements. After the third cut, markers change to extraction and pursuit continues.

Entering extraction while alive and after all cuts completes the run. Being killed opens the death flow; retry resets the mission and increments retries. Pause suspends gameplay; losing pointer lock after acquisition or hiding the page also pauses.

#ref("mM", d)
== Quantitative rules
#facts(o.Yi, ("introSeconds", "retryIntroSeconds", "interactRange", "interactFacing", "cutSeconds", "maxAlive", "spawnStagger", "spawnFar"))
Distances use world metres; time uses seconds. Facing is a dot-product threshold. Relay selection also rejects vertical separation above 2.5 m; very close targets bypass the facing test.

== Completion and scoring
Extraction requires horizontal distance below 2.5 m and vertical difference below 3.5 m. A run starts at full health and resets weapons, effects, combat counters and relay states.

The debrief combines time, accuracy and retries: time below 270 s earns two points, below 400 s one; accuracy at least 50% earns two, at least 30% one; zero retries adds one. Ratings span Messy, Loud, Steady, Clean, Sharp and Surgical.

== Reconstruction priorities
Preserve traversable scale, target readability, enemy warning time, weapon cadence and mission pacing. Small bevels, fastener counts and cosmetic noise can be interpreted after those contracts work.
]
])
#sheet("02", "The playable yard", "Coordinate map / mission locations / reference footprints", [
#plate("level-map.svg", [Top-down placement guide. Spawn dots are authored candidates; runtime navigation repair and jitter may move ground spawns.], height: 143mm)
#cols[
== Coordinates and scale
Y is vertical. Map X increases right and Z down. The player camera starts facing −Z before yaw; the robot builders use +Z as forward. Preserve that distinction when converting orientations.

The warehouse spans X −16…12 and Z −28…−11; floor Y 0.03, eaves 7 m and ridge 9.5 m. The quay edge is X 30. The ladder and boat extraction align with Z 20.
#ref("C1", d)
== Mission positions
#table(columns: (1fr, 1fr), table.header([*Location*], [*X / Y / Z*]), ..d.locations.pairs().map(p => ([#p.first()], [#p.last().map(str).join(" / ")])).flatten())
The drawn links express sequence only. Players can choose relay order; containers and walls determine the actual routes.
]
])
#sheet("03", "Space, cover & collision", "Level assembly / navigation / encounter placement", [
#plate("warehouse-section.svg", [Source-dimension warehouse section; structural envelope only.], height: 35mm)
#cols[
== Build the space in layers
Start with the ground plane, warehouse shell, quay and world boundaries. Add container stacks, stairs, catwalks and major cover. Then place relays and validate access before adding dressing such as pallets, cables, bollards and decals.

The drawing uses first-tier container sizes from the prop registry. The HTML also stacks, rotates and offsets containers. Preserve those relationships by inspecting `hR` and the `stack` helper in `ob`, rather than treating every catalog bound as final geometry.

== Authored zones
#table(columns: (1fr, 1fr, 1fr), table.header([*Zone*], [*X range*], [*Z range*]), [Gate], [−40…−24], [−30…30], [Yard], [−24…16], [−10…30], [Shed], [−24…16], [−30…−10], [Quay], [16…40], [−30…30])
Zone rectangles classify gameplay space. They are not complete physical boundary walls, and the shed zone is larger than the warehouse footprint.

== Collision contracts
The source uses box colliders, capsule movement, raycasts, ground-height queries and a navigation grid. Collider flags distinguish navigation exclusions, bullet-only geometry and player-only blockers. Keep these meanings separate when rebuilding collision in an engine.

Raycasts serve shooting and sensing; ground queries serve foot placement and movement. Visible geometry and collision need not have identical detail. Thin railings, decals and cables should not accidentally seal a walking route.
#ref("d3", d)
== Vertical routes
A yard vantage includes stair-foot location (−2.2, 0, 12.4) and roof location (13.5, 2.59, 12.4). The catwalk connects elevated cover near the container row. Verify stair approach and overhead clearance with the player capsule.

The quay contains a ladder opening and a boat destination. The reference can rescue the player from water; a reconstruction should inspect the player update before choosing fall or water behavior.

== Spawn selection
The source supplies #d.spawns.len() candidate spawns. Ground spawns use the navigation grid; air spawns retain altitude. Candidates are scored for distance, concealment and recent use. A preferred spawn distance of 18 m is a scoring preference rather than an absolute rejection boundary.

Recent positions are penalized for six seconds. Spawn jitter is up to ±1.2 m in X/Z and ground jitter requires walkability. Candidate repair searches neighboring walkable cells before gameplay.

== Cover and sight
`C1` defines explicit cover points. Hounds navigate toward firing positions; gulls move in three dimensions. Validate sight lines from player eye height and enemy sensors, not only from ground-plane points.

*Implementation acceptance:* every relay has an accessible approach, all starting paths fit the capsule, the elevated route is usable, and extraction remains reachable after reinforcements appear.
#ref("hR", d)
#ref("ab", d)
#ref("cb", d)
]
])
#sheet("04", "Player & camera", "Movement tuning / bodily feedback / interaction", [
#cols[
== Movement contract
The player is a capsule with radius #o.Pi.radius m and standing height #o.Pi.height m. Standing eye height is #o.Pi.eye m; crouched eye height is #o.Pi.crouchEye m. Keep collision height and camera height distinct.
#facts(o.Mt, ("walk", "sprint", "crouch", "ads", "accelGround", "decelGround", "accelAir", "gravity", "jumpApex", "coyote", "jumpBuffer", "crouchHeight"))
Speeds are m/s; accelerations and gravity are m/s². Jump apex is 1 m; launch velocity is derived from gravity and apex. Coyote time and input buffering make jumps tolerant around ledges and landing.

== Camera contract
World FOV defaults to #o.Pi.fov degrees, with a separate #(o.Pi.viewFov)-degree viewmodel camera. Sprint adds #o.Mt.fovSprint degrees; aimed FOV targets #o.Mt.fovAds degrees. Pitch is limited to ±#o.Mt.pitchLimit radians.

Aimed look scales by #o.Mt.adsLook. Camera movement includes stride-based bob, strafe roll, landing dip, recoil recovery and damage shake. These are presentation effects; avoid letting them unpredictably move the gameplay collision shape.
#ref("Pi", d)
#ref("Mt", d)
== Health and damage
The player starts at 100 health. Damage resets the regeneration timer; regeneration begins after #o.Mt.regenDelay s and restores #o.Mt.regenRate health/s. Death occurs at zero. Hits trigger directional feedback and camera shake.

Low health causes visual feedback and a one-time radio line below 30 health. Provide cover intervals long enough for the regeneration rule to matter.

== Relay hold interaction
A live relay must be within #o.Yi.interactRange m, within vertical tolerance and sufficiently in front. Holding interaction advances normalized progress by elapsed time divided by #o.Yi.cutSeconds. Completing it permanently cuts that relay for the run.

Leaving the interaction or releasing input decays accumulated progress at 1.6 normalized units/s. It does not instantly reset the internal progress to zero. The relay’s visual state returns to live while interrupted.

== Input and pause
Support movement, look, fire, aim, sprint, crouch, jump, reload, interaction and pause as actions. The reference has keyboard/mouse, gamepad and touch paths, with pointer-lock and drag-look modes. Rebinding is an implementation choice, not a claim about the reference UI.

*Acceptance:* walking the same path at different frame rates produces consistent speed; the player cannot stand into a ceiling; interrupted interaction decays; pause stops gameplay and resume restores input.
#ref("x_", d)
#ref("mM", d)
]
])
#sheet("05", "PAC-9 weapon system", "Active viewmodel / firing / reload / impact pipeline", [
#plate("thumbnails/pac9-side.png", [PAC-9 side view rendered from gallery/assets/pac9.js. Hands and transient effects are excluded.], height: 37mm)
#cols[
== Use the live implementation
The active `dm` builder creates the PAC-9 viewmodel. The fallback weapon stub identifies itself as KR-7 and has different reserve ammunition; that stub is not the weapon specification.
#table(columns: (1.5fr, 1fr), table.header([*Rule*], [*Value*]), [Magazine / Gv], [#d.weapon.Gv rounds], [Reserve / `B_`], [#d.weapon.B_ rounds], [Cadence / `O_`], [#d.weapon.O_ rpm], [Shot damage / `H_`], [#d.weapon.H_], [Range / `V_`], [#d.weapon.V_ m], [Tactical reload / `W_`], [#d.weapon.W_ s], [Empty reload / `G_`], [#d.weapon.G_ s])
At 720 rpm, the nominal shot interval is 0.0833 s. Treat timer accumulation carefully so low frame rates do not silently reduce cadence.

== Visual assembly
Use separate receiver, barrel/handguard, grip, magazine, bolt and optic parts. The reference separates anodized metal, phosphate metal, polymer, tan polymer, brass, glove fabric and rubber. Hands follow the weapon pose with dedicated finger and thumb geometry.

The viewmodel uses its own camera and suppresses world shadow casting. Aim alignment moves the weapon to the sight line rather than simply zooming the world camera.

== Fire pipeline
Input and cooldown decide whether a shot can occur. The weapon consumes ammunition, creates a shot direction including spread, and sends it through combat. Combat resolves enemy or world hits, then drives hitmarkers, effects and sound.

The weapon also animates recoil, bolt travel, muzzle flash, shell ejection, heat and smoke. Inspect `Be` inside `dm` for actual shot production; inspect `zv` for collision and damage resolution. Muzzle visuals and the gameplay ray have different roles.
#ref("dm", d)
== Reload timeline
A reload tracks normalized time, empty/tactical status and phase. The source drives magazine removal, replacement and bolt actions from this sequence. Reload audio should follow those milestones, not a single unchanging sound.

The precise phase thresholds, spread curves and recoil springs remain in the source. They are not approximated as exact values here: search `function Fe()` and `function Xe(` inside `dm`. Complete weapon tuning can be extended into the data package without changing the page layout.

== Feedback priorities
A hit must be distinguishable from a miss; a kill must be distinguishable from an ordinary hit. Surface impacts vary between metal, concrete, wood, water and glass. Reserve and magazine values must remain synchronized with the HUD.

== Implementation scenarios
Verify tactical reload with a partly full magazine, empty reload with reserve available, reserve exhaustion, dry firing, holding fire, aim transitions, sprint interruption, pause during reload and retry reset. Confirm that cosmetic recoil and muzzle flash do not introduce duplicate damage.

*Interpretation:* use a single authoritative shot result to drive damage, sound, particles and UI. This preserves consistent feedback even if an engine uses different rendering and animation APIs.
#ref("Gv", d)
#ref("W_", d)
]
])
#sheet("06", "Hound / construction", "H-03 / narrow chassis / optical head / dorsal gun", [
#plate("thumbnails/hound.png", [Original hound geometry and procedural surfaces, rendered from gallery/assets/hound.js. Static foot placement uses the original IK solver.], height: 62mm)
#grid(columns: (1fr, 1fr), gutter: 5mm,
 image("assets/thumbnails/hound-front.png", width: 100%, height: 52mm, fit: "contain"), image("assets/thumbnails/hound-side.png", width: 100%, height: 52mm, fit: "contain"))
#cols[
== Proportional anchors
#facts(asset-spec.hound, ("stand", "L1", "L2", "l0", "hipX", "hipY", "hipZ", "footR"))
Main shell deck width is 0.35 m; the metal body is 0.27 × 0.20 × 0.98 m. The head envelope is 0.19 × 0.118 × 0.25 m. The long antenna and projecting gun are important silhouette cues.
== Assembly reading
The extracted JavaScript builds the original skinned geometry and its joint hierarchy. White shell panels sit over dark structural metal, with orange bands and bright optics. Small screws, handles, fins and cables give scale but are secondary to chassis width, leg spacing and head position.

Edit `gallery/assets/hound.js` to change parts and local transforms. The gallery and PNG renderer execute that file directly; no JSON geometry conversion sits between source and image. Shared primitive and skinning helpers remain in `gallery/shared/reference.js`.
#ref("Z_", d)
]
])
#sheet("07", "Hound / motion & threat", "Rig relationships / planted feet / attack communication", [
#plate("hound-rig.svg", [Logical articulation tree; four legs repeat the abduction → thigh → shin chain.])
#plate("gait.svg", [Movement explanation, not extracted animation keyframes. The actual rig solves joints from world-space foot targets.])
#cols[
== Grounded movement
The reference alternates diagonal pairs FL+RR and FR+RL. Stance feet retain world positions. Swing targets anticipate movement and turning; a smooth interpolation moves the foot while a sine arc lifts it above the ground. Ground-height queries set landing height.

Inverse kinematics solves hip abduction, thigh and knee angles. Body height, pitch and roll respond to the four foot heights, acceleration and turning. Springs soften those responses. The head aims independently, the turret yaws, the gun pitches and the antenna sways.

This is substantially richer than swinging every limb with a sine wave. Preserve planted feet before adding secondary motion.
#ref("iM", d)
== Threat values
#facts(o.Kv, ("hp", "shotDamage", "burst", "burstGap", "telegraph", "runSpeed", "patrolSpeed", "lungeRange", "lungeDamage", "lungeCooldown", "lungeSpeed"))
== Behavior and warnings
Hounds patrol, investigate, search and engage. Their combat sequence includes telegraph, burst fire, cooldown and close-range lunge. Head and lens feedback make the attack legible. Weapon aim is not the same as chassis heading.

Damaged hounds smoke below a health threshold. Death changes body posture, releases attack ownership, slides feet and creates a wreck. Source code, not the rest pose, determines wreck animation.
]
])
#sheet("08", "Gull & encounter pacing", "Airborne enemy / shared senses / relay reinforcement", [
#plate("thumbnails/gull.png", [Original gull geometry rendered from gallery/assets/gull.js. Motion-blur discs and the searchlight cone are omitted for this static view.], height: 43mm)
#cols[
== Gull construction
Four rotor centers are arranged at X ±0.33 and Z ±0.30 m. The body carries rotor supports and protective rings, landing skids, navigation strobes, an underslung sensor gimbal, lamp and gun. Rotor motion and transparent rotor discs serve different visual roles.

The lamp/sensor ball pitches below a yaw pivot. The gun pitches separately. Keep the sensor line of sight and muzzle position tied to their actual transforms.
#facts(o.Zv, ("hp", "boltDamage", "boltSpeed", "burst", "burstGap", "telegraph", "rangeMin", "rangeMax", "altMin", "altMax", "maxSpeed", "fireRange"))
== Flight and destruction
Gulls orbit during engagement, investigate last-known positions and avoid obstructions with directional feelers. They check clearance above and below, steer with acceleration and bank in response to forces. They do not follow the hound navigation grid.

At death, a rotor is lost, gravity and spin take over, smoke appears, and collision ends in an explosion. Explosion radius is #o.Zv.explodeRadius m; damage is #o.Zv.explodeDamage.
#ref("Q_", d)
== Shared AI contracts
The enemy manager coordinates sight, hearing, exposure and attack tokens. Preserve telegraph windows and limits on concurrent attacks; enemy count alone does not determine difficulty.

Search `requestToken`, `think`, `rally`, `provoke` and `releaseToken` in `Em`. Attack ownership must be released on cancellation, death or deactivation. Do not let pooled inactive enemies retain audio loops or tokens.
#ref("Em", d)
== Reinforcement schedule
#table(columns: (1fr, 1fr, 1fr), table.header([*Trigger*], [*Gulls*], [*Hounds*]), [Initial patrol], [#o.Yi.patrol.gull], [#o.Yi.patrol.hound], ..o.Yi.waves.enumerate().map(p => ([Cut #(p.first()+1)], [#p.last().gull], [#p.last().hound])).flatten())
Waves depend on cut count, not relay identity. The first scheduled spawn is 1.6 s after the cut; subsequent entries are staggered by #o.Yi.spawnStagger s. The active cap is #o.Yi.maxAlive; queued entries defer when capped.

After all cuts, pursuit starts with a 12 s timer and up to #o.Yi.pursuitExtra additional enemies. The nominal pursuit interval is #o.Yi.pursuitEvery s, gated by queue and alive-count conditions. The source also periodically alerts survivors toward the player.

== Verification scenarios
Observe a gull under a roof, a hound behind cover, simultaneous attack requests, a wave at the alive cap, death during a warning, loss of sight and a retry while wrecks remain. In every case, warning, attack, damage and sound should agree.
#ref("mM", d)
]
])
#sheet("09", "Surface & atmosphere", "Materials / weather / rendering / quality tiers", [
#cols[
== Material vocabulary
The reference creates procedural albedo, normal and packed surface textures. Named materials provide painted steel, concrete, asphalt, brick, rubber, polymer, fabric, wood, water and glass. Decals add labels, hazards, grime, rust runs, bullet marks and scorch.

Hound geometry groups into shell, metal, glow and lens slots. Its vertex colors include values above one, so these are not plain display-sRGB paint chips. Preserve the intended light/dark hierarchy rather than interpreting raw triples as final screen colors.
#table(columns: (1fr, 1.4fr), table.header([*Family*], [*Visual intent*]), [Shell], [Light painted armor; orange bands], [Metal], [Dark structure; bright machined fasteners], [Rubber], [Near-black hoses and feet], [Lens / glow], [Amber idle; redder attack feedback], [Ground], [Wet asphalt and concrete; puddle variation], [Sea], [Dark moving surface with horizon haze])

== Readability in weather
Wetness changes roughness and surface response. Rain includes falling streaks and splashes; interior bounds suppress outdoor effects. Puddles and the sea use different scales. Do not apply kilometre-scale sea texture tiling to small puddles.
#ref("Gg", d)
#ref("O1", d)
== Lighting and post-processing
The renderer includes sky and sun lighting, shadows, fog/volumetrics, ambient occlusion, bloom, adaptation, lens dirt, streaks, flare, vignette, grain and sharpening. These effects support the industrial atmosphere but must leave relay lights and attack cues readable.

Damage, low health, aiming, flash, fade and saturation are exposed as post-process signals. Drive them from gameplay events rather than mixing game rules into a shader.

== Quality tiers
#table(columns: (1fr, 1fr, 1fr), table.header([*Tier*], [*Scale*], [*Shadow*]), ..("low", "medium", "high", "ultra").map(k => ([#k], [#str(o.Pr.at(k).scale)], [#str(o.Pr.at(k).shadow)])).flatten())
The complete tier definitions are in the data package, including MSAA, texture size, volumetric steps, AO, planar reflections, bloom, motion blur and rain. Runtime adaptive scaling can modify the effective resolution.

== Reconstruction guidance
Build a readable daylight scene first, then add sky/fog and wet surfaces. Validate combat cues before enabling glare and grain. Quality reductions should change visual cost while preserving collision, sensing, interaction reach and weapon behavior.

*Acceptance:* optics remain visible against both warehouse shadow and bright sky; rain does not visually continue through the warehouse; texture repetition remains plausible at player scale.
]
])
#for page in range(calc.ceil(props.len() / 9)) {
 sheet(str(10+page), "Asset thumbnails / " + str(page+1), "Executable JavaScript builders / original procedural surfaces / studio lighting", [
  #grid(columns: (1fr, 1fr, 1fr), gutter: 3mm, ..props.slice(page*9, calc.min((page+1)*9, props.len())).map(asset-card))
  #v(4pt)
  #cols[
   == Source of truth
   Every tile is a real Three.js render of its named builder. Edit the listed file under `gallery/`, then rebuild to refresh both the gallery and PDF. Nominal dimensions are read from the extracted JavaScript registry.
   == Reading the views
   Assets are framed individually, so image sizes are not mutually to scale. Lighting and exposure are standardized; construction geometry and surfaces come from the extracted source. Detailed colliders remain in the builders.
  ]
 ])
}
#sheet("14", "HUD, audio & delivery", "Presentation contracts / build instructions / implementation checks", [
#plate("hud.svg", [Screen-space hierarchy of the reference HUD. Exact spacing adapts to display and input mode.])
#cols[
== HUD and menus
Display heading, relay objective/count, world markers, crosshair, hitmarkers, directional damage, interaction progress, radio messages, health, magazine, reserve and reload cues. Pause, death and completion have distinct screens.

Settings include quality, sensitivity, invert-Y, volume and FOV. Touch mode changes layout and supplies stick/fire/aim/jump/reload/interact/pause controls. Keep control prompts synchronized with the active input mode.
#ref("hM", d)
#ref("pM", d)
== Sound event map
#table(columns: (1fr, 1.4fr), table.header([*Event*], [*Sound family*]), [Player fires], [Rifle shot and interior/outdoor tail], [Reload phase], [Magazine out / in / bolt], [Footstep], [Concrete / water / metal], [Enemy], [Rotor / servo loops, warning, firing, death], [World], [Rain, sea, wind, thunder, foghorn], [Mission], [Relay cut, objective, completion, death])
The source synthesizes audio buffers and spatializes world sounds. Music intensity changes from quiet patrol to contact and active waves/extraction. Interior transitions affect ambience and reverb. Pause and pooled-enemy cleanup must stop or attenuate the appropriate loops.
#ref("u_", d)
== Source package
Run `python3 docs/game-manual/build.py` from the repository root. Node extracts gameplay facts and renders the editable asset JavaScript in Chromium. Python generates map and behavior diagrams; Typst compiles this PDF. No game code or reference source is modified.

`reference.json` contains the source SHA-256, searchable anchors and line numbers. Asset thumbnails and measured bounds are generated from the asset JavaScript, while gameplay tables are regenerated from the HTML reference. Inspect anchors before carrying numbers into a port.

== Verification checklist
Validate movement and capsule clearances; all relay approaches; interruption decay; wave cap and retry cleanup; tactical/empty reload; enemy warning cancellation; grounded feet; air clearance; extraction gating; sound loop retirement; and HUD consistency.

The document build checks extraction counts, SVG XML validity and successful Typst compilation. It does not prove game equivalence. Compare a reconstructed level and enemy behavior against the original in play.

== Deliberate interpretation
Thumbnail geometry comes from executable JavaScript builders. Studio lighting, dry surfaces, fixed inspection poses and omission of transient effects are presentation choices. The gait strip explains behavior rather than providing authoritative poses. The source package remains available wherever the drawing leaves implementation freedom.
]
])

#sheet("15", "Source navigation", "Evidence index / JavaScript asset authority / regeneration", [
#note[Reference fingerprint (SHA-256): #text(size: 7pt, raw(d.sha256))]
#cols[
== Searchable definitions
#table(columns: (1fr, 1.8fr), table.header([*Symbol*], [*Responsibility*]),
[Pi / Mt], [Capsule, eye height, camera and movement],
[Yi / mM], [Mission, waves, relay and extraction logic],
[C1 / hR], [Level assembly, spawns and container rows],
[no / ab], [Warehouse dimensions and construction],
[lb / cb], [Quay, ladder and extraction],
[Gi / Yg], [Prop dimensions and registered builders],
[Z\_ / iM], [Hound geometry and motion],
[j\_ / Q\_], [Gull geometry and flight],
[Em], [Enemy manager, shared senses and tokens],
[dm / zv], [PAC-9 and combat ray resolution],
[Gg / O1], [Materials, wet ground, sea and rain],
[hM / pM], [HUD, menus and settings],
[u\_], [Audio generation, mixing and spatial loops])

== Read the executable assets
`gallery/assets/hound.js`, `gull.js` and `pac9.js` hold the extracted original geometry builders. Each prop builder has its own named file; the three container variants share `containers.js`. The original Three.js library, primitive helpers, materials and registry wiring live in `gallery/shared/reference.js`.

`gallery/registry.js` supplies readable names, static poses and material adapters. `gallery/gallery.js` provides camera framing and inspection controls. Those JavaScript files are the editable asset authority. A normal manual build executes them; it does not overwrite them from the original HTML.

The thumbnail manifest records camera view, measured bounds, builder path and builder-file SHA-256. It is a derived report. Neither it nor `reference.json` reconstructs asset geometry.

== Units and transforms
World distances are metres, elapsed times seconds, speeds metres/second, rotations radians unless explicitly described as FOV degrees. Material colors are source values rather than guaranteed display colors. A prop registry size is nominal and may differ from its detailed collider shape.

World Y is up. Model forward is +Z; the player camera uses −Z before yaw. Source Euler transforms use XYZ order. Do not mirror the front and rear legs when converting to a different engine convention.

== Check the source, then interpret
Use exact values for mechanics and major spatial relationships. Use diagrams for assembly intent. Where cosmetic freedom remains, choose an implementation that preserves silhouette, scale, material contrast and motion readability.

The map omits dressing, stack heights and rotations. The dashed mission links are not a route planner. The walking strip is explanatory. Asset grids show actual rendered geometry, including procedural material detail, at individually normalized scales.

== Rebuild and review
A rebuild refreshes gameplay facts, renders the asset JavaScript to PNGs, regenerates explanatory vectors and compiles the manual. Refresh asset source from the HTML only with the explicit `rip-assets.cjs` command; that import step overwrites extracted builder edits. The extractor fails if expected source definitions or location anchors disappear. Review the complete PDF after changing layout or dimensions.

The build does not run the original game or certify gameplay equivalence. Acceptance requires walking the level, cutting relays in multiple orders, exercising both enemies and comparing actual combat feedback.
]
])
