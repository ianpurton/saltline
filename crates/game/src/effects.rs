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
}
#[derive(Component)]
pub struct Particle {
    velocity: Vec3,
    life: f32,
    total: f32,
    smoke: bool,
}
pub fn combat_effects(
    mut c: Commands,
    clock: Res<PresentationClock>,
    mission: Res<Mission>,
    assets: Res<DebrisAssets>,
    mut messages: MessageReader<PresentationEvent>,
    mut particles: Query<(Entity, &mut Particle, &mut Transform)>,
) {
    if clock.reset || mission.mode == Mode::Menu {
        for (e, _, _) in &mut particles {
            c.entity(e).despawn();
        }
        messages.clear();
        return;
    }
    if clock.dt == 0. {
        return;
    }
    for event in messages.read() {
        let (position, count, smoke_count) = match *event {
            PresentationEvent::Shot { origin, direction } => {
                let side = direction.cross(Vec3::Y).normalize_or_zero();
                (
                    origin + direction * 1.2 + side * 0.15 - Vec3::Y * 0.11,
                    3,
                    2,
                )
            }
            PresentationEvent::Impact {
                position,
                destroyed,
            } => (
                position,
                if destroyed { 28 } else { 9 },
                if destroyed { 9 } else { 1 },
            ),
            PresentationEvent::RelayCut { position } => (position, 12, 3),
        };
        for i in 0..count + smoke_count {
            let smoke = i >= count;
            let a = i as f32 * 2.39996;
            let life = if smoke {
                0.6 + (i % 5) as f32 * 0.2
            } else {
                0.18 + (i % 7) as f32 * 0.06
            };
            c.spawn((
                Particle {
                    velocity: Vec3::new(a.cos(), if smoke { 0.5 } else { 1.8 }, a.sin())
                        * if smoke { 0.35 } else { 2.1 },
                    life,
                    total: life,
                    smoke,
                },
                Mesh3d(if smoke {
                    assets.sphere.clone()
                } else {
                    assets.mesh.clone()
                }),
                MeshMaterial3d(if smoke {
                    assets.smoke.clone()
                } else {
                    assets.material.clone()
                }),
                Transform::from_translation(position).with_scale(if smoke {
                    Vec3::splat(0.07)
                } else {
                    Vec3::new(0.012, 0.012, 0.075)
                }),
                bevy::light::NotShadowCaster,
            ));
        }
    }
    for (entity, mut p, mut t) in &mut particles {
        p.life -= clock.dt;
        if p.life <= 0. {
            c.entity(entity).despawn();
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
        assert!(app.world().get_entity(particle).is_err());
    }
}
