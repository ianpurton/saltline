use crate::*;
impl Kit {
    pub fn drone(&self, c: &mut Commands, e: Entity) {
        for x in [-1., 1.] {
            for z in [-0.6, 0.6] {
                self.rod(
                    c,
                    e,
                    Vec3::new(x * 0.22, 0., z * 0.4),
                    Vec3::new(x * 0.65, 0., z),
                    0.09,
                    4,
                );
                self.rod(
                    c,
                    e,
                    Vec3::new(x * 0.65, -0.1, z),
                    Vec3::new(x * 0.65, 0.16, z),
                    0.16,
                    0,
                );
                for i in 0..6 {
                    self.shaped(
                        c,
                        e,
                        Vec3::new(x * 0.65, -0.06 + i as f32 * 0.025, z),
                        Vec3::new(0.22, 0.012, 0.22),
                        4,
                    );
                }
            }
            for i in 0..7 {
                self.shaped(
                    c,
                    e,
                    Vec3::new(x * 0.355, 0.03, -0.26 + i as f32 * 0.08),
                    Vec3::new(0.018, 0.1, 0.035),
                    5,
                );
            }
        }
        self.shaped(
            c,
            e,
            Vec3::new(0., 0.16, 0.1),
            Vec3::new(0.45, 0.12, 0.45),
            0,
        );
        self.rod(
            c,
            e,
            Vec3::new(0.2, 0.2, 0.2),
            Vec3::new(0.2, 0.65, 0.24),
            0.012,
            4,
        );
        self.rod(
            c,
            e,
            Vec3::new(0., -0.22, -0.4),
            Vec3::new(0., -0.22, -0.53),
            0.14,
            4,
        );
        for x in [-0.28, 0.28] {
            self.shaped(c, e, Vec3::new(x, 0.18, 0.), Vec3::new(0.08, 0.06, 0.65), 0);
            for z in [-0.28, 0.28] {
                self.joint(c, e, Vec3::new(x, 0.22, z), 0.04);
            }
            self.shaped(
                c,
                e,
                Vec3::new(x, -0.33, 0.22),
                Vec3::new(0.035, 0.55, 0.035),
                5,
            );
            self.shaped(
                c,
                e,
                Vec3::new(x, -0.59, 0.13),
                Vec3::new(0.045, 0.045, 0.5),
                0,
            );
        }

        self.shaped(c, e, Vec3::ZERO, Vec3::new(0.7, 0.25, 0.8), 4);
        self.joint(c, e, Vec3::new(0., -0.21, -0.3), 0.3);
        self.shaped(
            c,
            e,
            Vec3::new(0., -0.2, -0.46),
            Vec3::new(0.2, 0.1, 0.04),
            7,
        );
        for x in [-0.65, 0.65] {
            for z in [-0.6, 0.6] {
                self.shaped(
                    c,
                    e,
                    Vec3::new(x * 0.5, 0., z * 0.5),
                    Vec3::new(0.7, 0.06, 0.08),
                    0,
                );
                self.shaped(c, e, Vec3::new(x, 0.12, z), Vec3::new(0.8, 0.025, 0.13), 5);
                self.joint(c, e, Vec3::new(x, 0., z), 0.22);
                let ring = c
                    .spawn((
                        Mesh3d(self.ring.clone()),
                        MeshMaterial3d(self.mats[0].clone()),
                        Transform::from_xyz(x, 0.1, z),
                    ))
                    .id();
                c.entity(e).add_child(ring);
            }
        }
    }
}
