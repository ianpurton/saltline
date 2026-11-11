use bevy::prelude::*;
use saltline_models::Kit;
#[derive(Resource, Default)]
pub struct Port {
    pub walls: Vec<(Vec3, Vec3)>,
}
pub const RELAYS: [Vec3; 3] = [
    Vec3::new(-5., 0., -13.),
    Vec3::new(8., 0., -38.),
    Vec3::new(-6., 0., -61.),
];
pub const EXIT: Vec3 = Vec3::new(18., 0., -75.);
pub fn build(c: &mut Commands, k: &Kit) -> Port {
    let mut port = Port::default();
    let root = k.root(c, Vec3::ZERO);
    k.part(
        c,
        root,
        Vec3::new(-12., -0.25, -35.),
        Vec3::new(81., 0.5, 105.),
        0,
    );
    let ocean = k.part(
        c,
        root,
        Vec3::new(65., -0.55, -40.),
        Vec3::new(2400., 0.1, 2400.),
        8,
    );
    c.entity(ocean).insert(MeshMaterial3d(k.sea.clone()));
    for row in 0..8 {
        for side in [-1., 1.] {
            let x = side * (12. + (row % 2) as f32 * 3.);
            let z = -row as f32 * 11. - 9.;
            for stack in 0..if row % 3 == 0 { 3 } else { 2 } {
                k.container(
                    c,
                    Vec3::new(x, stack as f32 * 2.85, z),
                    1 + (row % 3) as usize,
                );
            }
            port.walls
                .push((Vec3::new(x, 2., z), Vec3::new(1.8, 4., 4.8)));
        }
    }

    // Worn loading-bay paint and crossing stripes sit flush on the quay.
    for z in [-3., -31., -64.] {
        for i in 0..9 {
            k.part(
                c,
                root,
                Vec3::new(-4. + i as f32, 0.012, z),
                Vec3::new(0.5, 0.015, 2.),
                4,
            );
        }
        for x in [-20., 20.] {
            k.part(
                c,
                root,
                Vec3::new(x, 0.012, z),
                Vec3::new(3., 0.015, 0.12),
                3,
            );
        }
    }
    for i in 0..10 {
        for layer in 0..2 {
            k.container(
                c,
                Vec3::new(
                    -41. - (i % 2) as f32 * 3.3,
                    layer as f32 * 2.85,
                    -4. - i as f32 * 9.5,
                ),
                1 + (i + layer) % 3,
            );
        }
    }
    for x in [-26., 26.] {
        k.part(
            c,
            root,
            Vec3::new(x, 0.12, -35.),
            Vec3::new(0.6, 0.24, 104.),
            9,
        );
    }
    // Road lane markings, drainage grates, puddles and bollards.
    for i in 0..36 {
        let z = 8. - i as f32 * 2.5;
        k.part(
            c,
            root,
            Vec3::new(0., 0.009, z),
            Vec3::new(0.11, 0.012, 1.1),
            3,
        );
        for x in [-23., 23.] {
            k.part(c, root, Vec3::new(x, 0.3, z), Vec3::new(0.18, 0.6, 0.18), 3);
        }
    }
    for i in 0..70 {
        let x = ((i * 37) % 47) as f32 - 23.;
        let z = 8. - ((i * 19) % 94) as f32;
        let puddle = c
            .spawn((
                Mesh3d(k.puddle.clone()),
                MeshMaterial3d(k.mats[8].clone()),
                Transform::from_xyz(x, 0.023, z)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::new(1. + (i % 5) as f32, 0.7 + (i % 3) as f32, 1.)),
            ))
            .id();
        c.entity(root).add_child(puddle);
    }
    for z in [-15., -45., -75.] {
        for x in [-22., 22.] {
            k.part(c, root, Vec3::new(x, 6., z), Vec3::new(0.18, 12., 0.18), 4);
            k.part(c, root, Vec3::new(x, 12., z), Vec3::new(2., 0.3, 0.5), 7);
            c.spawn((
                PointLight {
                    color: Color::srgb(1., 0.63, 0.29),
                    intensity: 160000.,
                    range: 26.,
                    ..default()
                },
                Transform::from_xyz(x, 11.5, z),
            ));
        }
    }

    // Match the new enclosing walls while leaving the loading bays open.
    for z in [-25., -55.] {
        port.walls
            .push((Vec3::new(-35., 2.5, z), Vec3::new(0.15, 2.5, 10.)));
        for dz in [-10., 10.] {
            port.walls
                .push((Vec3::new(-30., 2.5, z + dz), Vec3::new(5., 2.5, 0.15)));
        }
        for dz in [-7.5, 7.5] {
            port.walls
                .push((Vec3::new(-25., 2.5, z + dz), Vec3::new(0.15, 2.5, 2.5)));
        }
    }
    saltline_models::library::warehouse::build(c, k, root);
    saltline_models::library::crane::build(c, k, root);
    saltline_models::library::props::build(c, k, root);
    saltline_models::library::crane::details(c, k, root);
    saltline_models::library::ship::build(c, k, root);
    saltline_models::library::boat::build(c, k, root, EXIT);
    port
}
