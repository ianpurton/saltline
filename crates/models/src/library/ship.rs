use crate::*;
pub fn build(c: &mut Commands, k: &Kit, root: Entity) {
    geometry::assembly::component(
        c,
        root,
        "formed freighter hull",
        k.hard.hull.clone(),
        k.mats[1].clone(),
        Transform::from_xyz(53., 2., -48.)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(14., 65., 5.)),
    );
    geometry::assembly::component(
        c,
        root,
        "continuous forecastle deck",
        k.hard.hull.clone(),
        k.mats[4].clone(),
        Transform::from_xyz(53., 4.6, -48.)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(14.2, 65., 0.2)),
    );
    for x in [46., 60.] {
        k.rod(
            c,
            root,
            Vec3::new(x, 5.5, -77.),
            Vec3::new(x, 5.5, -17.),
            0.065,
            4,
        );
        for i in 0..22 {
            k.rod(
                c,
                root,
                Vec3::new(x, 4.6, -77. + i as f32 * 2.8),
                Vec3::new(x, 5.5, -77. + i as f32 * 2.8),
                0.06,
                4,
            );
        }
        for z in [-22., -68.] {
            k.shaped(c, root, Vec3::new(x, 5., z), Vec3::new(0.9, 0.6, 1.3), 0);
        }
    }
    for y in [7., 10., 13.] {
        k.part(
            c,
            root,
            Vec3::new(53., y, -76.),
            Vec3::new(11.4, 0.18, 8.4),
            4,
        );
    }
    k.shaped(c, root, Vec3::new(56., 15., -77.), Vec3::new(2., 4., 2.), 1);
    k.rod(
        c,
        root,
        Vec3::new(51., 13.5, -76.),
        Vec3::new(51., 19., -76.),
        0.14,
        4,
    );
    k.rod(
        c,
        root,
        Vec3::new(48., 17., -76.),
        Vec3::new(54., 17., -76.),
        0.1,
        4,
    );
    for i in 0..5 {
        for x in [49., 53., 57.] {
            for layer in 0..(2 + i % 2) {
                k.container(
                    c,
                    Vec3::new(x, 4.8 + layer as f32 * 2.85, -28. - i as f32 * 10.),
                    1 + (i + layer) % 3,
                );
            }
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
}
