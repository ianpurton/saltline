use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
    },
};
/// Seamless directional cloud field, baked once. Uses Bevy's ordinary skybox and
/// filtered environment lighting rather than a per-frame cloud ray marcher.
pub fn overcast(images: &mut Assets<Image>) -> Handle<Image> {
    let n = 128;
    let mut pixels = Vec::with_capacity(n * n * 6 * 4);
    for face in 0..6 {
        for y in 0..n {
            for x in 0..n {
                let u = 2. * (x as f32 + 0.5) / n as f32 - 1.;
                let v = 2. * (y as f32 + 0.5) / n as f32 - 1.;
                let d = match face {
                    0 => Vec3::new(1., -v, -u),
                    1 => Vec3::new(-1., -v, u),
                    2 => Vec3::new(u, 1., v),
                    3 => Vec3::new(u, -1., -v),
                    4 => Vec3::new(u, -v, 1.),
                    _ => Vec3::new(-u, -v, -1.),
                }
                .normalize();
                let mut field = 0.;
                let mut weight = 0.5;
                let mut q = d * 4.;
                for _ in 0..5 {
                    field += (q.x + 0.6 * q.z.sin()).sin() * (q.z * 0.8 + q.y).cos() * weight;
                    q = q * 2.13 + Vec3::new(1.7, 3.1, 0.8);
                    weight *= 0.5;
                }
                let horizon = (1. - d.y.max(0.)).powf(4.);
                let sun = d
                    .dot(Vec3::new(65., 35., -80.).normalize())
                    .max(0.)
                    .powf(24.);
                let cloud = ((field + 0.2) * 1.5).clamp(0., 1.) * (0.3 + d.y.max(0.).sqrt());
                let c = Vec3::new(0.16, 0.23, 0.32).lerp(Vec3::new(0.52, 0.49, 0.4), horizon)
                    + Vec3::splat(cloud * 0.16)
                    + Vec3::new(0.38, 0.26, 0.12) * sun;
                pixels.extend_from_slice(&[
                    (c.x * 255.) as u8,
                    (c.y * 255.) as u8,
                    (c.z * 255.) as u8,
                    255,
                ]);
            }
        }
    }
    let mut image = Image::new_fill(
        Extent3d {
            width: n as u32,
            height: n as u32,
            depth_or_array_layers: 6,
        },
        TextureDimension::D2,
        &pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    images.add(image)
}

/// Convert the bundled linear HDR panorama to a filtered-light-compatible cubemap.
#[cfg(not(target_arch = "wasm32"))]
pub fn photographic(images: &mut Assets<Image>) -> Handle<Image> {
    let pano = image::load_from_memory_with_format(
        include_bytes!("../../../../assets/sky/industrial_sunset_02_puresky_2k.hdr"),
        image::ImageFormat::Hdr,
    )
    .expect("bundled HDR panorama must decode")
    .to_rgb32f();
    let (w, h) = pano.dimensions();
    let peak = pano
        .enumerate_pixels()
        .filter(|(_, y, _)| *y < h / 2)
        .max_by(|a, b| {
            a.2.0
                .iter()
                .sum::<f32>()
                .total_cmp(&b.2.0.iter().sum::<f32>())
        })
        .map(|(x, _, _)| x)
        .unwrap_or(w / 2);
    let source_angle = (peak as f32 / w as f32 - 0.5) * std::f32::consts::TAU;
    let desired_angle = (-0.86f32).atan2(0.48);
    let rotation = Quat::from_rotation_y(desired_angle - source_angle);
    let n = 512u32;
    let mut bytes = Vec::with_capacity((n * n * 6 * 8) as usize);
    for face in 0..6 {
        for y in 0..n {
            for x in 0..n {
                let u = 2. * (x as f32 + 0.5) / n as f32 - 1.;
                let v = 2. * (y as f32 + 0.5) / n as f32 - 1.;
                let d = match face {
                    0 => Vec3::new(1., -v, -u),
                    1 => Vec3::new(-1., -v, u),
                    2 => Vec3::new(u, 1., v),
                    3 => Vec3::new(u, -1., -v),
                    4 => Vec3::new(u, -v, 1.),
                    _ => Vec3::new(-u, -v, -1.),
                }
                .normalize();
                let d = rotation * d;
                let px = ((d.z.atan2(d.x) / std::f32::consts::TAU + 0.5) * w as f32)
                    .rem_euclid(w as f32);
                let py = (d.y.acos() / std::f32::consts::PI * h as f32).clamp(0., h as f32 - 1.001);
                let x0 = px as u32;
                let y0 = py as u32;
                let f = Vec2::new(px.fract(), py.fract());
                let a = Vec3::from_array(pano.get_pixel(x0, y0).0)
                    .lerp(Vec3::from_array(pano.get_pixel((x0 + 1) % w, y0).0), f.x);
                let b = Vec3::from_array(pano.get_pixel(x0, (y0 + 1).min(h - 1)).0).lerp(
                    Vec3::from_array(pano.get_pixel((x0 + 1) % w, (y0 + 1).min(h - 1)).0),
                    f.x,
                );
                let color = a.lerp(b, f.y).min(Vec3::splat(24.));
                for c in [color.x, color.y, color.z, 1.] {
                    bytes.extend_from_slice(&half::f16::from_f32(c).to_le_bytes());
                }
            }
        }
    }
    let mut image = Image::new_fill(
        Extent3d {
            width: n,
            height: n,
            depth_or_array_layers: 6,
        },
        TextureDimension::D2,
        &bytes,
        TextureFormat::Rgba16Float,
        RenderAssetUsages::default(),
    );
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    images.add(image)
}
