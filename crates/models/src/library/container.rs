use crate::*;
impl Kit {
    pub fn container(&self, c: &mut Commands, p: Vec3, mat: usize) -> Entity {
        let e = self.root(c, p);
        c.entity(e)
            .insert(crate::geometry::batch::StaticContainer(mat));
        self.part(c, e, Vec3::new(0., 1.4, 0.), Vec3::new(3., 2.8, 9.), mat);
        // Roof corrugations, corner castings and split doors.
        for i in 0..25 {
            self.part(
                c,
                e,
                Vec3::new(0., 2.82, i as f32 * 0.35 - 4.2),
                Vec3::new(2.85, 0.045, 0.065),
                mat,
            );
        }
        for x in [-1.43, 1.43] {
            for y in [0.12, 2.68] {
                for z in [-4.48, 4.48] {
                    self.part(c, e, Vec3::new(x, y, z), Vec3::new(0.22, 0.22, 0.16), 4);
                    self.part(
                        c,
                        e,
                        Vec3::new(x, y, z.signum() * 4.57),
                        Vec3::new(0.08, 0.07, 0.018),
                        5,
                    );
                }
            }
        }
        for x in [-0.74, 0.74] {
            self.part(
                c,
                e,
                Vec3::new(x, 1.4, 4.52),
                Vec3::new(1.43, 2.5, 0.035),
                mat,
            );
            for y in [0.55, 2.15] {
                self.part(c, e, Vec3::new(x, y, 4.59), Vec3::new(0.25, 0.06, 0.06), 4);
            }
        }
        for i in 0..25 {
            for x in [-1.52, 1.52] {
                self.part(
                    c,
                    e,
                    Vec3::new(x, 1.4, i as f32 * 0.35 - 4.2),
                    Vec3::new(0.055, 2.65, 0.065),
                    mat,
                );
            }
        }
        for x in [-1.4, 1.4] {
            for y in [0.1, 2.7] {
                self.part(c, e, Vec3::new(x, y, 0.), Vec3::new(0.14, 0.14, 9.1), 4);
            }
        }
        for x in [-0.65, 0.65] {
            self.part(c, e, Vec3::new(x, 1.4, 4.53), Vec3::new(0.05, 2.5, 0.05), 4);
        }

        for x in [-1.08, -0.4, 0.4, 1.08] {
            self.rod(
                c,
                e,
                Vec3::new(x, 0.23, 4.62),
                Vec3::new(x, 2.57, 4.62),
                0.04,
                4,
            );
            for y in [0.38, 1.25, 2.4] {
                self.shaped(c, e, Vec3::new(x, y, 4.64), Vec3::new(0.13, 0.09, 0.055), 4);
            }
            self.part(
                c,
                e,
                Vec3::new(x + 0.1, 1.1, 4.68),
                Vec3::new(0.23, 0.035, 0.04),
                0,
            );
        }
        for x in [-1.43, 1.43] {
            for y in [0.5, 1.4, 2.3] {
                self.shaped(c, e, Vec3::new(x, y, 4.58), Vec3::new(0.18, 0.13, 0.07), 4);
            }
            // Capacity plates and a sequence of identification stencil bars.
            self.part(
                c,
                e,
                Vec3::new(x * 1.075, 2.1, 2.8),
                Vec3::new(0.018, 0.25, 0.75),
                4,
            );
            for i in 0..7 {
                self.part(
                    c,
                    e,
                    Vec3::new(x * 1.075, 1.72, 2.5 + i as f32 * 0.1),
                    Vec3::new(0.02, 0.15, 0.035),
                    4,
                );
            }
        }
        for sign in [-1., 1.] {
            self.label(
                c,
                e,
                "SALTLINE",
                Transform::from_xyz(sign * 1.565, 1.65, -0.2)
                    .with_rotation(Quat::from_rotation_y(sign * std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::splat(0.8)),
            );
            self.label(
                c,
                e,
                "B4-07",
                Transform::from_xyz(sign * 1.566, 0.7, 2.1)
                    .with_rotation(Quat::from_rotation_y(sign * std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::splat(0.23)),
            );
            // Rust trails and corrugation normals are baked into the shared paint maps.
        }
        e
    }
}
