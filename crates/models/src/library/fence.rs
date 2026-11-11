use crate::*;
pub fn build(c: &mut Commands, k: &Kit, root: Entity) {
    for i in 0..30 {
        let z = 10. - i as f32 * 3.;
        k.rod(
            c,
            root,
            Vec3::new(-24., 0., z),
            Vec3::new(-24., 2.3, z),
            0.065,
            4,
        );
        if i < 29 {
            for y in [0.3, 2.1] {
                k.rod(
                    c,
                    root,
                    Vec3::new(-24., y, z),
                    Vec3::new(-24., y, z - 3.),
                    0.04,
                    4,
                );
            }
            for j in 0..7 {
                let dz = j as f32 * 0.43;
                k.rod(
                    c,
                    root,
                    Vec3::new(-24., 0.3, z - dz),
                    Vec3::new(-24., 2.1, z - dz - 0.7),
                    0.012,
                    4,
                );
                k.rod(
                    c,
                    root,
                    Vec3::new(-24., 2.1, z - dz),
                    Vec3::new(-24., 0.3, z - dz - 0.7),
                    0.012,
                    4,
                );
            }
        }
    }
}
