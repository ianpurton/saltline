//! Reference harbor motion, isolated from gameplay and driven by presentation time.
use crate::presentation::PresentationClock;
use bevy::{
    asset::RenderAssetUsages, mesh::VertexAttributeValues, prelude::*,
    render::render_resource::PrimitiveTopology,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Lamp {
    position: [f32; 3],
    radius: f32,
    color: [f32; 3],
    period: f32,
    phase: f32,
    duty: f32,
}
#[derive(Deserialize)]
struct Gull {
    center: [f32; 3],
    radius: f32,
    speed: f32,
    phase: f32,
    flap: f32,
}
#[derive(Resource, Deserialize)]
struct Backdrop {
    lamps: Vec<Lamp>,
    gulls: Vec<Gull>,
    #[serde(skip)]
    meshes: Option<(Handle<Mesh>, Handle<Mesh>)>,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, animate.after(crate::presentation::clock));
}
fn mesh(positions: Vec<[f32; 3]>, colors: Vec<[f32; 4]>) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; positions.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh
}
fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut data: Backdrop =
        serde_json::from_str(include_str!("../../../assets/models/skyline.json"))
            .expect("bundled skyline metadata must decode");
    let gulls = meshes.add(mesh(
        gull_positions(&data.gulls, 1.),
        vec![[1.; 4]; data.gulls.len() * 6],
    ));
    let mut positions = Vec::new();
    let axes = [
        Vec3::X,
        Vec3::Y,
        Vec3::Z,
        Vec3::NEG_X,
        Vec3::NEG_Y,
        Vec3::NEG_Z,
    ];
    // The reference lamps are octahedra; retain their exact sizes and HDR colors.
    for lamp in &data.lamps {
        for face in [
            [1, 0, 2],
            [1, 2, 3],
            [1, 3, 5],
            [1, 5, 0],
            [4, 2, 0],
            [4, 3, 2],
            [4, 5, 3],
            [4, 0, 5],
        ] {
            for vertex in face {
                positions.push((Vec3::from(lamp.position) + axes[vertex] * lamp.radius).to_array());
            }
        }
    }
    let lamps = meshes.add(mesh(positions, lamp_colors(&data.lamps, 1.)));
    for (name, handle, color) in [
        (
            "harbor gulls",
            gulls.clone(),
            Color::linear_rgb(0.2, 0.21, 0.23),
        ),
        ("harbor lamps", lamps.clone(), Color::WHITE),
    ] {
        c.spawn((
            Name::new(name),
            Mesh3d(handle),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                double_sided: true,
                cull_mode: None,
                fog_enabled: false,
                ..default()
            })),
            Transform::IDENTITY,
            bevy::camera::visibility::NoFrustumCulling,
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
        ));
    }
    data.meshes = Some((gulls, lamps));
    c.insert_resource(data);
}
fn smoothstep(a: f32, b: f32, value: f32) -> f32 {
    let x = ((value - a) / (b - a)).clamp(0., 1.);
    x * x * (3. - 2. * x)
}
fn lamp_colors(lamps: &[Lamp], time: f32) -> Vec<[f32; 4]> {
    lamps
        .iter()
        .flat_map(|lamp| {
            let phase = (time / lamp.period.max(0.001) + lamp.phase).fract();
            let brightness = if lamp.period <= 0. {
                1.
            } else {
                0.02 + 0.98
                    * smoothstep(0., 0.06, phase)
                    * (1. - smoothstep(lamp.duty, lamp.duty + 0.12, phase))
            };
            std::iter::repeat_n(
                [
                    lamp.color[0] * brightness,
                    lamp.color[1] * brightness,
                    lamp.color[2] * brightness,
                    1.,
                ],
                24,
            )
        })
        .collect()
}
fn gull_positions(gulls: &[Gull], time: f32) -> Vec<[f32; 3]> {
    let vertices = [
        [0.15, 0., 0.],
        [-0.13, 0., 0.],
        [-0.24, 0.8, 1.],
        [0.15, 0., 0.],
        [-0.24, -0.8, 1.],
        [-0.13, 0., 0.],
    ];
    gulls
        .iter()
        .flat_map(|gull| {
            let angle = gull.phase + time * gull.speed;
            let direction = gull.speed.signum();
            let forward = Vec3::new(-angle.sin(), 0., angle.cos()) * direction;
            let right = Vec3::new(forward.z, 0., -forward.x);
            let glide = smoothstep(-0.2, 0.5, (time * 0.31 + gull.phase * 3.).sin());
            let flap =
                (time * gull.flap + gull.phase * 7.).sin() * (0.12 + (0.55 - 0.12) * glide) + 0.18;
            let center = Vec3::from(gull.center)
                + Vec3::new(
                    angle.cos() * gull.radius,
                    (angle * 0.7 + gull.phase).sin() * 2.2 + (time * 0.23 + gull.phase).sin() * 1.5,
                    angle.sin() * gull.radius,
                );
            vertices.map(|[x, z, wing]| {
                let y = wing * flap * z.abs();
                let z = z * (1. - 0.14 * wing * flap.abs());
                (center + forward * x + right * z + Vec3::Y * (y - 0.3 * direction * z)).to_array()
            })
        })
        .collect()
}
fn animate(
    clock: Res<PresentationClock>,
    data: Res<Backdrop>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut previous: Local<Option<f32>>,
) {
    if *previous == Some(clock.elapsed) {
        return;
    }
    *previous = Some(clock.elapsed);
    let Some((gulls, lamps)) = &data.meshes else {
        return;
    };
    if let Some(mut mesh) = meshes.get_mut(gulls) {
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            *positions = gull_positions(&data.gulls, clock.elapsed);
        }
    }
    if let Some(mut mesh) = meshes.get_mut(lamps) {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            lamp_colors(&data.lamps, clock.elapsed),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wing_deformation_and_banking_match_reference_at_zero() {
        for direction in [1., -1.] {
            let gull = Gull {
                center: [0.; 3],
                radius: 10.,
                speed: direction,
                phase: 0.,
                flap: 6.,
            };
            let vertices = gull_positions(&[gull], 0.);
            let span = 0.8 * (1. - 0.14 * 0.18);
            let expected = Vec3::new(
                10. + direction * span,
                0.18 * 0.8 - 0.3 * direction * span,
                -0.24 * direction,
            );
            assert!(Vec3::from(vertices[2]).distance(expected) < 1e-5);
        }
    }
    #[test]
    fn reference_motion_and_blink_endpoints() {
        let data: Backdrop =
            serde_json::from_str(include_str!("../../../assets/models/skyline.json")).unwrap();
        assert_eq!(data.gulls.len(), 12);
        assert!(data.gulls.iter().any(|g| g.speed > 0.) && data.gulls.iter().any(|g| g.speed < 0.));
        for time in [0., 1., 30., 100.] {
            let positions = gull_positions(&data.gulls, time);
            assert_eq!(positions.len(), 72);
            assert!(positions.iter().flatten().all(|v| v.is_finite()));
            for (gull, bird) in data.gulls.iter().zip(positions.chunks_exact(6)) {
                let angle = gull.phase + time * gull.speed;
                let center = Vec3::from(gull.center)
                    + Vec3::new(
                        angle.cos() * gull.radius,
                        (angle * 0.7 + gull.phase).sin() * 2.2
                            + (time * 0.23 + gull.phase).sin() * 1.5,
                        angle.sin() * gull.radius,
                    );
                let forward = Vec3::new(-angle.sin(), 0., angle.cos()) * gull.speed.signum();
                assert!(Vec3::from(bird[0]).distance(center + forward * 0.15) < 1e-4);
                assert_eq!(bird[0], bird[3]);
            }
        }
        let lamps = [Lamp {
            position: [0.; 3],
            radius: 1.,
            color: [8., 0.45, 0.25],
            period: 2.,
            phase: 0.,
            duty: 0.45,
        }];
        assert!((lamp_colors(&lamps, 0.)[0][0] - 0.16).abs() < 1e-5);
        assert_eq!(lamp_colors(&lamps, 0.5)[0][0], 8.);
        assert!((lamp_colors(&lamps, 1.8)[0][0] - 0.16).abs() < 1e-5);
    }
}
