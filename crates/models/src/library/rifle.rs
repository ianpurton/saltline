use crate::*;
impl Kit {
    fn rifle_part(&self, c: &mut Commands, e: Entity, p: Vec3, s: Vec3, mat: usize) -> Entity {
        let material = match mat {
            3 => 0,
            4 => 1,
            5 => 3,
            _ => 1,
        };
        geometry::assembly::component(
            c,
            e,
            "rifle mechanism",
            self.hard.armor.clone(),
            if mat == 3 {
                self.mats[3].clone()
            } else {
                self.industrial[material].clone()
            },
            Transform::from_translation(p).with_scale(s),
        )
    }

    pub fn rifle(&self, c: &mut Commands, parent: Entity) -> Entity {
        let e = self.root(c, Vec3::ZERO);
        c.entity(parent).add_child(e);
        for (p, s, m) in [
            ((0., 0., 0.), (0.13, 0.16, 0.38), 0),
            ((0., 0., -0.38), (0.11, 0.12, 0.42), 0),
            ((0., 0., -0.7), (0.045, 0.045, 0.28), 5),
            ((0., -0.19, 0.02), (0.09, 0.28, 0.14), 3),
            ((0., -0.16, 0.21), (0.07, 0.23, 0.09), 5),
            ((0., 0., 0.34), (0.1, 0.14, 0.25), 5),
            ((0., 0.13, -0.06), (0.06, 0.035, 0.045), 4),
            ((0., 0.13, -0.53), (0.035, 0.1, 0.035), 4),
        ] {
            self.rifle_part(c, e, Vec3::from(p), Vec3::from(s), m);
        }
        for i in 0..19 {
            self.rifle_part(
                c,
                e,
                Vec3::new(0., 0.085, 0.13 - i as f32 * 0.036),
                Vec3::new(0.13, 0.018, 0.016),
                4,
            );
        }
        for i in 0..6 {
            for x in [-0.058, 0.058] {
                self.rifle_part(
                    c,
                    e,
                    Vec3::new(x, 0., -0.25 - i as f32 * 0.055),
                    Vec3::new(0.008, 0.045, 0.032),
                    5,
                );
            }
        }

        self.rifle_part(
            c,
            e,
            Vec3::new(0., 0.105, -0.06),
            Vec3::new(0.065, 0.05, 0.07),
            0,
        );
        // Open stock frame, separate butt pad and cheek rest.
        self.rifle_part(
            c,
            e,
            Vec3::new(0., 0.015, 0.53),
            Vec3::new(0.115, 0.11, 0.3),
            0,
        );
        self.rifle_part(
            c,
            e,
            Vec3::new(0., -0.08, 0.67),
            Vec3::new(0.12, 0.28, 0.045),
            5,
        );
        self.rod(
            c,
            e,
            Vec3::new(0., -0.04, 0.35),
            Vec3::new(0., -0.19, 0.65),
            0.045,
            0,
        );
        // Turned barrel, gas tube and slotted muzzle brake.
        self.rod(
            c,
            e,
            Vec3::new(0., 0., -0.57),
            Vec3::new(0., 0., -0.91),
            0.045,
            4,
        );
        self.rod(
            c,
            e,
            Vec3::new(0., 0., -0.85),
            Vec3::new(0., 0., -1.01),
            0.067,
            0,
        );
        self.rod(
            c,
            e,
            Vec3::new(0., 0.045, -0.37),
            Vec3::new(0., 0.045, -0.76),
            0.018,
            4,
        );
        for i in 0..4 {
            self.rifle_part(
                c,
                e,
                Vec3::new(0., 0., -0.88 - i as f32 * 0.03),
                Vec3::new(0.072, 0.008, 0.013),
                5,
            );
        }
        // Curved magazine sections and pressed ribs.
        for i in 0..6 {
            let y = -0.12 - i as f32 * 0.045;
            let z = 0.01 - i as f32 * i as f32 * 0.0015;
            self.rifle_part(c, e, Vec3::new(0., y, z), Vec3::new(0.092, 0.05, 0.15), 3);
            for x in [-0.048, 0.048] {
                self.rifle_part(c, e, Vec3::new(x, y, z), Vec3::new(0.006, 0.012, 0.12), 4);
            }
        }
        for x in [-0.069, 0.069] {
            self.rifle_part(
                c,
                e,
                Vec3::new(x, 0.015, 0.02),
                Vec3::new(0.008, 0.048, 0.23),
                5,
            );
            for z in [-0.08, 0.12] {
                self.rifle_part(c, e, Vec3::new(x, 0.045, z), Vec3::splat(0.018), 4);
            }
        }
        self.rifle_part(
            c,
            e,
            Vec3::new(0.085, -0.035, 0.14),
            Vec3::new(0.045, 0.025, 0.06),
            4,
        );
        // Trigger guard is hollow, with a separate trigger.
        self.rod(
            c,
            e,
            Vec3::new(0., -0.1, 0.08),
            Vec3::new(0., -0.25, 0.08),
            0.018,
            0,
        );
        self.rod(
            c,
            e,
            Vec3::new(0., -0.25, 0.08),
            Vec3::new(0., -0.25, 0.21),
            0.018,
            0,
        );
        self.rod(
            c,
            e,
            Vec3::new(0., -0.12, 0.14),
            Vec3::new(0., -0.21, 0.12),
            0.014,
            4,
        );
        self.rifle_part(
            c,
            e,
            Vec3::new(0., -0.17, -0.4),
            Vec3::new(0.065, 0.19, 0.08),
            0,
        );
        e
    }
}
