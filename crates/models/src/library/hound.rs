use crate::*;
use geometry::assembly::{component, pivot};
impl Kit {
    pub fn hound(&self, c: &mut Commands, e: Entity) {
        let h = &self.hard;
        let part = |c: &mut Commands,
                    parent,
                    name: &str,
                    mesh: &Handle<Mesh>,
                    p: Vec3,
                    s: Vec3,
                    rot: Quat,
                    mat: usize| {
            component(
                c,
                parent,
                name,
                mesh.clone(),
                self.industrial[mat].clone(),
                Transform::from_translation(p)
                    .with_scale(s)
                    .with_rotation(rot),
            )
        };

        // Recessed identification strokes, inspection plates and deck tie-downs.
        for x in [-0.383, 0.383] {
            for i in 0..5 {
                part(
                    c,
                    e,
                    "unit identification",
                    &h.armor,
                    Vec3::new(x, 0.99, -0.39 + i as f32 * 0.055),
                    Vec3::new(0.006, 0.095, 0.021),
                    Quat::IDENTITY,
                    1,
                );
            }
            part(
                c,
                e,
                "service latch",
                &h.armor,
                Vec3::new(x, 1.02, 0.39),
                Vec3::new(0.018, 0.035, 0.1),
                Quat::IDENTITY,
                2,
            );
        }
        let side = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let axle = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
        part(
            c,
            e,
            "armored keel",
            &h.armor,
            Vec3::new(0., 0.91, 0.),
            Vec3::new(0.64, 0.38, 1.34),
            Quat::IDENTITY,
            1,
        );
        part(
            c,
            e,
            "upper deck",
            &h.armor,
            Vec3::new(0., 1.15, 0.),
            Vec3::new(0.78, 0.09, 1.42),
            Quat::IDENTITY,
            0,
        );
        for x in [-1., 1.] {
            for (z, length) in [(-0.2, 0.66), (0.37, 0.36)] {
                part(
                    c,
                    e,
                    "separated side armor",
                    &h.panel,
                    Vec3::new(x * 0.354, 0.98, z),
                    Vec3::new(length, 0.29, 0.045),
                    side,
                    0,
                );
                for dz in [-length * 0.4, length * 0.4] {
                    for y in [0.87, 1.09] {
                        part(
                            c,
                            e,
                            "captive fastener",
                            &h.hub,
                            Vec3::new(x * 0.383, y, z + dz),
                            Vec3::new(0.025, 0.016, 0.025),
                            axle,
                            1,
                        );
                    }
                }
            }
            part(
                c,
                e,
                "safety sill",
                &h.armor,
                Vec3::new(x * 0.35, 0.77, 0.05),
                Vec3::new(0.045, 0.05, 0.98),
                Quat::IDENTITY,
                4,
            );
            // Tubular rack perimeter and vertical mounts.
            for z in [-0.53, 0.53] {
                self.hound_rod(
                    c,
                    e,
                    "rack upright",
                    Vec3::new(x * 0.32, 1.19, z),
                    Vec3::new(x * 0.32, 1.31, z),
                    0.018,
                    1,
                );
            }
            self.hound_rod(
                c,
                e,
                "rack rail",
                Vec3::new(x * 0.32, 1.31, -0.58),
                Vec3::new(x * 0.32, 1.31, 0.58),
                0.024,
                1,
            );
            for z in [-0.49, 0.49] {
                let phase = if x * z > 0. { 0. } else { std::f32::consts::PI };
                let hip = pivot(c, e, "hip articulation", Vec3::new(x * 0.43, 0.96, z));
                c.entity(hip).insert(Leg { phase });
                part(
                    c,
                    hip,
                    "hip gearbox",
                    &h.hub,
                    Vec3::ZERO,
                    Vec3::new(0.27, 0.19, 0.27),
                    axle,
                    4,
                );
                part(
                    c,
                    hip,
                    "hip cover",
                    &h.hub,
                    Vec3::new(x * 0.11, 0., 0.),
                    Vec3::new(0.245, 0.035, 0.245),
                    axle,
                    0,
                );
                part(
                    c,
                    hip,
                    "hip shaft",
                    &h.hub,
                    Vec3::new(x * 0.135, 0., 0.),
                    Vec3::new(0.125, 0.025, 0.125),
                    axle,
                    1,
                );
                for i in 0..6 {
                    let a = i as f32 * std::f32::consts::TAU / 6.;
                    part(
                        c,
                        hip,
                        "gearbox cover screw",
                        &h.hub,
                        Vec3::new(x * 0.137, a.cos() * 0.096, a.sin() * 0.096),
                        Vec3::new(0.014, 0.012, 0.014),
                        axle,
                        1,
                    );
                }
                // Upper limb leans rearward. Its armor is a six-sided extruded profile.
                let knee_pos = Vec3::new(x * 0.025, -0.44, 0.20);
                self.hound_rod(c, hip, "upper load member", Vec3::ZERO, knee_pos, 0.115, 1);
                part(
                    c,
                    hip,
                    "upper leg armor",
                    &h.limb,
                    Vec3::new(x * 0.055, -0.23, 0.1),
                    Vec3::new(0.22, 0.43, 0.11),
                    side * Quat::from_rotation_z(-0.36),
                    0,
                );
                part(
                    c,
                    hip,
                    "upper safety band",
                    &h.armor,
                    Vec3::new(x * 0.08, -0.20, 0.08),
                    Vec3::new(0.14, 0.045, 0.2),
                    Quat::from_rotation_x(-0.4),
                    4,
                );
                self.hound_rod(
                    c,
                    hip,
                    "hydraulic cylinder",
                    Vec3::new(-x * 0.08, -0.05, -0.03),
                    Vec3::new(-x * 0.08, -0.28, 0.08),
                    0.055,
                    1,
                );
                self.hound_rod(
                    c,
                    hip,
                    "piston shaft",
                    Vec3::new(-x * 0.08, -0.24, 0.06),
                    knee_pos + Vec3::new(-x * 0.08, 0., 0.),
                    0.024,
                    2,
                );
                part(
                    c,
                    hip,
                    "flexible hydraulic hose",
                    &h.cable,
                    Vec3::new(x * 0.12, 0., 0.),
                    Vec3::new(0.6, 0.65, 1.),
                    Quat::IDENTITY,
                    3,
                );
                let knee = pivot(c, hip, "knee articulation", knee_pos);
                c.entity(knee).insert(Knee { phase });
                part(
                    c,
                    knee,
                    "knee housing",
                    &h.hub,
                    Vec3::ZERO,
                    Vec3::new(0.17, 0.16, 0.17),
                    axle,
                    1,
                );
                part(
                    c,
                    knee,
                    "knee cap",
                    &h.hub,
                    Vec3::new(x * 0.09, 0., 0.),
                    Vec3::new(0.13, 0.025, 0.13),
                    axle,
                    0,
                );
                let ankle = Vec3::new(0., -0.40, -0.20);
                self.hound_rod(c, knee, "lower spar", Vec3::ZERO, ankle, 0.075, 1);
                self.hound_rod(
                    c,
                    knee,
                    "lower alloy actuator",
                    Vec3::new(x * 0.04, -0.05, -0.065),
                    ankle + Vec3::new(x * 0.04, 0.06, 0.035),
                    0.042,
                    0,
                );
                self.hound_rod(
                    c,
                    knee,
                    "exposed lower linkage",
                    Vec3::new(-x * 0.075, -0.02, 0.04),
                    ankle + Vec3::new(-x * 0.075, 0.02, 0.03),
                    0.017,
                    2,
                );
                part(
                    c,
                    knee,
                    "chamfered shin armor",
                    &h.limb,
                    Vec3::new(x * 0.038, -0.19, -0.095),
                    Vec3::new(0.095, 0.29, 0.065),
                    side * Quat::from_rotation_z(0.46),
                    0,
                );
                part(
                    c,
                    knee,
                    "shin safety cuff",
                    &h.armor,
                    Vec3::new(x * 0.035, -0.32, -0.16),
                    Vec3::new(0.10, 0.025, 0.09),
                    Quat::from_rotation_x(0.46),
                    4,
                );
                part(
                    c,
                    knee,
                    "ankle boot",
                    &h.taper,
                    ankle + Vec3::new(0., 0.035, 0.),
                    Vec3::new(0.11, 0.14, 0.13),
                    Quat::IDENTITY,
                    3,
                );
                part(
                    c,
                    knee,
                    "ground contact pad",
                    &h.panel,
                    ankle + Vec3::new(0., -0.015, -0.035),
                    Vec3::new(0.18, 0.065, 0.25),
                    Quat::IDENTITY,
                    3,
                );
                for dz in [-0.07, 0.03] {
                    part(
                        c,
                        knee,
                        "foot tread",
                        &h.armor,
                        ankle + Vec3::new(0., -0.04, dz),
                        Vec3::new(0.17, 0.018, 0.025),
                        Quat::IDENTITY,
                        1,
                    );
                }
            }
        }
        part(
            c,
            e,
            "sensor neck",
            &h.taper,
            Vec3::new(0., 1.14, -0.78),
            Vec3::new(0.22, 0.23, 0.22),
            Quat::from_rotation_x(-0.5),
            1,
        );
        part(
            c,
            e,
            "forward optical housing",
            &h.armor,
            Vec3::new(0., 1.28, -0.92),
            Vec3::new(0.43, 0.24, 0.35),
            Quat::IDENTITY,
            0,
        );
        for x in [-0.115, 0.115] {
            part(
                c,
                e,
                "optical bezel",
                &h.hub,
                Vec3::new(x, 1.28, -1.10),
                Vec3::new(0.12, 0.035, 0.12),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                1,
            );
            part(
                c,
                e,
                "optical lens",
                &h.hub,
                Vec3::new(x, 1.28, -1.122),
                Vec3::new(0.082, 0.012, 0.082),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                5,
            );
        }
        for x in [-1., 1.] {
            part(
                c,
                e,
                "lateral optical bezel",
                &h.hub,
                Vec3::new(x * 0.219, 1.28, -0.93),
                Vec3::new(0.11, 0.018, 0.11),
                axle,
                1,
            );
            part(
                c,
                e,
                "lateral optical lens",
                &h.hub,
                Vec3::new(x * 0.231, 1.28, -0.93),
                Vec3::new(0.073, 0.012, 0.073),
                axle,
                5,
            );
        }
        part(
            c,
            e,
            "top communications module",
            &h.armor,
            Vec3::new(0., 1.37, 0.18),
            Vec3::new(0.32, 0.18, 0.3),
            Quat::IDENTITY,
            0,
        );
        part(
            c,
            e,
            "communications cap",
            &h.armor,
            Vec3::new(0., 1.48, 0.18),
            Vec3::new(0.16, 0.055, 0.15),
            Quat::IDENTITY,
            1,
        );
        for i in 0..7 {
            part(
                c,
                e,
                "deck cooling fins",
                &h.armor,
                Vec3::new(0., 1.22, -0.4 + i as f32 * 0.07),
                Vec3::new(0.4, 0.035, 0.025),
                Quat::IDENTITY,
                1,
            );
        }
        part(
            c,
            e,
            "rear equipment tray",
            &h.armor,
            Vec3::new(0., 1.24, 0.68),
            Vec3::new(0.56, 0.13, 0.25),
            Quat::IDENTITY,
            1,
        );
        self.hound_rod(
            c,
            e,
            "radio antenna",
            Vec3::new(0.23, 1.27, 0.72),
            Vec3::new(0.26, 1.98, 0.82),
            0.009,
            1,
        );
    }
    fn hound_rod(
        &self,
        c: &mut Commands,
        parent: Entity,
        name: &str,
        a: Vec3,
        b: Vec3,
        r: f32,
        mat: usize,
    ) {
        let d = b - a;
        component(
            c,
            parent,
            name,
            self.hard.rod.clone(),
            self.industrial[mat].clone(),
            Transform::from_translation(a)
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, d.normalize()))
                .with_scale(Vec3::new(r, d.length(), r)),
        );
    }
}
