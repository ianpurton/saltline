# The original berth

The playable world follows the construction manual in `docs/game-manual/`: sheets
02–03 and 17–21 define scale, structures, placement and traversal. The extracted
JavaScript asset builders remain construction references; the game builds native
Rust geometry. The old corridor layout and its title-only container stacks are
no longer spawned.

## Layout and ownership

`crates/world/src/layout.rs` is the editable, typed Rust placement schedule. It
contains 26 stacks (47 tiers), 232 prop placements, 42 spawn candidates, 55 cover
points, four gameplay zones and the original 12 cinematic camera anchors. Initial
transcription used the manual's derived reports (HTML SHA-256
`f34e6debbfa7526b72be2daa24595c2f195e102a7aede4f22e59138c99cd6852`). The runtime does
not load the manual JSON or execute JavaScript. Changes to the specification need
an explicit corresponding layout update; neither asset extraction nor a manual
build automatically rewrites the game.

Coordinates are metres, Y-up, with the original X/Z orientation. Player forward
is -Z before yaw; robot geometry faces +Z. Deployment looks into the yard from
the west gate. Standard containers are 6.06 or 12.19 m long along X, 2.438 m wide
and 2.59 m high; high cubes are 2.9 m high. Tier elevations accumulate their actual
heights, and authored dx/dz offsets remain in world axes. Placement jitter uses
a fixed Mulberry32 seed of 1337. Detail toggles do not alter collision.

| Anchor | Ground X / Y / Z |
| --- | --- |
| Deployment | -32 / 0 / 6 |
| Relay A | 1.5 / 0 / 7.5 |
| Relay B | -2 / 0.03 / -19.5 |
| Relay C | 23 / 0 / 0 |
| Extraction | 29 / 0 / 20 |

The warehouse occupies X -16..12, Z -28..-11, with its floor at .03 m, eaves at
7 m and ridge at 9.5 m. Its south and east doorways remain open. The quay edge is
X 30; the ladder and rescue boat approach are at Z 20. The gantry straddles rails
at X 18.5 and 28.5. The freighter lies outside the playable berth. Gate fences,
quay markings, shelf rows, cargo dressing and elevated cover are separate from
these structural envelopes.

## Traversal

`Port` owns yaw-oriented collision boxes with independent player, enemy, bullet
and navigation flags. Ground support and sight lines use those volumes rather
than render meshes. Thin decoration does not add arbitrary navigation walls.
Player-only anti-climb and gate barriers do not block bullets or enemy navigation.
The pitched warehouse roof uses a conservative overhead collision envelope;
its roof is not a player route.

The existing movement system now supports ground heights, 20 cm stair risers,
container roofs, the catwalk, crouch clearance, ceilings and falling. Mission
position remains the eye position; `Mission::feet()` centralizes the conversion.
The player can walk up the stairs from (-2.2,0,12.42), cross the blue roof and
catwalk, and reach the green container roof. Extraction is ground-side, with no
ladder-climbing requirement. Dropping 2.2 m below sea level returns the player to
the last safe grounded position inland of X 28.8, resets velocity, rotates yaw by
pi and applies 12 damage. Hard landings use the manual's impact damage rule.

Initial enemies use repaired authored ground candidates and unchanged air
altitudes, retaining ten enemies and the existing combat behavior. The full
reference reinforcement, tactical AI, weapons and scoring systems are outside
this map change. The mission replay follows a cached ground route around cover
instead of walking directly through obstacles toward each relay.

## Inspecting the map

Use the existing offscreen renderer; these captures need neither DISPLAY nor Xvfb:

```sh
cargo build --release -p saltline-game
target/release/saltline-game --headless-render --inspect port --no-audio --output-dir tmp/screenshots/berth
target/release/saltline-game --headless-render --inspect elevated --no-audio --output-dir tmp/screenshots/berth
```

Other views include deployment, corridor (the yard), warehouse, waterfront and
extraction. `map`, `yard` and `quay` are aliases for the corresponding inspection
views. The title camera follows the authored berth tour; opening Graphics holds
its position for comparison. Native and browser builds share the map while
retaining their platform-specific rendering paths.

Benchmark stage identifiers and artifact format remain compatible, but camera
locations and the scene workload have changed. Old corridor timing results are
not directly comparable. The renderer and performance acceptance threshold are
unchanged. Native materials and small construction details are interpretations
of the reference, not a claim of pixel-identical rendering.
