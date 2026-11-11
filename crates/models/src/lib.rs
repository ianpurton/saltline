use bevy::prelude::*;
/// Shared primitive cache: every procedural object uses these meshes and materials.
pub struct Kit {
    pub cube: Handle<Mesh>,
    pub ball: Handle<Mesh>,
    pub ring: Handle<Mesh>,
    pub mats: Vec<Handle<StandardMaterial>>,
}
impl Kit {
    pub fn new(
        m: &mut Assets<Mesh>,
        a: &mut Assets<StandardMaterial>,
        images: &mut Assets<Image>,
    ) -> Self {
        use bevy::{
            asset::RenderAssetUsages,
            render::render_resource::{Extent3d, TextureDimension, TextureFormat},
        };
        let mut bytes = vec![0u8; 256 * 256 * 4];
        let mut seed = 93u32;
        for pixel in bytes.chunks_exact_mut(4) {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let value = 155 + ((seed >> 24) % 100) as u8;
            pixel.copy_from_slice(&[value, value, value, 255]);
        }
        let texture = images.add(Image::new_fill(
            Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &bytes,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        ));
        let colors = [
            (0.055, 0.075, 0.085),
            (0.35, 0.15, 0.09),
            (0.06, 0.22, 0.24),
            (0.64, 0.48, 0.17),
            (0.52, 0.57, 0.55),
            (0.015, 0.025, 0.03),
            (0.1, 0.9, 0.75),
            (1., 0.27, 0.045),
            (0.08, 0.12, 0.15),
        ];
        Self {
            cube: m.add(Cuboid::default()),
            ring: m.add(Torus::new(0.28, 0.36)),
            ball: m.add(Sphere::new(0.5).mesh().uv(12, 8)),
            mats: colors
                .iter()
                .enumerate()
                .map(|(i, &(r, g, b))| {
                    a.add(StandardMaterial {
                        base_color: Color::srgb(r, g, b),
                        base_color_texture: if i == 6 || i == 7 {
                            None
                        } else {
                            Some(texture.clone())
                        },
                        metallic: if i == 6 || i == 7 { 0. } else { 0.65 },
                        perceptual_roughness: if i == 8 { 0.12 } else { 0.36 },
                        emissive: if i == 6 || i == 7 {
                            LinearRgba::rgb(r * 5., g * 5., b * 5.)
                        } else {
                            LinearRgba::BLACK
                        },
                        ..default()
                    })
                })
                .collect(),
        }
    }
    pub fn part(&self, c: &mut Commands, parent: Entity, p: Vec3, s: Vec3, mat: usize) -> Entity {
        let e = c
            .spawn((
                Mesh3d(self.cube.clone()),
                MeshMaterial3d(self.mats[mat].clone()),
                Transform::from_translation(p).with_scale(s),
            ))
            .id();
        c.entity(parent).add_child(e);
        e
    }
    pub fn joint(&self, c: &mut Commands, parent: Entity, p: Vec3, r: f32) -> Entity {
        let e = c
            .spawn((
                Mesh3d(self.ball.clone()),
                MeshMaterial3d(self.mats[4].clone()),
                Transform::from_translation(p).with_scale(Vec3::splat(r)),
            ))
            .id();
        c.entity(parent).add_child(e);
        e
    }
    pub fn root(&self, c: &mut Commands, p: Vec3) -> Entity {
        c.spawn((Transform::from_translation(p), Visibility::default()))
            .id()
    }
    pub fn container(&self, c: &mut Commands, p: Vec3, mat: usize) -> Entity {
        let e = self.root(c, p);
        self.part(c, e, Vec3::new(0., 1.4, 0.), Vec3::new(3., 2.8, 9.), mat);
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
        e
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
            self.part(c, e, Vec3::from(p), Vec3::from(s), m);
        }
        for i in 0..19 {
            self.part(
                c,
                e,
                Vec3::new(0., 0.085, 0.13 - i as f32 * 0.036),
                Vec3::new(0.13, 0.018, 0.016),
                4,
            );
        }
        for i in 0..6 {
            for x in [-0.058, 0.058] {
                self.part(
                    c,
                    e,
                    Vec3::new(x, 0., -0.25 - i as f32 * 0.055),
                    Vec3::new(0.008, 0.045, 0.032),
                    5,
                );
            }
        }
        e
    }
    pub fn hound(&self, c: &mut Commands, e: Entity) {
        for x in [-0.34, 0.34] {
            for z in [-0.4, 0., 0.4] {
                self.part(c, e, Vec3::new(x, 0.95, z), Vec3::new(0.035, 0.32, 0.32), 4);
                for y in [0.83, 1.07] {
                    self.joint(c, e, Vec3::new(x * 1.08, y, z - 0.11), 0.035);
                }
            }
            self.part(c, e, Vec3::new(x, 0.7, 0.), Vec3::new(0.04, 0.07, 1.1), 3);
        }
        for z in [-0.4, -0.28, -0.16, -0.04, 0.08, 0.2, 0.32] {
            self.part(
                c,
                e,
                Vec3::new(0., 1.24, z),
                Vec3::new(0.45, 0.04, 0.035),
                5,
            );
        }

        self.part(c, e, Vec3::new(0., 0.9, 0.), Vec3::new(0.65, 0.48, 1.25), 0);
        self.part(c, e, Vec3::new(0., 1.16, 0.), Vec3::new(0.72, 0.12, 1.1), 4);
        self.part(
            c,
            e,
            Vec3::new(0., 1.05, -0.83),
            Vec3::new(0.48, 0.3, 0.4),
            4,
        );
        self.part(
            c,
            e,
            Vec3::new(0., 1.07, -1.04),
            Vec3::new(0.31, 0.07, 0.025),
            7,
        );
        for x in [-0.43, 0.43] {
            for z in [-0.43, 0.43] {
                let leg = self.root(c, Vec3::new(x, 0.9, z));
                c.entity(e).add_child(leg);
                c.entity(leg).insert(Leg {
                    phase: if x * z > 0. { 0. } else { 3.14 },
                });
                self.joint(c, leg, Vec3::ZERO, 0.22);
                self.part(
                    c,
                    leg,
                    Vec3::new(0., -0.3, 0.14),
                    Vec3::new(0.18, 0.08, 0.22),
                    3,
                );
                self.part(
                    c,
                    leg,
                    Vec3::new(0., -0.23, 0.09),
                    Vec3::new(0.16, 0.44, 0.2),
                    4,
                );
                self.joint(c, leg, Vec3::new(0., -0.45, 0.16), 0.18);
                self.part(
                    c,
                    leg,
                    Vec3::new(0., -0.64, 0.07),
                    Vec3::new(0.09, 0.4, 0.1),
                    0,
                );
                self.part(
                    c,
                    leg,
                    Vec3::new(0., -0.83, 0.),
                    Vec3::new(0.2, 0.08, 0.3),
                    5,
                );
            }
        }
        self.part(
            c,
            e,
            Vec3::new(0.2, 1.5, 0.4),
            Vec3::new(0.025, 0.8, 0.025),
            0,
        );
    }
    pub fn drone(&self, c: &mut Commands, e: Entity) {
        self.part(c, e, Vec3::ZERO, Vec3::new(0.7, 0.25, 0.8), 4);
        self.joint(c, e, Vec3::new(0., -0.21, -0.3), 0.3);
        self.part(
            c,
            e,
            Vec3::new(0., -0.2, -0.46),
            Vec3::new(0.2, 0.1, 0.04),
            7,
        );
        for x in [-0.65, 0.65] {
            for z in [-0.6, 0.6] {
                self.part(
                    c,
                    e,
                    Vec3::new(x * 0.5, 0., z * 0.5),
                    Vec3::new(0.7, 0.06, 0.08),
                    0,
                );
                self.part(c, e, Vec3::new(x, 0.12, z), Vec3::new(0.8, 0.025, 0.13), 5);
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
#[derive(Component)]
pub struct Leg {
    pub phase: f32,
}
