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
