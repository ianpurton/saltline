use crate::*;
pub fn build(c: &mut Commands, k: &Kit, root: Entity) {
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
