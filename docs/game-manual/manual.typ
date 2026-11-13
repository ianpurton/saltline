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
#note[*SALTLINE* is a single-player industrial-port FPS. Cut three uplink relays, survive the responding machines, and reach the boat. Rain, salt haze and wet steel frame a compact berth where exposed lanes alternate with container cover and warehouse shadow. The intended rhythm is reconnaissance, a committed relay hold, a sharp reinforcement response, then relocation.]
#plate("mission.svg", [Mission sequence. Extraction is a condition inside PLAYING, part of the active mission.])
#cols[
== Design and asset package
This document specifies the playable mission in metres and seconds. The accompanying `gallery/assets/*.js` files define executable model construction; retain their proportions, surfaces and articulation when translating them to another engine. The gallery provides orbitable inspection views. The following chapters and detailed rule sheets define movement, combat, enemy behavior and level placement independently of the asset code.

== Mission contract
Deploy at the gate. All three relays begin live and may be cut in any order. Hold interaction while near and facing a live relay. Cutting a relay schedules reinforcements. After the third cut, markers change to extraction and pursuit continues.

Entering extraction while alive and after all cuts completes the run. Being killed opens the death flow; retry resets the mission and increments retries. Pause suspends gameplay; losing pointer lock after acquisition or hiding the page also pauses.

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
#sheet("02", "The playable yard", "Coordinate map / mission locations / structure footprints", [
#plate("level-map.svg", [Top-down placement guide. Spawn dots are authored candidates; runtime navigation repair and jitter may move ground spawns.], height: 143mm)
#cols[
== Coordinates and scale
Y is vertical. Map X increases right and Z down. The player camera starts facing −Z before yaw; the robot builders use +Z as forward. Preserve that distinction when converting orientations.

The warehouse spans X −16…12 and Z −28…−11; floor Y 0.03, eaves 7 m and ridge 9.5 m. The quay edge is X 30. The ladder and boat extraction align with Z 20.
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

The drawing uses first-tier container sizes from the prop registry. The placement schedule later in this document specifies every container tier, including offsets, rotations and high-cube heights.

== Authored zones
#table(columns: (1fr, 1fr, 1fr), table.header([*Zone*], [*X range*], [*Z range*]), [Gate], [−40…−24], [−30…30], [Yard], [−24…16], [−10…30], [Shed], [−24…16], [−30…−10], [Quay], [16…40], [−30…30])
Zone rectangles classify gameplay space. They are not complete physical boundary walls, and the shed zone is larger than the warehouse footprint.

== Collision contracts
The game uses box colliders, capsule movement, raycasts, ground-height queries and a navigation grid. Collider flags distinguish navigation exclusions, bullet-only geometry and player-only blockers. Keep these meanings separate when rebuilding collision in an engine.

Raycasts serve shooting and sensing; ground queries serve foot placement and movement. Visible geometry and collision need not have identical detail. Thin railings, decals and cables should not accidentally seal a walking route.
== Vertical routes
A yard vantage includes stair-foot location (−2.2, 0, 12.4) and roof location (13.5, 2.59, 12.4). The catwalk connects elevated cover near the container row. Verify stair approach and overhead clearance with the player capsule.

The quay contains a ladder opening and boat destination. Falling 2.2 m below sea level returns the player to the last grounded safe position inland of X 28.8, zeros velocity, turns yaw by π and deals 12 damage. Landing after more than 0.12 s airborne deals 7 × (impact speed − 13) damage when impact speed exceeds 13 m/s.

== Spawn selection
The level supplies #d.spawns.len() candidate spawns. Ground spawns use the navigation grid; air spawns retain altitude. Candidates are scored for distance, concealment and recent use. A preferred spawn distance of 18 m is a scoring preference rather than an absolute rejection boundary.

Recent positions are penalized for six seconds. Spawn jitter is up to ±1.2 m in X/Z and ground jitter requires walkability. Candidate repair searches neighboring walkable cells before gameplay.

== Cover and sight
Authored cover points support the container lanes and quay. Hounds navigate toward firing positions; gulls move in three dimensions. Validate sight lines from player eye height and enemy sensors, not only from ground-plane points.

