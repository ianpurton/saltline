use bevy::prelude::*;
use saltline_game::{Enemy, Mission, Mode};
#[derive(Resource)]
pub struct DebrisAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}
#[derive(Component)]
pub struct Spark {
    velocity: Vec3,
    life: f32,
}
/// Presentation only: enemy health remains owned by the existing simulation.
pub fn combat_effects(
    mut c: Commands,
    time: Res<Time>,
    mission: Res<Mission>,
    assets: Res<DebrisAssets>,
    enemies: Query<(Entity, &Enemy, &Transform), Without<Spark>>,
    mut sparks: Query<(Entity, &mut Spark, &mut Transform)>,
    mut destroyed: Local<Vec<Entity>>,
    mut previous_elapsed: Local<f32>,
) {
    if mission.mode == Mode::Menu || mission.elapsed < *previous_elapsed {
        destroyed.clear();
    }
    *previous_elapsed = mission.elapsed;
    for (entity, enemy, t) in &enemies {
        if enemy.health > 0. || destroyed.contains(&entity) {
            continue;
        }
        destroyed.push(entity);
        for i in 0..18 {
            let angle = i as f32 * 2.39996;
            let velocity = Vec3::new(angle.cos() * 3., 2. + (i % 4) as f32, angle.sin() * 3.);
            c.spawn((
                Spark {
                    velocity,
                    life: 0.45 + (i % 5) as f32 * 0.12,
                },
                Mesh3d(assets.mesh.clone()),
                MeshMaterial3d(assets.material.clone()),
                Transform::from_translation(t.translation + Vec3::Y * 0.8)
                    .with_scale(Vec3::new(0.018, 0.018, 0.13)),
            ));
        }
    }
    for (e, mut spark, mut t) in &mut sparks {
        let dt = time.delta_secs();
        spark.life -= dt;
        if spark.life <= 0. {
            c.entity(e).despawn();
            continue;
        }
        spark.velocity.y -= 9.8 * dt;
        t.translation += spark.velocity * dt;
        if t.translation.y < 0.05 {
            t.translation.y = 0.05;
            spark.velocity.y = spark.velocity.y.abs() * 0.35;
        }
        t.rotation = Quat::from_rotation_arc(Vec3::Z, spark.velocity.normalize_or_zero());
        t.scale.x = 0.018 * spark.life.min(1.);
    }
}
