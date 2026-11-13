//! Presentation time and procedural animation never change simulation state.
use bevy::prelude::*;
use saltline_game::{Enemy, Mission, Mode};
use saltline_models::{Knee, Leg, Rotor};
#[derive(Resource, Default)]
pub struct PresentationClock {
    pub elapsed: f32,
    pub dt: f32,
    pub reset: bool,
    previous: f32,
}
pub fn clock(
    time: Res<Time>,
    mission: Res<Mission>,
    ready: Res<crate::capture::RenderReady>,
    mut clock: ResMut<PresentationClock>,
) {
    clock.reset = mission.elapsed < clock.previous;
    clock.previous = mission.elapsed;
    let inspect = saltline_game::launch_args().any(|a| a == "--inspect");
    clock.dt = if inspect
        || (saltline_game::launch_args().any(|a| a == "--title-capture") && !ready.ready())
        || !matches!(mission.mode, Mode::Playing | Mode::Menu)
    {
        0.
    } else {
        time.delta_secs()
    };
    if clock.reset {
        clock.elapsed = 0.;
    }
    if inspect {
        clock.elapsed = 1.;
    } else {
        clock.elapsed += clock.dt;
    }
}
#[derive(Component, Default)]
pub struct RigMotion {
    last: Option<Vec3>,
    phase: f32,
    speed: f32,
}
pub fn rigs(
    clock: Res<PresentationClock>,
    mut roots: Query<(Entity, &Enemy, &Transform, &mut RigMotion)>,
    parents: Query<&ChildOf>,
    mut parts: Query<
        (
            &mut Transform,
            Option<&Leg>,
            Option<&Knee>,
            Option<&Rotor>,
            &ChildOf,
        ),
        Without<Enemy>,
    >,
) {
    let mut states = std::collections::HashMap::new();
    for (id, enemy, t, mut motion) in &mut roots {
        if clock.reset {
            motion.last = None;
            motion.phase = 0.;
            motion.speed = 0.;
        }
        let movement = motion.last.map(|p| t.translation.distance(p)).unwrap_or(0.);
        if clock.dt > 0. {
            motion.speed = (movement / clock.dt).min(5.);
            motion.phase += movement * 3.2;
        }
        motion.last = Some(t.translation);
        states.insert(id, (motion.phase, motion.speed, enemy.health > 0.));
    }
    for (mut t, leg, knee, rotor, parent) in &mut parts {
        let mut root = parent.parent();
        for _ in 0..5 {
            if states.contains_key(&root) {
                break;
            }
            let Ok(p) = parents.get(root) else { break };
            root = p.parent();
        }
        let Some(&(phase, speed, alive)) = states.get(&root) else {
            continue;
        };
        if !alive || clock.dt == 0. {
            continue;
        }
        let strength = (speed / 3.1).min(1.);
        if let Some(leg) = leg {
            t.rotation = Quat::from_rotation_x((phase + leg.phase).sin() * 0.28 * strength);
        }
        if let Some(knee) = knee {
            t.rotation = Quat::from_rotation_x((phase + knee.phase + 0.8).sin() * 0.21 * strength);
        }
        if let Some(rotor) = rotor {
            t.rotate_y(clock.dt * rotor.0 * 47.);
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub fn water(
    clock: Res<PresentationClock>,
    mut materials: ResMut<Assets<saltline_models::materials::wet::WetMaterial>>,
) {
    for (_, m) in materials.iter_mut() {
        m.extension.settings.x = clock.elapsed;
    }
}