*Implementation acceptance:* every relay has an accessible approach, all starting paths fit the capsule, the elevated route is usable, and extraction remains reachable after reinforcements appear.
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
== Health and damage
The player starts at 100 health. Damage resets the regeneration timer; regeneration begins after #o.Mt.regenDelay s and restores #o.Mt.regenRate health/s. Death occurs at zero. Hits trigger directional feedback and camera shake.

Low health causes visual feedback and a one-time radio line below 30 health. Provide cover intervals long enough for the regeneration rule to matter.

== Relay hold interaction
A live relay must be within #o.Yi.interactRange m, within vertical tolerance and sufficiently in front. Holding interaction advances normalized progress by elapsed time divided by #o.Yi.cutSeconds. Completing it permanently cuts that relay for the run.

Leaving the interaction or releasing input decays accumulated progress at 1.6 normalized units/s. It does not instantly reset the internal progress to zero. The relay’s visual state returns to live while interrupted.

== Input and pause
Support movement, look, fire, aim, sprint, crouch, jump, reload, interaction and pause as actions. The game has keyboard/mouse, gamepad and touch paths, with pointer-lock and drag-look modes. Rebinding is an implementation choice, not a claim about the game UI.

*Acceptance:* walking the same path at different frame rates produces consistent speed; the player cannot stand into a ceiling; interrupted interaction decays; pause stops gameplay and resume restores input.
]
])
#sheet("05", "PAC-9 weapon system", "Active viewmodel / firing / reload / impact pipeline", [
#plate("thumbnails/pac9-side.png", [PAC-9 side view rendered from gallery/assets/pac9.js. Hands and transient effects are excluded.], height: 37mm)
#cols[
== PAC-9 specification
#table(columns: (1.5fr, 1fr), table.header([*Rule*], [*Value*]), [Magazine], [#d.weapon.Gv rounds], [Reserve], [#d.weapon.B_ rounds], [Cadence], [#d.weapon.O_ rpm], [Shot damage], [#d.weapon.H_], [Range], [#d.weapon.V_ m], [Tactical reload], [#d.weapon.W_ s], [Empty reload], [#d.weapon.G_ s])
At 720 rpm, the nominal shot interval is 0.0833 s. Treat timer accumulation carefully so low frame rates do not silently reduce cadence.

== Visual assembly
Use separate receiver, barrel/handguard, grip, magazine, bolt and optic parts. The game separates anodized metal, phosphate metal, polymer, tan polymer, brass, glove fabric and rubber. Hands follow the weapon pose with dedicated finger and thumb geometry.

The viewmodel uses its own camera and suppresses world shadow casting. Aim alignment moves the weapon to the sight line rather than simply zooming the world camera.

== Fire pipeline
Input and cooldown decide whether a shot can occur. The weapon consumes ammunition, creates a shot direction including spread, and sends it through combat. Combat resolves enemy or world hits, then drives hitmarkers, effects and sound.

The weapon also animates recoil, bolt travel, muzzle flash, shell ejection, heat and smoke. Shots originate at the player eye; muzzle-to-impact tracers are cosmetic. Muzzle visuals and the gameplay ray have different roles.
== Reload timeline
A reload tracks normalized time, empty/tactical status and phase. The game drives magazine removal, replacement and bolt actions from this sequence. Reload audio should follow those milestones, not a single unchanging sound.

Magazine removal occurs at 0.47 s; the replacement becomes visible after 0.86 s; insertion transfers min(magazine capacity − rounds remaining, reserve) at 1.345 s. A seating tap occurs at 1.5 s. Empty reloads pull the bolt from 2.02 to 2.14 s, hold until 2.19 s and release over 0.035 s. Fire is blocked throughout the reload. A dry trigger clicks once per press and schedules reload after 0.32 s when reserve exists.

Aim blends over 0.18 s with quintic easing. Full spread and recoil rules appear in the detailed combat sheet.

== Feedback priorities
A hit must be distinguishable from a miss; a kill must be distinguishable from an ordinary hit. Surface impacts vary between metal, concrete, wood, water and glass. Reserve and magazine values must remain synchronized with the HUD.

