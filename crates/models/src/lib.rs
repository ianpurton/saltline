use bevy::prelude::*;
/// Shared primitive cache: every procedural object uses these meshes and materials.
pub struct Kit {
    pub cube: Handle<Mesh>,
    pub ball: Handle<Mesh>,
    pub puddle: Handle<Mesh>,
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
        // Deterministic, tileable weathering at several spatial frequencies.
        // Separate linear normal/ORM maps keep surface relief in the PBR lighting.
        let mut maps = Vec::new();
        for kind in 0..10 {
            let n = 512usize;
            let height = |x: usize, y: usize| -> f32 {
                let u = x as f32 / n as f32 * std::f32::consts::TAU;
                let v = y as f32 / n as f32 * std::f32::consts::TAU;
                let hash = ((x as u32).wrapping_mul(374761393)
                    ^ (y as u32).wrapping_mul(668265263))
                .wrapping_mul(1274126177);
                let grain = (hash >> 24) as f32 / 255.;
                let noise = |scale: usize| {
                    let sample = |a: usize, b: usize| {
                        let h = ((a % scale) as u32).wrapping_mul(374761393)
                            ^ ((b % scale) as u32).wrapping_mul(668265263);
                        (h.wrapping_mul(1274126177) >> 16) as f32 / 65535.
                    };
                    let xx = x as f32 / n as f32 * scale as f32;
                    let yy = y as f32 / n as f32 * scale as f32;
                    let a = xx.floor() as usize;
                    let b = yy.floor() as usize;
                    let fx = xx.fract();
                    let fy = yy.fract();
                    let fx = fx * fx * (3. - 2. * fx);
                    let fy = fy * fy * (3. - 2. * fy);
                    (sample(a, b) * (1. - fx) + sample(a + 1, b) * fx) * (1. - fy)
                        + (sample(a, b + 1) * (1. - fx) + sample(a + 1, b + 1) * fx) * fy
                };
                let patch = noise(16) * 0.65 + noise(47) * 0.35 - 0.5;
                if kind == 8 {
                    0.5 + 0.045 * (u * 12. + (v * 5.).sin()).sin() + grain * 0.015
                } else {
                    0.5 + patch * 0.6 + grain * if kind == 0 { 0.12 } else { 0.06 }
                }
            };
            let mut color = Vec::new();
            let mut normal = Vec::new();
            let mut orm = Vec::new();
            for y in 0..n {
                for x in 0..n {
                    let h = height(x, y);
                    let rust = kind > 0 && kind < 4 && h < 0.39;
                    let shade = (155. + h * 100.) as u8;
                    color.extend_from_slice(&if rust {
                        [130, 81, 47, 255]
                    } else {
                        [shade, shade, shade, 255]
                    });
                    let strength = if kind == 8 {
                        0.5
                    } else if kind == 0 {
                        0.045
                    } else {
                        0.35
                    };
                    let dx = (height((x + 1) % n, y) - height((x + n - 1) % n, y)) * strength;
                    let dy = (height(x, (y + 1) % n) - height(x, (y + n - 1) % n)) * strength;
                    let nn = Vec3::new(-dx, -dy, 1.).normalize();
                    normal.extend_from_slice(&[
                        (nn.x * 127. + 128.) as u8,
                        (nn.y * 127. + 128.) as u8,
                        (nn.z * 127. + 128.) as u8,
                        255,
                    ]);
                    let rough = if kind == 8 {
                        0.12 + h * 0.12
                    } else if kind == 0 {
                        0.32 + h * 0.6
                    } else {
                        0.42 + h * 0.4
                    };
                    orm.extend_from_slice(&[
                        255,
                        (rough * 255.) as u8,
                        if kind == 0 || kind == 8 {
                            0
                        } else if rust {
                            35
                        } else {
                            175
                        },
                        255,
                    ]);
                }
            }
            let mut add = |bytes: Vec<u8>, format| {
                let mut image = Image::new_fill(
                    Extent3d {
                        width: n as u32,
                        height: n as u32,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &bytes,
                    format,
                    RenderAssetUsages::default(),
                );
                image.sampler =
                    bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                        address_mode_u: bevy::image::ImageAddressMode::Repeat,
                        address_mode_v: bevy::image::ImageAddressMode::Repeat,
                        ..default()
                    });
                images.add(image)
            };
            maps.push((
                add(color, TextureFormat::Rgba8UnormSrgb),
                add(normal, TextureFormat::Rgba8Unorm),
                add(orm, TextureFormat::Rgba8Unorm),
            ));
        }
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
            (0.32, 0.33, 0.30),
        ];
        Self {
            cube: m.add({
                let mut mesh = Mesh::from(Cuboid::default());
                let _ = mesh.generate_tangents();
                mesh
            }),
            ring: m.add(Torus::new(0.28, 0.36)),
            puddle: m.add({
                let mut mesh = Mesh::from(Circle::new(0.5).mesh().resolution(40));
                if let Some(bevy::mesh::VertexAttributeValues::Float32x3(positions)) =
                    mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
                {
                    for p in positions {
                        let angle = p[1].atan2(p[0]);
                        let r = 1. + 0.16 * (angle * 5.).sin() + 0.09 * (angle * 9.).cos();
                        p[0] *= r;
                        p[1] *= r;
                    }
                }
                let _ = mesh.generate_tangents();
                mesh
            }),
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
                            Some(maps[i].0.clone())
                        },
                        normal_map_texture: if i == 6 || i == 7 {
                            None
                        } else {
                            Some(maps[i].1.clone())
                        },
                        metallic_roughness_texture: Some(maps[i].2.clone()),
                        uv_transform: bevy::math::Affine2::from_scale(Vec2::splat(if i == 0 {
                            24.
                        } else if i == 8 {
                            12.
                        } else {
                            2.
                        })),
                        metallic: if i == 0 || i == 8 || i == 9 { 0. } else { 0.7 },
                        perceptual_roughness: 1.,
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
        // Armour panels, captive bolts, top rails and hydraulic actuators.
        for x in [-0.365f32, 0.365] {
            for z in [-0.32, 0.2] {
                self.part(c, e, Vec3::new(x, 0.98, z), Vec3::new(0.055, 0.29, 0.42), 4);
                for y in [0.87, 1.09] {
                    for dz in [-0.16, 0.16] {
                        self.joint(c, e, Vec3::new(x * 1.1, y, z + dz), 0.035);
                    }
                }
            }
            self.part(
                c,
                e,
                Vec3::new(x, 1.34, 0.),
                Vec3::new(0.035, 0.035, 1.12),
                5,
            );
            for z in [-0.45, 0.45] {
                self.part(
                    c,
                    e,
                    Vec3::new(x, 1.28, z),
                    Vec3::new(0.035, 0.16, 0.035),
                    5,
                );
                self.part(
                    c,
                    e,
                    Vec3::new(x * 1.3, 0.52, z + 0.03),
                    Vec3::new(0.045, 0.54, 0.045),
                    5,
                );
                self.part(
                    c,
                    e,
                    Vec3::new(x * 1.3, 0.34, z + 0.03),
                    Vec3::new(0.025, 0.3, 0.025),
                    4,
                );
            }
        }
        for x in [-0.15, 0.15] {
            self.joint(c, e, Vec3::new(x, 1.1, -1.04), 0.15);
            self.part(
                c,
                e,
                Vec3::new(x, 1.1, -1.12),
                Vec3::new(0.075, 0.065, 0.018),
                7,
            );
        }

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
        for x in [-0.28, 0.28] {
            self.part(c, e, Vec3::new(x, 0.18, 0.), Vec3::new(0.08, 0.06, 0.65), 0);
            for z in [-0.28, 0.28] {
                self.joint(c, e, Vec3::new(x, 0.22, z), 0.04);
            }
            self.part(
                c,
                e,
                Vec3::new(x, -0.33, 0.22),
                Vec3::new(0.035, 0.55, 0.035),
                5,
            );
            self.part(
                c,
                e,
                Vec3::new(x, -0.59, 0.13),
                Vec3::new(0.045, 0.045, 0.5),
                0,
            );
        }

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
