use crate::*;
pub fn build(c: &mut Commands, k: &Kit, root: Entity) {
    super::fence::build(c, k, root);
    for z in [-18., -48., -70.] {
        for x in [-21., 21.] {
            super::barrel::build(c, k, root, Vec3::new(x, 0., z));
        }
    }
    // Parked forklift stays outside the central mission corridor.
    let p = Vec3::new(20., 0., -32.);
    k.shaped(
        c,
        root,
        p + Vec3::new(0., 0.65, 0.),
        Vec3::new(1.4, 1., 2.1),
        3,
    );
    k.shaped(
        c,
        root,
        p + Vec3::new(0., 1.3, 0.3),
        Vec3::new(0.6, 0.65, 0.6),
        5,
    );
    for x in [-0.75, 0.75] {
        for z in [-0.65, 0.65] {
            geometry::assembly::component(
                c,
                root,
                "forklift tire",
                k.hard.hub.clone(),
                k.industrial[3].clone(),
                Transform::from_translation(p + Vec3::new(x, 0.4, z))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::new(0.7, 0.25, 0.7)),
            );
        }
        k.rod(
            c,
            root,
            p + Vec3::new(x, 0.8, 0.6),
            p + Vec3::new(x, 2.6, 0.6),
            0.07,
            0,
        );
        k.part(
            c,
            root,
            p + Vec3::new(x * 0.6, 1.7, -1.1),
            Vec3::new(0.1, 3.2, 0.15),
            0,
        );
        k.part(
            c,
            root,
            p + Vec3::new(x * 0.6, 0.2, -1.9),
            Vec3::new(0.13, 0.1, 1.7),
            4,
        );
    }
    k.part(
        c,
        root,
        p + Vec3::new(0., 2.6, 0.25),
        Vec3::new(1.65, 0.1, 1.3),
        3,
    );
    for i in 0..12 {
        let z = 4. - i as f32 * 7.;
        // Quay drains, mooring bollards, guard fence and pipe stacks.
        for x in [-22., 22.] {
            k.part(
                c,
                root,
                Vec3::new(x, 0.025, z),
                Vec3::new(0.9, 0.03, 1.3),
                5,
            );
            for j in 0..9 {
                k.part(
                    c,
                    root,
                    Vec3::new(x, 0.045, z - 0.55 + j as f32 * 0.13),
                    Vec3::new(0.85, 0.025, 0.035),
                    4,
                );
            }
        }
        k.rod(
            c,
            root,
            Vec3::new(25., 0., z),
            Vec3::new(25., 0.7, z),
            0.35,
            0,
        );
        k.rod(
            c,
            root,
            Vec3::new(24.6, 0.65, z),
            Vec3::new(25.4, 0.65, z),
            0.23,
            0,
        );
        for j in 0..3 {
            k.rod(
                c,
                root,
                Vec3::new(-22. + j as f32 * 0.38, 0.35, z),
                Vec3::new(-22. + j as f32 * 0.38, 0.35, z - 3.),
                0.3,
                4,
            );
        }
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
}
