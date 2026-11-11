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
        Vec3::new(2400., 0.1, 2400.),
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
    for z in [-25., -55.] {
        k.part(
            c,
            root,
            Vec3::new(-30., -0.15, z),
            Vec3::new(10., 0.3, 20.),
            9,
        );
        // Corrugated roof underside and loaded racks in the open dock sheds.
        for i in 0..45 {
            k.part(
                c,
                root,
                Vec3::new(-30., 4.8, z - 9.8 + i as f32 * 0.44),
                Vec3::new(9.8, 0.07, 0.07),
                4,
            );
        }
        for dz in [-7., 0., 7.] {
            for y in [0.4, 1.8, 3.2] {
                k.part(
                    c,
                    root,
                    Vec3::new(-33., y, z + dz),
                    Vec3::new(2.7, 0.1, 2.),
                    1,
                );
                for dx in [-0.8, 0., 0.8] {
                    k.part(
                        c,
                        root,
                        Vec3::new(-33. + dx, y + 0.5, z + dz),
                        Vec3::new(0.7, 0.9, 1.3),
                        3,
                    );
                }
            }
            for dx in [-1.4, 1.4] {
                k.part(
                    c,
                    root,
                    Vec3::new(-33. + dx, 1.8, z + dz),
                    Vec3::new(0.1, 3.6, 0.1),
                    1,
                );
            }
            k.part(
                c,
                root,
                Vec3::new(-29., 4.55, z + dz),
                Vec3::new(0.7, 0.08, 0.35),
                7,
            );
            c.spawn((
                PointLight {
                    intensity: 18000.,
                    range: 9.,
                    color: Color::srgb(1., 0.78, 0.52),
                    ..default()
                },
                Transform::from_xyz(-29., 4.3, z + dz),
            ));
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
    // Non-colliding quayside dressing stays outside the playable road.
    for i in 0..18 {
        let z = 5. - i as f32 * 5.2;
        for x in [-20., 20.] {
            k.part(c, root, Vec3::new(x, 0.12, z), Vec3::new(1.4, 0.24, 0.9), 1);
            for dz in [-0.32, 0., 0.32] {
                k.part(
                    c,
                    root,
                    Vec3::new(x, 0.27, z + dz),
                    Vec3::new(1.5, 0.06, 0.15),
                    3,
                );
            }
            if i % 3 == 0 {
                k.part(c, root, Vec3::new(x, 0.7, z), Vec3::new(0.95, 0.8, 0.8), 4);
                for dx in [-0.35, 0.35] {
                    k.part(
                        c,
                        root,
                        Vec3::new(x + dx, 0.7, z + 0.41),
                        Vec3::new(0.05, 0.8, 0.025),
                        5,
                    );
                }
            }
        }
    }
    for z in [-22., -60., -100.] {
        // Suspended trolley and hoist spreader, diagonal boom webbing.
        k.part(c, root, Vec3::new(47., 24.6, z), Vec3::new(2.4, 1.1, 2.), 4);
        k.part(c, root, Vec3::new(47., 9., z), Vec3::new(3.5, 0.35, 2.2), 3);
        for x in [45.6, 48.4] {
            for dz in [-0.8, 0.8] {
                k.part(
                    c,
                    root,
                    Vec3::new(x, 17., z + dz),
                    Vec3::new(0.04, 15., 0.04),
                    5,
                );
            }
        }
        for i in 0..9 {
            let x = 20. + i as f32 * 3.5;
            let beam = k.part(c, root, Vec3::new(x, 25., z), Vec3::new(4., 0.12, 0.15), 4);
            c.entity(beam).insert(
                Transform::from_xyz(x, 25., z)
                    .with_scale(Vec3::new(4., 0.12, 0.15))
                    .with_rotation(Quat::from_rotation_z(if i % 2 == 0 { 0.5 } else { -0.5 })),
            );
        }
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
