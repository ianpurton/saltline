//! Bounded acceptance replay using the same Actions consumed by gameplay.
use bevy::prelude::*;
use saltline_game::*;
use saltline_world::{EXIT, RELAYS};
pub fn drive(
    mut c: Commands,
    mut m: ResMut<Mission>,
    mut a: ResMut<Actions>,
    enemies: Query<(&Enemy, &Transform)>,
    time: Res<Time<Real>>,
    mut frames: Local<Vec<f32>>,
) {
    if !saltline_game::launch_args().any(|x| x == "--mission-test") {
        return;
    }
    frames.push(time.delta_secs() * 1000.);
    if m.mode == Mode::Menu {
        m.mode = Mode::Playing;
    }
    if m.mode == Mode::Victory {
        frames.sort_by(|a, b| a.total_cmp(b));
        let n = frames.len();
        if n > 0 {
            info!(
                "FRAME TIMINGS debug optimized: samples={}, median_ms={:.2}, p95_ms={:.2}",
                n,
                frames[n / 2],
                frames[(n - 1) * 95 / 100]
            );
        }

        info!(
            "MISSION DRIVER PASSED: kills={}, seconds={:.2}",
            m.kills, m.elapsed
        );
        c.write_message(AppExit::Success);
        return;
    }
    assert!(m.mode != Mode::Dead, "mission driver died");
    assert!(m.elapsed < 480., "mission driver timed out");
    let target = RELAYS
        .iter()
        .enumerate()
        .find(|(i, _)| !m.relays[*i])
        .map(|(_, p)| *p)
        .unwrap_or(EXIT);
    let enemy = enemies
        .iter()
        .filter(|(e, _)| e.health > 0.)
        .filter(|(_, t)| t.translation.distance(m.position) < 20.)
        .min_by(|a, b| {
            a.1.translation
                .distance(m.position)
                .total_cmp(&b.1.translation.distance(m.position))
        });
    a.fire = false;
    a.reload = m.ammo < 5;
    a.interact = true;
    a.movement = Vec2::ZERO;
    if let Some((_, t)) = enemy {
        let d = t.translation + Vec3::Y * 0.65 - m.position;
        m.yaw = (-d.x).atan2(-d.z);
        m.pitch = d.y.atan2(d.xz().length());
        a.fire = true;
    } else {
        let d = target - m.position;
        m.yaw = (-d.x).atan2(-d.z);
        m.pitch = 0.;
        if d.xz().length() > 1.5 {
            a.movement = Vec2::Y;
        }
    }
}
