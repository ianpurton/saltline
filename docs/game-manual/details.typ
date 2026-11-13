#import "theme.typ": *
#let d = json("data/reference.json")
#sheet("15", "Combat rulebook", "Spread, recoil and enemy attack state machines", [
#cols[
== Shot distribution and recovery
Let A be eased aim blend, S = clamp(horizontal speed / 6, 0, 1), and B be bloom. Target angular spread in radians is:

`lerp(.0125,.0012,A) + S*lerp(.013,.004,A) + B*lerp(.0065,.0022,A) + airborne*.022 - crouched*.002*(1-A)`.

Follow this target exponentially at rate 18/s. Sample angle uniformly over 2π and radius as tan(spread) × sqrt(uniform random); normalize (cos(angle) × radius, sin(angle) × radius, −1) in camera space. Each shot adds 0.17 bloom, capped at 1; bloom falls by 1.5/s. Aim uses a linear 0.18 s transition eased with `6t^5 - 15t^4 + 10t^3`.

Held fire catches up missed shot intervals, at most three shots per update. Cooldown cannot accumulate more than 0.001 s credit while idle. Fire requires no reload and sprint blend below 0.45; aim requires sprint blend below 0.35. Sprint blend follows at exponential rate 9/s and is suppressed by fire, aim or reload. Burst index resets after 0.24 s without shooting, randomizing lateral phase.

== Recoil
Per-shot multiplier M = lerp(1, .55, A) × (crouched ? .85 : 1). Pitch impulse = .0068 × M × uniform(.85,1.15), reduced to 38% after the ninth consecutive shot. For shots 1–3 yaw is uniform(−.0011,.0011) × M; subsequently yaw is `(sin(shotIndex*.57 + phase)*.0042 + uniform(-.001,.001))*M`.

Keep 35% of pitch/yaw permanently in look orientation. The remaining 65% is recoverable displacement, clamped to ±0.3 pitch and ±0.2 yaw. Hold recovery for .09 s; then decay at 6.5/s. Displayed displacement follows at 34/s.

Viewmodel recoil uses damped springs: backward translation stiffness/damping 520/30, pitch 380/26, yaw 300/22, roll 300/20. Shot velocity impulses are respectively 1.5 × lerp(1,.6,A), 1.5M, uniform(−.5,.5)M, and uniform(−.9,.9)M. Use spring acceleration = −stiffness × displacement − damping × velocity. Heat rises .05/shot to 1.3 and cools .2/s. Above .4 heat, stopping fire produces muzzle smoke after .11 s.

== Hound attack machine
Engagement begins by selecting a firing position. Move at 4.5 m/s; reconsider when the player moves over 7 m. Aim on arrival, after 7 s moving, or probabilistically after 1.4 s with visibility and range 7–20 m (probability .7 × elapsed seconds per update).

In aim/cool, face the player. With sight, expired cooldown, living target, aim error below .14 rad and attack ownership, warn for .5 s, then fire four hitscan rounds .11 s apart. Cooldown is uniform 1.8–3 s. After the first burst, 60% reposition and 40% allow a second burst; after two, reposition. Without sight, reposition after 1.1 s unseen or 3 s aiming. At less than 5 m, reposition after .8 s aiming.

From move, aim or cool, distance below 3 m and expired melee cooldown interrupts ranged ownership: wind up .28 s → lunge .36 s at 9.5 m/s → recover .55 s → reposition. Distance below 1.35 m deals 15 damage once and ends the lunge. Melee cooldown is 3.5 s.

Hound shots aim at player feet + 1 m. Add independent uniform noise to each direction component with half-width `lerp(.12,.045,exposure)*(1+filteredPlayerSpeed*.3)`, then normalize. Range is 200 m; tracer speed 260 m/s is visual. Player speed filtering uses exponential rate 6/s.

== Gull attack machine
While engaged, visible, cooldown expired, facing error below .35 rad and target within 42 m, acquire ownership and warn .55 s. Leaving engagement cancels the warning. Fire three bolts .12 s apart, then release ownership and wait uniform 1.9–3.3 s. Bolts travel at 70 m/s, live 1.3 s and deal 7 damage. World obstruction stops them.

