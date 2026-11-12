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
#[derive(Component)]
pub struct RigOwner(Entity);
pub fn bind_rigs(
    mut c: Commands,
    parts: Query<(Entity, &ChildOf), Or<(With<Leg>, With<Knee>, With<Rotor>)>>,
    parents: Query<&ChildOf>,
    roots: Query<(), With<Enemy>>,
) {
    for (e, parent) in &parts {
        let mut root = parent.parent();
        while !roots.contains(root) {
            let Ok(parent) = parents.get(root) else { break };
            root = parent.parent();
        }
        if roots.contains(root) {
            c.entity(e).insert(RigOwner(root));
        }
    }
}
pub fn rigs(
    clock: Res<PresentationClock>,
    mut roots: Query<(&Enemy, &Transform, &mut RigMotion)>,
    mut parts: Query<
        (
            &mut Transform,
            Option<&Leg>,
            Option<&Knee>,
            Option<&Rotor>,
            &RigOwner,
        ),
        Without<Enemy>,
    >,
    performance: Option<Res<saltline_game::performance::PerformanceSettings>>,
    diagnostics: Option<bevy::diagnostic::Diagnostics>,
) {
    let _trace = info_span!("presentation::rigs").entered();
    let _cpu = saltline_game::performance::CpuMeasurement::new(
        performance.as_deref(),
        diagnostics,
        "saltline/system/presentation/rigs_ms",
    );
    for (_, t, mut motion) in &mut roots {
        if clock.reset {
            *motion = RigMotion::default();
        }
        let movement = motion.last.map(|p| t.translation.distance(p)).unwrap_or(0.);
        if clock.dt > 0. {
            motion.speed = (movement / clock.dt).min(5.);
            motion.phase += movement * 3.2;
        }
        motion.last = Some(t.translation);
    }
    if clock.dt == 0. {
        return;
    }
    for (mut t, leg, knee, rotor, owner) in &mut parts {
        let Ok((enemy, _, motion)) = roots.get(owner.0) else {
            continue;
        };
        if enemy.health <= 0. {
            continue;
        }
        let strength = (motion.speed / 3.1).min(1.);
        if let Some(leg) = leg {
            t.rotation = Quat::from_rotation_x((motion.phase + leg.phase).sin() * 0.28 * strength);
        }
        if let Some(knee) = knee {
            t.rotation =
                Quat::from_rotation_x((motion.phase + knee.phase + 0.8).sin() * 0.21 * strength);
        }
        if let Some(rotor) = rotor {
            t.rotate_y(clock.dt * rotor.0 * 47.);
        }
    }
}

