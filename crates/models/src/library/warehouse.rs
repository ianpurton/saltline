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
                    Vec3::new(-30., 6.35, z + dz),
                    0.13,
                    0,
                );
            }
            k.part(
                c,
                root,
                Vec3::new(-30., 6.32, z + dz),
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
        for dz in [-10., 10.] {
            geometry::assembly::component(
                c,
                root,
                "roof gable",
                k.hard.gable.clone(),
                k.mats[4].clone(),
                Transform::from_xyz(-30., 5., z + dz).with_scale(Vec3::new(10., 1.35, 0.15)),
            );
        }
        // Pitched roof with actual skylight openings between panel segments.
        for sign in [-1., 1.] {
            for i in 0..10 {
                if i == 2 || i == 7 {
                    continue;
                }
                let roof = k.part(
                    c,
                    root,
                    Vec3::new(-30. + sign * 2.55, 5.7, z - 9. + i as f32 * 2.),
                    Vec3::new(5.35, 0.14, 1.96),
                    4,
                );
                c.entity(roof).insert(
                    Transform::from_xyz(-30. + sign * 2.55, 5.7, z - 9. + i as f32 * 2.)
                        .with_rotation(Quat::from_rotation_z(-sign * 0.25))
                        .with_scale(Vec3::new(5.35, 0.14, 1.96)),
                );
            }
        }
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
            Vec3::new(-30., -0.135, z),
            Vec3::new(10., 0.3, 20.),
            9,
        );
        // Corrugated roof underside and loaded racks in the open dock sheds.
        for dz in [-9., -5., -1., 3., 7., 9.] {
            for sign in [-1., 1.] {
                k.rod(
                    c,
                    root,
                    Vec3::new(-30. + sign * 5., 4.95, z + dz),
                    Vec3::new(-30., 6.35, z + dz),
                    0.15,
                    0,
                );
                k.rod(
                    c,
                    root,
                    Vec3::new(-30. + sign * 4.7, 4.95, z + dz),
                    Vec3::new(-30., 4.95, z + dz),
                    0.10,
                    0,
                );
                k.rod(
                    c,
                    root,
                    Vec3::new(-30. + sign * 2.5, 4.95, z + dz),
                    Vec3::new(-30., 6.35, z + dz),
                    0.07,
                    4,
                );
            }
        }
        // Storage mezzanine in the rear half, decorative and outside the gameplay boundary.
        k.part(
            c,
            root,
            Vec3::new(-30.5, 2.6, z - 7.5),
            Vec3::new(8., 0.16, 4.),
            0,
        );
        for i in 0..11 {
            k.part(
                c,
                root,
                Vec3::new(-28.5, i as f32 * 0.23, z - 2.8 - i as f32 * 0.29),
                Vec3::new(1.0, 0.12, 0.32),
                4,
            );
        }
        k.rod(
            c,
            root,
            Vec3::new(-27.95, 1., z - 2.8),
            Vec3::new(-27.95, 3.5, z - 5.7),
            0.045,
            3,
        );
        k.rod(
            c,
            root,
            Vec3::new(-34.5, 3.5, z - 5.5),
            Vec3::new(-31.2, 3.5, z - 5.5),
            0.045,
            3,
        );
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
            for y in [0.9, 2.3, 3.7] {
                k.label(
                    c,
                    root,
                    "CARGO",
                    Transform::from_xyz(-31.62, y, z + dz)
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
                        .with_scale(Vec3::splat(0.24)),
                );
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
                SpotLight {
                    intensity: 45000.,
                    range: 12.,
                    inner_angle: 0.55,
                    outer_angle: 1.05,
                    shadow_maps_enabled: true,
                    color: Color::srgb(1., 0.78, 0.52),
                    ..default()
                },
                Transform::from_xyz(-29., 4.3, z + dz).looking_to(Vec3::NEG_Y, Vec3::Z),
            ));
        }
    }
}