Choose lead fraction uniform .2–.85 per burst. Target height is player feet + 1.05 m; horizontal lead = player velocity × (distance / 70) × lead fraction. Component noise half-width is `lerp(.065,.018,exposure)*(1+filteredPlayerSpeed*.45)`, multiplied by 1.5 below half gull health. Firing adds .35 m/s recoil opposite the bolt direction.
]
])
#sheet("16", "Motion and encounter rules", "Grounding, flight, reinforcement selection and progression", [
#cols[
== Hound foot solver
Alternate diagonal pairs, choosing the pair with largest predicted horizontal foot error; weight the pair not used last by 1.25. Let J = speed + abs(yaw rate) × .5. Swing duration N = clamp(.27 − J × .026, .14, .27). Trigger at .09 m error while moving (speed > .15 or yaw rate > .25); while idle for over .25 s use .035 m. Predict targets 1.5N seconds ahead using translation and yaw rate. Sample ground below each landing target.

Swing interpolation is smoothstep t²(3−2t), plus sin(πt) × lift; lift = .065 + min(.09,J × .02). Stance feet stay planted in world space. Solve abduction and two-link leg IK after undoing body pitch/roll/yaw. Effective lower link length subtracts .6 × foot radius; add that amount to foot target Y.

Pitch target = .6 × atan2(rear mean foot Y − front mean foot Y,.8), plus clamp(acceleration × .006,−.07,.07). Roll = .6 × atan2(left mean Y − right mean Y,.55), plus clamp(yawRate × speed × .02,−.12,.12). Body height offset = .45 × (mean foot Y − ground body Y) − min(.05,speed × .011). Height spring stiffness/damping is 160/17, pitch and roll 130/15; add idle breathing sin(time × 1.7 + phase) × .004 m. Antenna springs 60/5 follow acceleration and turning, clamped ±.6 rad.

== Gull steering
During combat orbit the player at radius 12–25 m, altitude offset 4–8 m. Every 3–8 s, reverse direction with 60% probability and perturb radius by ±3 m, clamped 13–23 m. Orbit tangential rate is 3.6/radius rad/s, reduced to .9/radius during warning/burst. After 1.5 s unseen, steer toward last known X/Z and last known Y + altitude + 1.

Investigation/search orbit radius 4.5 m at .5 rad/s, speed cap 5.5 m/s, altitude last known Y + min(altitude,5.5). Patrol circles home at .28 rad/s and 3 m/s. Steering acceleration is horizontal error × 2.2 − velocity × 2.4, vertical error × 5 − vertical velocity × 3.6, capped to 11 m/s². Combat speed cap is 8 m/s.

Cycle feelers forward, down and up. Forward reach is 2.2 + horizontal speed × .5; obstruction adds normal × 26 × proximity and upward 9 × proximity. Strong proximity over .5 reverses orbit. Body clearance is .62 m horizontally and .35 vertically; collisions reverse the affected velocity component at 25%. Maintain ground clearance 1.1 m. Gimbal follows target exponentially at 9/s; pitch −.5…1.5 rad; gun pitch −.2…1.3 rad, compensating body pitch.

Death removes one random rotor, adds upward velocity 1.5 m/s and spin ±5–8 rad/s. Gravity 9.5 m/s²; horizontal drag .4/s; spin accelerates 7 rad/s². Collision, time over 5 s or Y below −3 ends the fall in a radius-3 m, 30-damage explosion.

== Spawn scoring and pressure
For each candidate score uniform(0,1.5), plus 2 + .5 × min(1,(distance−preferred)/30) beyond preferred distance, otherwise minus (preferred−distance)/4. Add 2.5 if hidden from player eye; subtract 3 if used in the last 6 s. Highest score wins. Preferred distance is 32 m for initial patrol and 18 m for waves. Apply X/Z jitter ±1.2 m if airborne or walkable. Air spawns below 2.5 m rise to uniform 6–9 m.

Wave order interleaves hounds into the gull list at index 1, then 3, then 5. Schedule from cut time + 1.6 s with .85 s spacing. Spawn at most one queued enemy per update. At nine living enemies defer due entries 1 s. Failed activation retries in 1.5 s, up to six attempts. A successful arrival alerts enemies within 60 m toward the player.

