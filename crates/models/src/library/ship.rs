use crate::*;
pub fn build(c: &mut Commands, k: &Kit, root: Entity) {
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
}