/// Cosmetic motion is isolated from the authoritative aim and damage calculation.
#[derive(Resource, Default)]
pub struct WeaponMotion {
    pub ads: f32,
    pub recoil: f32,
    pub kick: f32,
    pub phase: f32,
    pub land: f32,
    pub hit: f32,
    previous_vertical: f32,
    pub previous_position: Option<Vec3>,
}
pub fn weapon(
    clock: Res<PresentationClock>,
    m: Res<Mission>,
    a: Res<saltline_game::Actions>,
    mut events: MessageReader<saltline_game::PresentationEvent>,
    mut motion: ResMut<WeaponMotion>,
    mut projections: Query<&mut Projection, With<crate::title::GameplayCamera>>,
) {
    if clock.reset || m.mode == Mode::Menu {
        *motion = WeaponMotion::default();
        events.clear();
    }
    if clock.dt == 0. {
        if crate::capture::option("--inspect").as_deref() == Some("aim") {
            for mut projection in &mut projections {
                if let Projection::Perspective(p) = &mut *projection {
                    p.fov = 54_f32.to_radians();
                }
            }
        }
        return;
    }
    let blend = 1. - (-clock.dt * 14.).exp();
    motion.ads += (a.aim as u32 as f32 - motion.ads) * blend;
    motion.recoil *= (-clock.dt * 13.).exp();
    motion.kick *= (-clock.dt * 22.).exp();
    motion.land *= (-clock.dt * 15.).exp();
    motion.hit = (motion.hit - clock.dt).max(0.);
    if motion.previous_vertical < -1. && m.vertical == 0. {
        motion.land = (-motion.previous_vertical * 0.006).min(0.08);
    }
    motion.previous_vertical = m.vertical;
    if let Some(previous) = motion.previous_position {
        motion.phase +=
            m.position.xz().distance(previous.xz()).min(0.3) * if a.sprint { 3.8 } else { 3.2 };
    }
    motion.previous_position = Some(m.position);
    for event in events.read() {
        match event {
            saltline_game::PresentationEvent::Shot { .. } => {
                motion.recoil = (motion.recoil + 0.018).min(0.065);
                motion.kick = 0.055;
            }
            saltline_game::PresentationEvent::Impact { .. } => motion.hit = 0.12,
            _ => {}
        }
    }
    for mut projection in &mut projections {
        if let Projection::Perspective(p) = &mut *projection {
            p.fov = (74. - 20. * motion.ads).to_radians();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aiming_and_recoil_change_presentation_without_changing_authoritative_aim() {
        let mut app = App::new();
        app.add_message::<saltline_game::PresentationEvent>()
            .insert_resource(PresentationClock {
                dt: 1. / 60.,
                ..default()
            })
            .insert_resource(Mission {
                mode: Mode::Playing,
                ..default()
            })
            .insert_resource(saltline_game::Actions {
                aim: true,
                ..default()
            })
            .init_resource::<WeaponMotion>()
            .add_systems(Update, weapon);
        let camera = app
            .world_mut()
            .spawn((
                crate::title::GameplayCamera,
                Projection::Perspective(PerspectiveProjection::default()),
            ))
            .id();
        app.world_mut()
            .write_message(saltline_game::PresentationEvent::Shot {
                origin: Vec3::ZERO,
                direction: Vec3::NEG_Z,
            });
        for _ in 0..60 {
            app.update();
        }
        let motion = app.world().resource::<WeaponMotion>();
        assert!(motion.ads > 0.99 && motion.recoil < 0.001);
        let Projection::Perspective(projection) = app.world().get::<Projection>(camera).unwrap()
        else {
            panic!("perspective camera")
        };
        assert!((projection.fov.to_degrees() - 54.).abs() < 0.1);
        let mission = app.world().resource::<Mission>();
        assert_eq!((mission.pitch, mission.yaw, mission.ammo), (0., 0., 30));
        app.world_mut().resource_mut::<PresentationClock>().dt = 0.;
        app.world_mut().resource_mut::<saltline_game::Actions>().aim = false;
        app.update();
        assert!(app.world().resource::<WeaponMotion>().ads > 0.99);
    }
}

/// First-person meshes receive lighting but do not cast shadows into the port.
pub fn bind_viewmodel(mut c: Commands, visuals: Res<crate::Visuals>, children: Query<&Children>) {
    let mut stack = vec![visuals.rifle];
    while let Some(entity) = stack.pop() {
        c.entity(entity).insert(bevy::light::NotShadowCaster);
        if let Ok(children) = children.get(entity) {
            stack.extend(children.iter());
        }
    }
}

pub fn magazines(
    clock: Res<PresentationClock>,
    m: Res<Mission>,
    mut magazines: Query<&mut Transform, With<saltline_models::ReloadMagazine>>,
) {
    if clock.dt == 0. && !clock.reset && crate::capture::option("--inspect").is_none() {
        return;
    }
    let reload = if crate::capture::option("--inspect").as_deref() == Some("reload") {
        0.7
    } else {
        m.reload
    };
    let phase = (1. - reload / 1.8).clamp(0., 1.);
    let withdrawal = if reload > 0. {
        ((phase - 0.12) / 0.25).clamp(0., 1.) * (1. - ((phase - 0.58) / 0.22).clamp(0., 1.))
    } else {
        0.
    };
    for mut t in &mut magazines {
        t.translation = Vec3::new(0., -withdrawal * 0.26, withdrawal * 0.055);
    }
}