== Implementation scenarios
Verify tactical reload with a partly full magazine, empty reload with reserve available, reserve exhaustion, dry firing, holding fire, aim transitions, sprint interruption, pause during reload and retry reset. Confirm that cosmetic recoil and muzzle flash do not introduce duplicate damage.

*Interpretation:* use a single authoritative shot result to drive damage, sound, particles and UI. This preserves consistent feedback even if an engine uses different rendering and animation APIs.
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
]
])
#sheet("07", "Hound / motion & threat", "Rig relationships / planted feet / attack communication", [
#plate("hound-rig.svg", [Logical articulation tree; four legs repeat the abduction → thigh → shin chain.])
#plate("gait.svg", [Movement explanation, not extracted animation keyframes. The actual rig solves joints from world-space foot targets.])
#cols[
== Grounded movement
The game alternates diagonal pairs FL+RR and FR+RL. Stance feet retain world positions. Swing targets anticipate movement and turning; a smooth interpolation moves the foot while a sine arc lifts it above the ground. Ground-height queries set landing height.

Inverse kinematics solves hip abduction, thigh and knee angles. Body height, pitch and roll respond to the four foot heights, acceleration and turning. Springs soften those responses. The head aims independently, the turret yaws, the gun pitches and the antenna sways.

This is substantially richer than swinging every limb with a sine wave. Preserve planted feet before adding secondary motion.
== Threat values
#facts(o.Kv, ("hp", "shotDamage", "burst", "burstGap", "telegraph", "runSpeed", "patrolSpeed", "lungeRange", "lungeDamage", "lungeCooldown", "lungeSpeed"))
== Behavior and warnings
Hounds patrol, investigate, search and engage. Their combat sequence includes telegraph, burst fire, cooldown and close-range lunge. Head and lens feedback make the attack legible. Weapon aim is not the same as chassis heading.

Hounds smoke below 56 health (40% of maximum). Death changes body posture, releases attack ownership, slides feet and creates a wreck. The chassis drops to 0.215 m with a 0.22 bounce factor; legs splay and slide, head droops and the optics flicker for 1.2 s. Smoke lasts 6.5 s. Retain five wrecks; older wrecks sink at 0.22 m/s for 3.2 s before removal.
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
== Shared AI contracts
Sight, hearing and ranged attack ownership control combat pressure.

Sight uses a 45 m range and 200° horizontal cone, bypassed within 3.5 m or while engaged. Sample visibility every 0.11–0.19 s. Hounds require 0.55 s visible contact; gulls 0.4 s; investigating/searching enemies need 45% of that. Loss of sight for over 4 s enters an 8–11 s search, then patrol. Sight accumulates exposure at 0.1/s and absence removes 0.45/s.

Two enemies may own ranged attacks simultaneously, rising to three above five living enemies. Grants are at least 1.25 s apart. If the other species requested in the past 0.5 s, a repeated-species grant waits until 2.75 s after the previous grant. Release ownership after bursts, cancellation, death or removal. Melee does not require ownership.

An alert rallies non-engaged allies within 18 m to investigate for 9–13 s. Taking a hit provokes investigation for 10 s with immediate sensing and rallies within 20 m. Gunshots alert within 45 m, throttled to once per 0.3 s; explosions alert within 30 m. Hearing supplies a location rather than immediate permission to shoot.

== Reinforcement schedule
#table(columns: (1fr, 1fr, 1fr), table.header([*Trigger*], [*Gulls*], [*Hounds*]), [Initial patrol], [#o.Yi.patrol.gull], [#o.Yi.patrol.hound], ..o.Yi.waves.enumerate().map(p => ([Cut #(p.first()+1)], [#p.last().gull], [#p.last().hound])).flatten())
Waves depend on cut count, not relay identity. The first scheduled spawn is 1.6 s after the cut; subsequent entries are staggered by #o.Yi.spawnStagger s. The active cap is #o.Yi.maxAlive; queued entries defer when capped.

After all cuts, pursuit starts with a 12 s timer and up to #o.Yi.pursuitExtra additional enemies. The nominal pursuit interval is #o.Yi.pursuitEvery s, gated by queue and alive-count conditions. The game also periodically alerts survivors toward the player.

]
])
#sheet("09", "Surface & atmosphere", "Materials / weather / rendering / quality tiers", [
#cols[
== Material vocabulary
The game creates procedural albedo, normal and packed surface textures. Named materials provide painted steel, concrete, asphalt, brick, rubber, polymer, fabric, wood, water and glass. Decals add labels, hazards, grime, rust runs, bullet marks and scorch.

