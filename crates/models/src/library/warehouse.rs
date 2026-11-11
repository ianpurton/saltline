use crate::*;
pub fn build(c: &mut Commands, k: &Kit, root: Entity) {
    for z in [-25., -55.] {
        // Solid back and end walls; the east loading bay remains accessible.
        k.part(
            c,
            root,
            Vec3::new(-35., 2.5, z),
            Vec3::new(0.16, 5., 20.),
            4,
        );
        for dz in [-10., 10.] {
            k.part(
                c,
                root,
                Vec3::new(-30., 2.5, z + dz),
                Vec3::new(10., 5., 0.16),
                4,
            );
            for i in 0..28 {
                k.part(
                    c,
                    root,
                    Vec3::new(-34.8 + i as f32 * 0.35, 2.5, z + dz),
                    Vec3::new(0.06, 4.9, 0.07),
                    0,
                );
            }
        }
        for dz in [-7.5, 7.5] {
            k.part(
                c,
                root,
                Vec3::new(-25., 2.5, z + dz),
                Vec3::new(0.16, 5., 5.),
                4,
            );
            for i in 0..14 {
                k.part(
                    c,
                    root,
                    Vec3::new(-24.9, 2.5, z + dz - 2.4 + i as f32 * 0.35),
                    Vec3::new(0.08, 4.9, 0.055),
                    0,
                );
            }
        }
        k.part(
            c,
            root,
            Vec3::new(-25., 4.5, z),
            Vec3::new(0.22, 1., 10.),
            4,
        );
        for dz in [-5., 5.] {
            k.shaped(
                c,
                root,
                Vec3::new(-24.8, 0.65, z + dz),
                Vec3::new(0.4, 1.3, 0.4),
                3,
            );
        }
        for dz in [-8., -4., 0., 4., 8.] {
            for x in [-34., -26.] {
                k.rod(
                    c,
                    root,
                    Vec3::new(x, 4.6, z + dz),
                    Vec3::new(-30., 5.5, z + dz),
                    0.13,
                    0,
                );
            }
            k.part(
                c,
                root,
                Vec3::new(-30., 5.2, z + dz),
                Vec3::new(1.5, 0.1, 1.),
                8,
            );
        }
        // Wall conduit, ventilation and maintenance walkway.
        k.rod(
            c,
            root,
            Vec3::new(-34.7, 3.9, z - 9.),
            Vec3::new(-34.7, 3.9, z + 9.),
            0.09,
            0,
        );
        for dz in [-6., 6.] {
            k.shaped(
                c,
                root,
                Vec3::new(-34.6, 3.4, z + dz),
                Vec3::new(0.25, 0.8, 1.4),
                0,
            );
            for i in 0..6 {
                k.part(
                    c,
                    root,
                    Vec3::new(-34.4, 3.1 + i as f32 * 0.1, z + dz),
                    Vec3::new(0.02, 0.03, 1.2),
                    4,
                );
            }
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
                        10,
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
}