After the last cut, wait 12 s. When queue empty and fewer than four enemies live, schedule one more after .5 s, 60% gull / 40% hound; allow five extras and reset timer to 7 s. When due but blocked, reset to 4 s and alert survivors within 90 m.

== Mission flow and feedback
Title → intro (3.5 s; retry 1.3 s) → playing. Pause freezes the mission clock and simulation. Retry clears all enemies, wrecks, projectiles, cuts and combat statistics and restores full health/ammunition; only retry count persists. Dead presentation desaturates toward .12 and fades toward .55, showing the death menu after 1.5 s. Completion disables movement/damage immediately, fades toward .72 and shows debrief after 3.3 s.

No-shot accuracy is unavailable and contributes zero accuracy points. Ratings map total points 0–5 to Messy, Loud, Steady, Clean, Sharp, Surgical. Kills are displayed but do not affect rating.

Music intensity is 1 during extraction, queued waves or active wave pressure, .6 for recent alerts or enemies within 26 m, otherwise .15. Alerts persist 10 s, damage at least 8 s; wave pressure lasts 30 s but clears early once all enemies and queue are gone after its first 6 s.
]
])
#sheet("17", "Level assembly schedule", "Container tiers and elevated route / metres and radians", [
Containers run along X at yaw zero. Standard 20 ft is 6.06 × 2.438 × 2.59 m (length × width × height); 40 ft is 12.19 × 2.438 × 2.59 m; high cube (HC) is 12.19 × 2.438 × 2.9 m. Entries below are bottom to top. X/Z are stack centers; base Y is zero. Tier Y equals the sum of lower tier heights. Jitter is a multiplier: upper X offsets add uniform ±.05 × jitter, Z ±.04 × jitter; yaw adds uniform ±.022 × jitter radians (× .6 above tier zero). Authored dx/dz offsets add in world X/Z. Door numbers are opening amounts; flip reverses orientation.
#table(columns: (1fr, .7fr, .7fr, 3fr), table.header([*X / Z*],[*Yaw*],[*Jitter*],[*Tiers: size, color, options*]), ..d.assembly.stacks.map(s => ([#measurement(s.at(0)) / #measurement(s.at(1))], [#measurement(s.at(3,default:0))], [#measurement(s.at(4,default:1))], [#s.at(2).map(t => repr(t)).join(" → ")])).flatten())
Stairs at (−.1,0,12.42), yaw 0, rise to the blue container roof; catwalk at (8.98,2.54,12.42) bridges blue and green containers. Four support rods stand at X 7.6/10.36 and Z 11.86/12.98 up to Y 2.5. Roof cover is placed at Y 2.59. Keep these surfaces continuous and traversable.
])
#sheet("18", "Authored prop placement", "Yard, gate approach and elevated cover", [
Each row gives builder, X/Z, yaw, options, base Y. Displayed measurements are rounded to three decimal places. Unspecified elevation is ground level. Use catalogue dimensions and the named executable builder for visual construction and detailed collision. Props marked noCol form part of the forklift load, with its separate wood collision envelope.
#text(size:8pt)[
#table(columns:(1.2fr,1fr,.55fr,1.8fr,.5fr),table.header([*Asset*],[*X / Z*],[*Yaw*],[*Options*],[*Y*]),..d.assembly.placements.filter(p => p.at(0) != "stairs" and p.at(0) != "catwalk").map(p => ([#str(p.at(0))],[#measurement(p.at(1)) / #measurement(p.at(2))],[#measurement(p.at(3))],[#repr(p.at(4,default: (:)))],[#measurement(p.at(5,default:0))])).flatten())
]
])
#sheet("19", "Spawn coordinate schedule", "Authored candidates / ground positions are repaired to walkable navigation", [
#cols[
#table(columns:(.7fr,1.5fr,.7fr),table.header([*Kind*],[*X / Y / Z*],[*Zone*]),..d.spawns.map(s => ([#s.type],[#s.pos.map(str).join(" / ")],[#s.zone])).flatten())
== Boundary and quay
Ground yard bounds are X −40…40, Z −30…30; clamp player centers .4 m inside. Sea level is −1.6 m. Quay lip is X 30, Z −31…29, from Y −.7 to .028, with a .65 m concrete width. The sea-facing wall extends to −4.2 m. Yellow edge paint, mooring bollards and tire fenders establish the hazardous edge.

