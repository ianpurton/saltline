use crate::*;
pub fn build(c: &mut Commands, k: &Kit, p: Vec3) -> Entity {
    let e = k.root(c, p);
    k.part(c, e, Vec3::new(0., 0.8, 0.), Vec3::new(0.9, 1.6, 0.6), 4);
    k.part(
        c,
        e,
        Vec3::new(0., 1.25, 0.32),
        Vec3::new(0.65, 0.4, 0.04),
        6,
    );
    k.part(c, e, Vec3::new(0.3, 2., 0.), Vec3::new(0.035, 2., 0.035), 0);

    k.shaped(c, e, Vec3::new(0., 0.08, 0.), Vec3::new(1.2, 0.16, 0.95), 0);
    k.shaped(
        c,
        e,
        Vec3::new(0., 1.65, 0.),
        Vec3::new(1.02, 0.12, 0.72),
        0,
    );
    for x in [-0.4, 0.4] {
        for y in [0.3, 1.4] {
            k.shaped(c, e, Vec3::new(x, y, 0.34), Vec3::new(0.07, 0.15, 0.045), 0);
        }
    }
    for i in 0..8 {
        k.part(
            c,
            e,
            Vec3::new(0., 0.35 + i as f32 * 0.065, 0.31),
            Vec3::new(0.58, 0.024, 0.03),
            5,
        );
    }
    for x in [-0.2, 0., 0.2] {
        k.shaped(
            c,
            e,
            Vec3::new(x, 0.95, 0.34),
            Vec3::new(0.065, 0.065, 0.035),
            7,
        );
    }
    k.rod(
        c,
        e,
        Vec3::new(-0.32, 1.8, 0.),
        Vec3::new(-0.32, 2.75, 0.),
        0.025,
        4,
    );
    for y in [2.2, 2.5, 2.7] {
        k.rod(
            c,
            e,
            Vec3::new(-0.55, y, 0.),
            Vec3::new(-0.1, y, 0.),
            0.02,
            4,
        );
    }
    geometry::assembly::component(
        c,
        e,
        "power cable",
        k.hard.cable.clone(),
        k.mats[5].clone(),
        Transform::from_xyz(0.35, 0.4, 0.2).with_scale(Vec3::splat(2.)),
    );
    let indicator = k.part(c, e, Vec3::new(0., 1.8, 0.), Vec3::new(0.4, 0.1, 0.4), 7);
    indicator
}
