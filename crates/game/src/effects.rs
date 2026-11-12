//! Bounded cosmetic particles driven only by confirmed simulation outcomes.
use crate::presentation::PresentationClock;
use bevy::prelude::*;
use saltline_game::{Mission, Mode, PresentationEvent};
#[derive(Resource)]
pub struct DebrisAssets {
    pub mesh: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub smoke: Handle<StandardMaterial>,
    pub glow: Handle<StandardMaterial>,
}
#[derive(Component)]
pub struct Particle {
    velocity: Vec3,
    life: f32,
    total: f32,
    smoke: bool,
}
pub const PARTICLE_CAPACITY: usize = 256;
pub fn setup(mut c: Commands, assets: Res<DebrisAssets>) {
    for _ in 0..PARTICLE_CAPACITY {
        c.spawn((
            Particle {
                velocity: Vec3::ZERO,
                life: 0.,
                total: 1.,
                smoke: false,
            },
            Mesh3d(assets.mesh.clone()),
            MeshMaterial3d(assets.material.clone()),
            Transform::from_scale(Vec3::ZERO),
            Visibility::Hidden,
            bevy::light::NotShadowCaster,
        ));
    }
}
pub fn combat_effects(
    clock: Res<PresentationClock>,
    mission: Res<Mission>,
    assets: Res<DebrisAssets>,
    mut messages: MessageReader<PresentationEvent>,
    mut particles: Query<(
        &mut Particle,
        &mut Transform,
        &mut Visibility,
        &mut Mesh3d,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
    performance: Option<Res<saltline_game::performance::PerformanceSettings>>,
    diagnostics: Option<bevy::diagnostic::Diagnostics>,
) {
    let _trace = info_span!("effects::combat_effects").entered();
    let _cpu = saltline_game::performance::CpuMeasurement::new(
        performance.as_deref(),
        diagnostics,
        "saltline/system/effects/combat_effects_ms",
    );
    if clock.reset || mission.mode == Mode::Menu {
        for (mut p, mut t, mut v, _, _) in &mut particles {
            p.life = 0.;
            t.scale = Vec3::ZERO;
            *v = Visibility::Hidden;
        }
        messages.clear();
        return;
    }
    if clock.dt == 0. {
        return;
    }
    for (mut p, mut t, mut v, _, _) in &mut particles {
        if p.life <= 0. {
            continue;
        }
        p.life -= clock.dt;
        if p.life <= 0. {
            *v = Visibility::Hidden;
            t.scale = Vec3::ZERO;
            continue;
        }
        if !p.smoke {
            p.velocity.y -= clock.dt * 9.8;
        }
        t.translation += p.velocity * clock.dt;
        if t.translation.y < 0.035 {
            t.translation.y = 0.035;
            p.velocity.y = p.velocity.y.abs() * 0.25;
        }
        if p.smoke {
            let phase = 1. - p.life / p.total;
            t.scale = Vec3::splat((0.08 + phase * 0.35) * (p.life / p.total).sqrt());
        } else {
            t.rotation = Quat::from_rotation_arc(Vec3::Z, p.velocity.normalize_or_zero());
            t.scale.x = 0.012 * (p.life / p.total);
        }
    }
    for event in messages.read() {
        let (position, count, smoke_count, shot) = match *event {
            PresentationEvent::Shot { origin, direction } => (
                origin + direction * 1.2 + direction.cross(Vec3::Y) * 0.15 - Vec3::Y * 0.11,
                3,
                2,
                Some(direction),
            ),
            PresentationEvent::Impact {
                position,
                destroyed,
            } => (
                position,
                if destroyed { 28 } else { 9 },
                if destroyed { 9 } else { 1 },
                None,
            ),
            PresentationEvent::RelayCut { position } => (position, 12, 3, None),
        };
        let mut free = particles.iter_mut().filter(|(p, _, _, _, _)| p.life <= 0.);
        for i in 0..count + smoke_count {
            let Some((mut p, mut t, mut v, mut mesh, mut material)) = free.next() else {
                break;
            };
            let smoke = i >= count;
            let angle = i as f32 * 2.39996;
            let life = if smoke {
                0.6 + (i % 5) as f32 * 0.2
            } else {
                0.18 + (i % 7) as f32 * 0.06
            };
            let tracer = shot.is_some() && i == 0;
            let life = if tracer { 0.055 } else { life };
            *p = Particle {
                velocity: if tracer {
                    shot.unwrap_or(Vec3::NEG_Z) * 100.
                } else {
                    Vec3::new(angle.cos(), if smoke { 0.5 } else { 1.8 }, angle.sin())
                        * if smoke { 0.35 } else { 2.1 }
                },
                life,
                total: life,
                smoke,
            };
            *t = Transform::from_translation(position).with_scale(if smoke {
                Vec3::splat(0.07)
            } else {
                Vec3::new(0.012, 0.012, if tracer { 1.8 } else { 0.075 })
            });
            if tracer {
                t.rotation = Quat::from_rotation_arc(Vec3::Z, shot.unwrap_or(Vec3::NEG_Z));
            }
            mesh.0 = if smoke {
                assets.sphere.clone()
            } else {
                assets.mesh.clone()
            };
            material.0 = if smoke {
                assets.smoke.clone()
            } else if tracer {
                assets.glow.clone()
            } else {
                assets.material.clone()
            };
            *v = Visibility::Inherited;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn particles_freeze_on_pause_and_are_cleared_on_restart() {
        let mut app = App::new();
        app.add_message::<PresentationEvent>()
            .init_resource::<PresentationClock>()
            .insert_resource(Mission {
                mode: Mode::Paused,
                ..default()
            })
            .insert_resource(DebrisAssets {
                mesh: default(),
                sphere: default(),
                material: default(),
                smoke: default(),
                glow: default(),
            })
            .add_systems(Update, combat_effects);
        let particle = app
            .world_mut()
            .spawn((
                Particle {
                    velocity: Vec3::X,
                    life: 0.5,
                    total: 0.5,
                    smoke: false,
                },
                Transform::from_xyz(1., 2., 3.),
                Visibility::Inherited,
                Mesh3d::default(),
                MeshMaterial3d::<StandardMaterial>::default(),
            ))
            .id();
        app.update();
        assert_eq!(
            app.world().get::<Transform>(particle).unwrap().translation,
            Vec3::new(1., 2., 3.)
        );
        assert_eq!(app.world().get::<Particle>(particle).unwrap().life, 0.5);
        app.world_mut().resource_mut::<PresentationClock>().reset = true;
        app.world_mut().resource_mut::<Mission>().mode = Mode::Playing;
        app.update();
        assert_eq!(app.world().get::<Particle>(particle).unwrap().life, 0.);
        assert_eq!(
            *app.world().get::<Visibility>(particle).unwrap(),
            Visibility::Hidden
        );
    }
    #[test]
    fn event_bursts_cannot_grow_the_particle_pool() {
        let mut app = App::new();
        let mut clock = PresentationClock::default();
        clock.dt = 1. / 60.;
        app.add_message::<PresentationEvent>()
            .insert_resource(clock)
            .insert_resource(Mission {
                mode: Mode::Playing,
                ..default()
            })
            .insert_resource(DebrisAssets {
                mesh: default(),
                sphere: default(),
                material: default(),
                smoke: default(),
                glow: default(),
            })
            .add_systems(Startup, setup)
            .add_systems(Update, combat_effects);
        app.update();
        for _ in 0..100 {
            app.world_mut().write_message(PresentationEvent::Impact {
                position: Vec3::Y,
                destroyed: true,
            });
        }
        app.update();
        assert_eq!(
            app.world_mut()
                .query::<&Particle>()
                .iter(app.world())
                .count(),
            PARTICLE_CAPACITY
        );
        assert!(
            app.world_mut()
                .query::<&Particle>()
                .iter(app.world())
                .all(|p| p.life > 0.)
        );
        app.world_mut().resource_mut::<PresentationClock>().reset = true;
        app.update();
        assert!(
            app.world_mut()
                .query::<&Particle>()
                .iter(app.world())
                .all(|p| p.life == 0.)
        );
    }
}