The ladder is at X 29.88, Z 20, with side rails Z 19.74/20.26 down to −3.9 m and rungs every .3 m. Its .84 m opening spans Z 19.58…20.42. Yellow safety posts at X 29.18 stand at Z 17.4,19.1,20.9,22.6 with chains between the outer pairs, leaving the middle passage open. Extraction is the ground-side radius around (29,0,20), without a climb requirement.

Bollards stand X 29.2 at Z −22,−12,−2,9,16.4,25.6. Crane rails run along Z −29.6…27.6 at X 18.5 and 28.5. Preserve their small .034 m raised steel profiles rather than treating them as high walls.
]
])
#sheet("20", "Complete movement tuning", "Camera, strides and bodily feedback", [
Exponential following at rate r means new = target + (old − target) × exp(−r × elapsed seconds). Spring stiffness/damping uses acceleration = stiffness × (target − position) − damping × velocity. Angles below are radians except FOV in degrees. Strides are metres per cycle; movement speeds are m/s, acceleration and gravity m/s².
#cols[
#facts(d.objects.Mt, d.objects.Mt.keys())
== Look and landing
Camera bob amplitudes are .024 m vertical, .013 m horizontal, multiplied by 1.9 sprinting or .22 while aiming. Keep the camera inside the capsule and blend eye height at rate 11/s. Jump launch speed is sqrt(2 × 21 × 1) ≈ 6.481 m/s. Terminal downward speed is 40 m/s. Crouch cannot end under a ceiling. Landing shake starts above 7.5 m/s at min(.5,(impact−7.5) × .06); the landing spring receives a downward velocity impulse min(impact × .36,3.2).
]
])
#sheet("21", "Combat volumes and scene direction", "Hit feedback, warehouse access and presentation anchors", [
#cols[
== Damage resolution
Player shots stop at the nearest world surface or enemy volume; there is no penetration or damage falloff. Hound optics use a .155 m radius weak-point sphere at the animated head: damage multiplier 2.2 (57.2 per PAC-9 hit). Body targeting uses two longitudinal rounded segments, endpoints −.45 to +.47 m along body forward: upper radius .27 m at body center + .03 m; lower radius .25 m at 45% of body height. Use the closest intersected volume. Gull body is an ellipsoid centered .03 m below its root, horizontal radius .5 m and vertical radius .5/2.3 m; no weak point.

Hound bullets test the player around feet +1 m, with horizontal hit tolerance .38 m and vertical center .95 m ±.95; world obstruction wins. Gull bolts sweep against a .36 m radius segment from feet +.35 to feet +1.5 m (crouching +1 m). Distinguish ordinary, weak-point and lethal hitmarkers; count one hit per damaging player shot.

Explosions require line of sight and linear falloff. Enemies receive base damage × (1−distance/radius), measured to root position, with visibility tested at root +.5 m. Player damage is half that base, measured and visibility-tested at feet +1 m. Thus a gull crash has maximum player damage 15, despite its 30 base damage against enemies. Explosions can cause enemy chain destruction.

== Warehouse and perimeter
Warehouse walls are .25 m thick. Main south-facing opening at Z −11 spans X −6…−1, height 5 m. East opening at X 12 spans Z −22…−17, height 4.5 m. Preserve clear approaches to both and the central relay (−2,.03,−19.5). Lower wall sections are concrete to 1.2 m, corrugated cladding above; pitched corrugated roof rises from 7 m eaves to 9.5 m at Z −19.5. Epoxy floor is Y .03. Shelf rows and stored goods leave circulation around relay and doors.

Chain-link perimeter follows north Z −30 and south Z 28 from X −38 to 30; west fence X −38 has its gate opening at Z 2…8. Fence physical height is 2.4 m with a 7 m player barrier to prevent climbing out. The open sliding gate rests centered Z 11.3, X −37.72. Guard booth at (−35.2,0,.35), raised gate arm at (−36.3,0,5) and concrete barriers make deployment read as a checkpoint. The west gate also has a player-only blocker across its opening: the mission remains inside the berth.

