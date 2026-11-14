use super::map_props::paint_index;
use crate::*;
/// Authored dimensions: length along X, base at Y=0, standard width 2.438 m.
#[derive(Clone, Copy, Debug)]
pub struct ContainerSpec {
    pub length: f32,
    pub height: f32,
    pub paint: &'static str,
    pub reefer: bool,
    pub doors: f32,
}
impl Kit {
    /// Compatibility builder for model inspection and existing examples.
    pub fn container(&self, c: &mut Commands, p: Vec3, mat: usize) -> Entity {
        let colors = ["red", "red", "teal", "yellow", "grey"];
        self.container_spec(
            c,
            p,
            ContainerSpec {
                length: 12.19,
                height: 2.59,
                paint: colors[mat.min(4)],
                reefer: false,
                doors: 0.,
            },
        )
    }
    pub fn container_spec(&self, c: &mut Commands, p: Vec3, s: ContainerSpec) -> Entity {
        let e = self.root(c, p);
        let paint = paint_index(s.paint);
        let l = s.length;
        let h = s.height;
        let w = 2.438;
        // Cache separates size, paint and all geometry options; transforms remain per instance.
        let variant = if l < 7. {
            0
        } else if h > 2.7 {
            2
        } else {
            1
        };
        let key = 10000
            + paint
            + variant * 64
            + (s.reefer as usize) * 256
            + ((s.doors.clamp(0., 1.) * 1000.).round() as usize) * 1024;
        c.entity(e).insert((
            Name::new(format!("container {l} / {}", s.paint)),
            crate::geometry::batch::StaticContainer(key),
        ));
        self.part(c, e, Vec3::new(0., h * 0.5, 0.), Vec3::new(l, h, w), paint);
        // Corrugations stay inside the catalogue envelope.
        let ribs = (l / 0.24).floor() as usize;
        for i in 0..ribs {
            let x = -l * 0.5 + 0.18 + i as f32 * (l - 0.36) / (ribs - 1) as f32;
            for z in [-1., 1.] {
                self.part(
                    c,
                    e,
                    Vec3::new(x, h * 0.5, z * (w * 0.5 - 0.018)),
                    Vec3::new(0.045, h - 0.16, 0.036),
                    paint,
                );
            }
            self.part(
                c,
                e,
                Vec3::new(x, h - 0.018, 0.),
                Vec3::new(0.045, 0.036, w - 0.14),
                paint,
            );
        }
        for z in [-1., 1.] {
            for y in [0.065, h - 0.065] {
                self.part(
                    c,
                    e,
                    Vec3::new(0., y, z * (w * 0.5 - 0.045)),
                    Vec3::new(l, 0.13, 0.09),
                    4,
                );
            }
        }
        for x in [-1., 1.] {
            for z in [-1., 1.] {
                self.part(
                    c,
                    e,
                    Vec3::new(x * (l * 0.5 - 0.055), h * 0.5, z * (w * 0.5 - 0.055)),
                    Vec3::new(0.11, h, 0.11),
                    4,
                );
                for y in [0.09, h - 0.09] {
                    self.part(
                        c,
                        e,
                        Vec3::new(x * (l * 0.5 - 0.1), y, z * (w * 0.5 - 0.09)),
                        Vec3::new(0.2, 0.18, 0.18),
                        4,
                    );
                }
            }
        }
        for side in [-1., 1.] {
            let hinge = Vec3::new(l * 0.5 - 0.035, h * 0.5, side * (w * 0.5 - 0.07));
            let rotation = Quat::from_rotation_y(-side * s.doors * 2.1);
            let center = hinge + rotation * Vec3::new(0., 0., -side * w * 0.24);
            let door = self.part(c, e, center, Vec3::new(0.045, h - 0.18, w * 0.48), paint);
            c.entity(door).insert(
                Transform::from_translation(center)
                    .with_rotation(rotation)
                    .with_scale(Vec3::new(0.045, h - 0.18, w * 0.48)),
            );
            for dz in [0.24, 0.72] {
                let center = hinge + rotation * Vec3::new(0.04, 0., -side * dz);
                let rod = self.part(c, e, center, Vec3::new(0.035, h - 0.4, 0.035), 4);
                c.entity(rod).insert(
                    Transform::from_translation(center)
                        .with_rotation(rotation)
                        .with_scale(Vec3::new(0.035, h - 0.4, 0.035)),
                );
            }
        }
        if s.reefer {
            self.part(
                c,
                e,
                Vec3::new(-l * 0.5 + 0.15, h * 0.5, 0.),
                Vec3::new(0.3, h - 0.2, w - 0.15),
                11,
            );
            for i in 0..12 {
                self.part(
                    c,
                    e,
                    Vec3::new(-l * 0.5 - 0.005, 0.2 + i as f32 * (h - 0.4) / 12., 0.),
                    Vec3::new(0.012, 0.045, w - 0.3),
                    5,
                );
            }
        }
        for side in [-1., 1.] {
            self.label(
                c,
                e,
                "SALTLINE",
                Transform::from_xyz(-l * 0.15, h * 0.62, side * (w * 0.5 + 0.002))
                    .with_rotation(Quat::from_rotation_y(if side < 0. {
                        std::f32::consts::PI
                    } else {
                        0.
                    }))
                    .with_scale(Vec3::splat(0.6)),
            );
            self.label(
                c,
                e,
                "B4-07",
                Transform::from_xyz(l * 0.25, h * 0.28, side * (w * 0.5 + 0.003))
                    .with_rotation(Quat::from_rotation_y(if side < 0. {
                        std::f32::consts::PI
                    } else {
                        0.
                    }))
                    .with_scale(Vec3::splat(0.2)),
            );
        }
        e
    }
}
