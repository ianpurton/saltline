#[cfg(not(target_arch = "wasm32"))]
pub mod embedded;
pub mod industrial;
pub mod sky;
#[cfg(not(target_arch = "wasm32"))]
pub mod wet;
use bevy::prelude::*;
pub fn legacy(
    a: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) -> Vec<Handle<StandardMaterial>> {
    use bevy::{
        asset::RenderAssetUsages,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };
    // Deterministic, tileable weathering at several spatial frequencies.
    // Separate linear normal/ORM maps keep surface relief in the PBR lighting.
    let mut maps = Vec::new();
    for kind in 0..13 {
        let n = 512usize;
        let height = |x: usize, y: usize| -> f32 {
            let u = x as f32 / n as f32 * std::f32::consts::TAU;
            let v = y as f32 / n as f32 * std::f32::consts::TAU;
            let hash = ((x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(668265263))
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
            if kind == 10 {
                0.5 + 0.08 * (u * 35. + (v * 2.).sin()).sin() + grain * 0.03
            } else if kind == 8 {
                0.5 + 0.045 * (u * 12. + (v * 5.).sin()).sin() + grain * 0.015
            } else {
                if kind == 0 {
                    0.5 + patch * 0.12 + grain * 0.22
                } else if kind == 9 {
                    0.5 + noise(6) * 0.16 + grain * 0.1
                } else if kind == 5 {
                    0.55 + grain * 0.025
                } else {
                    0.65 + patch * 0.14 + grain * 0.025
                }
            }
        };
        let mut color = Vec::new();
        let mut normal = Vec::new();
        let mut orm = Vec::new();
        for y in 0..n {
            for x in 0..n {
                let h = height(x, y);
                let rust = kind > 0 && kind < 4 && h < 0.60;
                let shade = (155. + h * 100.) as u8;
                color.extend_from_slice(&if kind == 11 {
                    if ((x + y) / 64) % 2 == 0 {
                        [210, 158, 34, 255]
                    } else {
                        [22, 26, 28, 255]
                    }
                } else if rust {
                    [145, 108, 78, 255]
                } else {
                    [shade, shade, shade, 255]
                });
                let strength = if kind == 8 {
                    0.5
                } else if kind == 0 {
                    0.045
                } else {
                    0.10
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
        (0.38, 0.25, 0.12),
        (0.9, 0.9, 0.9),
        (0.08, 0.16, 0.2),
    ];
    #[cfg(not(target_arch = "wasm32"))]
    let authored = embedded::surfaces(images);
    colors
        .iter()
        .enumerate()
        .map(|(i, &(r, g, b))| {
            let mut material = StandardMaterial {
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
                    48.
                } else if i == 8 {
                    12.
                } else {
                    2.
                })),
                metallic: if i == 4 || i == 5 { 0.75 } else { 0. },
                perceptual_roughness: 1.,
                emissive: if i == 6 || i == 7 {
                    LinearRgba::rgb(r * 2., g * 2., b * 2.)
                } else {
                    LinearRgba::BLACK
                },
                ..default()
            };
            #[cfg(not(target_arch = "wasm32"))]
            if i == 0 || i == 9 {
                let maps = &authored[if i == 0 { 0 } else { 1 }];
                material.base_color = Color::srgb(0.7, 0.72, 0.73);
                material.base_color_texture = Some(maps.0.clone());
                material.normal_map_texture = Some(maps.1.clone());
                material.metallic_roughness_texture = Some(maps.2.clone());
                material.occlusion_texture = Some(maps.2.clone());
                material.uv_transform = bevy::math::Affine2::from_scale(if i == 0 {
                    Vec2::new(40.5, 52.5)
                } else {
                    Vec2::new(5., 10.)
                });
            }
            a.add(material)
        })
        .collect()
}
