use crate::*;
pub fn build(c: &mut Commands, k: &Kit, root: Entity, p: Vec3) {
    geometry::assembly::component(
        c,
        root,
        "steel drum",
        k.hard.hub.clone(),
        k.mats[2].clone(),
        Transform::from_translation(p + Vec3::Y * 0.45).with_scale(Vec3::new(0.6, 0.9, 0.6)),
    );
    for y in [0.08, 0.25, 0.65, 0.87] {
        geometry::assembly::component(
            c,
            root,
            "rolled drum bead",
            k.hard.hub.clone(),
            k.mats[4].clone(),
            Transform::from_translation(p + Vec3::Y * y).with_scale(Vec3::new(0.62, 0.025, 0.62)),
        );
    }
    k.shaped(
        c,
        root,
        p + Vec3::new(0.16, 0.91, 0.),
        Vec3::new(0.07, 0.02, 0.07),
        0,
    );
}
