use crate::*;
pub fn build(c: &mut Commands, k: &Kit, root: Entity) {
    for z in [-22., -60., -100.] {
        // Four-legged portal, access ladder, machinery cabin and raised boom.
        for x in [26., 43.] {
            for dz in [-2., 2.] {
                k.shaped(
                    c,
                    root,
                    Vec3::new(x, 0.8, z + dz),
                    Vec3::new(2.2, 1.6, 2.),
                    0,
                );
                k.rod(
                    c,
                    root,
                    Vec3::new(x, 1., z + dz),
                    Vec3::new(x, 26., z + dz),
                    0.5,
                    3,
                );
            }
            for i in 0..42 {
                k.part(
                    c,
                    root,
                    Vec3::new(x + 0.65, 1. + i as f32 * 0.6, z),
                    Vec3::new(0.12, 0.045, 0.65),
                    4,
                );
            }
        }
        k.shaped(
            c,
            root,
            Vec3::new(40., 25., z + 2.),
            Vec3::new(4., 2., 3.),
            4,
        );
        k.part(
            c,
            root,
            Vec3::new(40., 25.3, z + 3.52),
            Vec3::new(3.4, 0.8, 0.035),
            8,
        );
        k.rod(
            c,
            root,
            Vec3::new(31., 26., z),
            Vec3::new(31., 39., z),
            0.6,
            3,
        );
        for x in [19., 54.] {
            k.rod(
                c,
                root,
                Vec3::new(31., 39., z),
                Vec3::new(x, 26., z),
                0.08,
                0,
            );
        }
        for dz in [-2., 2.] {
            k.rod(
                c,
                root,
                Vec3::new(18., 27., z + dz),
                Vec3::new(54., 27., z + dz),
                0.22,
                3,
            );
            for i in 0..10 {
                let x = 19. + i as f32 * 3.5;
                k.rod(
                    c,
                    root,
                    Vec3::new(x, 26., z + dz),
                    Vec3::new(x + 2., 28., z + dz),
                    0.12,
                    0,
                );
            }
        }
    }
    // Quayside gantry cranes, cables and cross braces.
    for z in [-22., -60., -100.] {
        for x in [26., 43.] {
            k.part(c, root, Vec3::new(x, 13., z), Vec3::new(1.5, 26., 1.5), 3);
            for h in [5., 13., 21.] {
                k.part(c, root, Vec3::new(x, h, z), Vec3::new(1.3, 0.3, 5.), 0);
            }
        }
        k.part(c, root, Vec3::new(36., 26., z), Vec3::new(36., 2.0, 3.2), 3);
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
}
pub fn details(c: &mut Commands, k: &Kit, root: Entity) {
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
}
