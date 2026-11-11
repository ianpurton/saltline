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
        Vec3::new(0., -0.25, -35.),
        Vec3::new(55., 0.5, 105.),
        0,
    );
    k.part(
        c,
        root,
        Vec3::new(65., -0.55, -40.),
        Vec3::new(75., 0.1, 250.),
        8,
    );
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
        k.part(
            c,
            root,
            Vec3::new(x, 0.018, z),
            Vec3::new(0.6 + (i % 5) as f32, 0.008, 0.4 + (i % 3) as f32),
            8,
        );
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
    // Warehouse, structural columns, corrugated facade and exposed roof trusses.
    for z in [-25., -55.] {
        k.part(c, root, Vec3::new(-30., 5., z), Vec3::new(10., 0.3, 20.), 4);
        for dz in [-9., -3., 3., 9.] {
            for x in [-35., -25.] {
                k.part(
                    c,
                    root,
                    Vec3::new(x, 2.5, z + dz),
                    Vec3::new(0.25, 5., 0.25),
                    3,
                );
            }
            k.part(
                c,
                root,
                Vec3::new(-30., 4.7, z + dz),
                Vec3::new(10., 0.18, 0.18),
                0,
            );
        }
        for i in 0..55 {
            k.part(
                c,
                root,
                Vec3::new(-35., 2.5, z - 10. + i as f32 * 0.37),
                Vec3::new(0.09, 5., 0.08),
                4,
            );
        }
    }
    // Quayside gantry cranes, cables and cross braces.
    for z in [-22., -60., -100.] {
        for x in [26., 43.] {
            k.part(c, root, Vec3::new(x, 13., z), Vec3::new(0.9, 26., 0.9), 3);
            for h in [5., 13., 21.] {
                k.part(c, root, Vec3::new(x, h, z), Vec3::new(1.3, 0.3, 5.), 0);
            }
        }
        k.part(c, root, Vec3::new(36., 26., z), Vec3::new(36., 1.2, 1.2), 3);
        k.part(
            c,
            root,
            Vec3::new(40., 24., z),
            Vec3::new(32., 0.35, 0.35),
            0,
        );
        for x in [24., 29., 34., 39., 44., 49.] {
            k.part(c, root, Vec3::new(x, 25., z), Vec3::new(0.18, 2., 0.18), 0);
        }
        k.part(
            c,
            root,
            Vec3::new(47., 17., z),
            Vec3::new(0.035, 16., 0.035),
            0,
        );
    }
    // Triangulated crane bracing and trolley rails.
    for z in [-22., -60., -100.] {
        for h in [4., 10., 16., 22.] {
            let bar = k.part(
                c,
                root,
                Vec3::new(34.5, h, z),
                Vec3::new(18., 0.22, 0.25),
                0,
            );
            c.entity(bar).insert(
                Transform::from_xyz(34.5, h, z)
                    .with_scale(Vec3::new(18., 0.22, 0.25))
                    .with_rotation(Quat::from_rotation_z(if h % 12. < 6. {
                        0.34
                    } else {
                        -0.34
                    })),
            );
        }
        for x in [27., 42.] {
            k.part(c, root, Vec3::new(x, 27.6, z), Vec3::new(0.14, 2., 0.14), 4);
        }
        k.part(
            c,
            root,
            Vec3::new(34., 28.6, z),
            Vec3::new(22., 0.14, 0.14),
            4,
        );
    }
    // Moored freighter and deck cargo.
    k.part(
        c,
        root,
        Vec3::new(53., 2., -48.),
        Vec3::new(14., 5., 65.),
        1,
    );
    k.part(
        c,
        root,
        Vec3::new(53., 4.6, -48.),
        Vec3::new(14.3, 0.35, 65.),
        4,
    );
    for i in 0..5 {
        for x in [49., 53., 57.] {
            k.container(c, Vec3::new(x, 4.8, -28. - i as f32 * 10.), 2);
        }
    }
    k.part(c, root, Vec3::new(53., 9., -76.), Vec3::new(11., 9., 8.), 4);
    for x in [49., 51., 53., 55., 57.] {
        k.part(
            c,
            root,
            Vec3::new(x, 12., -71.9),
            Vec3::new(1.3, 1., 0.05),
            8,
        );
    }
    let boat = k.root(c, EXIT + Vec3::new(4., -0.15, 0.));
    k.part(c, boat, Vec3::ZERO, Vec3::new(4., 0.9, 7.), 0);
    k.part(c, boat, Vec3::new(0., 0.7, 0.), Vec3::new(2.5, 0.5, 4.), 3);
    k.part(c, boat, Vec3::new(0., 1.4, 0.8), Vec3::new(1.8, 1., 1.3), 4);
    port
}