== Default controls
WASD move, mouse look, left mouse/Enter fire, right mouse aim, Shift sprint, C or Control crouch, Space jump, R reload, hold E interact, Escape/P pause. Enter begins deployment from title. Arrow keys offer turning. Keep displayed keyboard, gamepad and touch prompts aligned with their active device. Bound melee/alternate inputs have no additional player combat action; PAC-9 is the mission weapon.

== Palette and lighting
Use low warm sun against cold storm shadows. Sun direction is normalized (.916,.225,.331), linear RGB (1,.72,.42), intensity 9. Sky zenith is (.055,.085,.13), sunward horizon (.95,.55,.28), opposite horizon (.16,.22,.3); fog (.42,.47,.52). These are lighting values, not flat display colors. Preserve wet specular asphalt, pale corrugated metal, rust streaks, orange/teal container contrast and dark seawater.

Relay lights, gull searchlights and robot lenses carry gameplay contrast through rain. Warehouse shelter suppresses rain and changes reverberation; outside shots have an open tail, interior shots a denser reflected tail. Rotor and servo pitch follow motion; warning sounds begin with visible attack telegraphs. Magazine-out, insertion and bolt audio follow the exact reload milestones. Radio announces deployment, first contact, relay progress, final extraction instruction, low health once, rescue and completion; it must not obscure warning cues.

== Flight clearance detail
Downward feelers reach 2.4 m and add upward avoidance (2.4−hit distance) × 10. Upward feelers reach 4 m and set ceiling target to hit Y −1.3 m; refresh .35 s on that feeler cycle. Avoidance decays exponentially at 6/s. Below half health, oscillatory force amplitudes 5/3/5 on X/Y/Z destabilize flight while smoke runs every .16 s.
]
])
#sheet("22", "Tactical positioning", "Hound firing positions and player movement qualifications", [
#cols[
== Choosing firing positions
Consider the authored cover positions below with score bias −2; seven random walkable positions within 18 m of the player with bias 0; and five positions 9–17 m from the player, at angles within ±1.3 rad of the current player-to-hound bearing, with bias +1. Reject unwalkable positions and distances outside 8–18 m.

Minimize .9 × travel distance + bias + uniform(0,3). For each other engaged hound add 16 if within 3 m of its current/intended firing position; add `14*(directionDot-.86)/.14` when both attack directions from player have dot > .86. Add 22 if the candidate has no sight from Y +.95 m to player eye. If none qualify, stay at the current position.

Navigate around world cover. Stop within .45 m of the goal. For yaw error above 1.1 rad stop to turn; otherwise speed cap is min(requestedSpeed × (1−yawError × .5), .4 + goalDistance × 2.6). Maximum turn rate is 2.5 rad/s, doubled for lunges, following desired turn velocity at 10/s. Patrol waits 1.5–4.5 s between destinations. Investigation approaches last known position until within 2.2 m; use 2 m/s within 8 m, otherwise 4.5 m/s, then scan yaw with sin(scanTime × .8) × .9 rad.

== Movement qualifications
Sprint requires forward input > .55, lateral magnitude < .6, no crouch, aim blend < .35 and no firing. Sprint presentation begins grounded above 60% walk speed and persists in air if already sprinting. Aim interpolates movement speed toward min(current speed,2.4); backwards input scales speed toward .82 at full reverse. Ground input acceleration is 34, braking 26; airborne acceleration 9 and no-input drift deceleration .6 m/s². Horizontal velocity approaches the desired vector by at most acceleration × elapsed seconds.

== Authored cover schedule
All values are X/Y/Z metres. Repair blocked ground-level points by testing radii .5,1,1.5,2 m in eight directions at angle increments .7854 rad; take the first walkable position. Apply the same repair to ground spawn candidates. Elevated points above .2 m are retained.
#text(size:8pt)[#table(columns:(1fr,1fr),table.header([*X / Y / Z*],[*X / Y / Z*]), ..d.coverPoints.map(p => [#p.map(str).join(" / ")]))]
]
])