Hound geometry groups into shell, metal, glow and lens slots. Its vertex colors include values above one, so these are not plain display-sRGB paint chips. Preserve the intended light/dark hierarchy rather than interpreting raw triples as final screen colors.
#table(columns: (1fr, 1.4fr), table.header([*Family*], [*Visual intent*]), [Shell], [Light painted armor; orange bands], [Metal], [Dark structure; bright machined fasteners], [Rubber], [Near-black hoses and feet], [Lens / glow], [Amber idle; redder attack feedback], [Ground], [Wet asphalt and concrete; puddle variation], [Sea], [Dark moving surface with horizon haze])

== Readability in weather
Wetness changes roughness and surface response. Rain includes falling streaks and splashes; interior bounds suppress outdoor effects. Puddles and the sea use different scales. Do not apply kilometre-scale sea texture tiling to small puddles.
== Lighting and post-processing
The renderer includes sky and sun lighting, shadows, fog/volumetrics, ambient occlusion, bloom, adaptation, lens dirt, streaks, flare, vignette, grain and sharpening. These effects support the industrial atmosphere but must leave relay lights and attack cues readable.

Damage, low health, aiming, flash, fade and saturation are exposed as post-process signals. Drive them from gameplay events rather than mixing game rules into a shader.

== Quality tiers
#table(columns: (1fr, 1fr, 1fr), table.header([*Tier*], [*Scale*], [*Shadow*]), ..("low", "medium", "high", "ultra").map(k => ([#k], [#str(o.Pr.at(k).scale)], [#str(o.Pr.at(k).shadow)])).flatten())
Quality changes may reduce MSAA, texture resolution, volumetric steps, ambient occlusion, planar reflections, bloom, motion blur and rain density. Runtime scaling can modify effective resolution.

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
   Assets are framed individually, so image sizes are not mutually to scale. Lighting and exposure are standardized; construction geometry and surfaces come from the executable builders. Detailed colliders remain in the builders.
  ]
 ])
}
#sheet("14", "HUD, audio & delivery", "Presentation contracts / build instructions / implementation checks", [
#plate("hud.svg", [Screen-space hierarchy of the game HUD. Exact spacing adapts to display and input mode.])
#cols[
== HUD and menus
Display heading, relay objective/count, world markers, crosshair, hitmarkers, directional damage, interaction progress, radio messages, health, magazine, reserve and reload cues. Pause, death and completion have distinct screens.

Settings include quality, sensitivity, invert-Y, volume and FOV. Touch mode changes layout and supplies stick/fire/aim/jump/reload/interact/pause controls. Keep control prompts synchronized with the active input mode.
== Sound event map
#table(columns: (1fr, 1.4fr), table.header([*Event*], [*Sound family*]), [Player fires], [Rifle shot and interior/outdoor tail], [Reload phase], [Magazine out / in / bolt], [Footstep], [Concrete / water / metal], [Enemy], [Rotor / servo loops, warning, firing, death], [World], [Rain, sea, wind, thunder, foghorn], [Mission], [Relay cut, objective, completion, death])
The game synthesizes audio buffers and spatializes world sounds. Music intensity changes from quiet patrol to contact and active waves/extraction. Interior transitions affect ambience and reverb. Pause and pooled-enemy cleanup must stop or attenuate the appropriate loops.
== Verification checklist
Validate movement and capsule clearances; all relay approaches; interruption decay; wave cap and retry cleanup; tactical/empty reload; enemy warning cancellation; grounded feet; air clearance; extraction gating; sound loop retirement; and HUD consistency.

The document build checks extraction counts, SVG XML validity and successful Typst compilation. It does not prove game equivalence. Compare a reconstructed level and enemy behavior against the original in play.

== Reading the catalogue
Studio thumbnails use dry surfaces and fixed poses to clarify construction. In play, use the wet lighting and moving articulation specified here. Individually framed images do not share an image scale; use their metre dimensions.
]
])

#include "details.typ"
