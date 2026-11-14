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
    mut benchmark: ResMut<crate::benchmark::Benchmark>,
    batches: Res<crate::benchmark::BatchCounters>,
    ready: Res<crate::capture::RenderReady>,
    port: Res<saltline_world::Port>,
    mut route: Local<Vec<Vec3>>,
    mut route_target: Local<Option<Vec3>>,
) {
    if !saltline_game::launch_args().any(|x| x == "--mission-test" || x == "--benchmark") {
        return;
    }
    if benchmark.enabled && (!ready.ready() || !benchmark.replay) {
        return;
    }
    if ready.ready() {
        frames.push(time.delta_secs() * 1000.);
    }
    if m.mode == Mode::Menu {
        m.mode = Mode::Playing;
    }
    if m.mode == Mode::Victory {
        frames.sort_by(|a, b| a.total_cmp(b));
        let n = frames.len();
        if n > 0 {
            info!(
                "FRAME TIMINGS: samples={}, median_ms={:.2}, p95_ms={:.2}",
                n,
                frames[n / 2],
                frames[(n - 1) * 95 / 100]
            );
        }

        info!(
            "DRAW SUBMISSIONS main opaque multidraw batches + transparent items = {} (before GPU culling; excludes shadow/UI passes)",
            batches.0.load(std::sync::atomic::Ordering::Relaxed)
        );
        info!(
            "MISSION DRIVER PASSED: kills={}, seconds={:.2}",
            m.kills, m.elapsed
        );
        if benchmark.enabled {
            benchmark.completed = true;
        } else {
            c.write_message(AppExit::Success);
        }
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
        .filter(|(_, t)| visible(m.position, t.translation + Vec3::Y * 0.65, &port))
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
    if *route_target != Some(target) {
        *route = port.route(m.feet(), target);
        *route_target = Some(target);
    }
    while route
        .first()
        .is_some_and(|p| p.xz().distance(m.position.xz()) < 0.25)
    {
        route.remove(0);
    }
    if let Some((_, t)) =
        enemy.filter(|(_, t)| visible(m.position, t.translation + Vec3::Y * 0.65, &port))
    {
        let d = t.translation + Vec3::Y * 0.65 - m.position;
        m.yaw = (-d.x).atan2(-d.z);
        m.pitch = d.y.atan2(d.xz().length());
        a.fire = true;
    } else {
        let waypoint = route.first().copied().unwrap_or(target);
        let d = waypoint - m.position;
        m.yaw = (-d.x).atan2(-d.z);
        m.pitch = 0.;
        if !route.is_empty() || (target - m.position).xz().length() > 1.5 {
            a.movement = Vec2::Y * (d.xz().length() / (3.8 * time.delta_secs().max(0.001))).min(1.);
        }
    }
}
