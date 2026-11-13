//! Keep both cameras on the same platform rendering profile.
use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};

#[cfg(not(target_arch = "wasm32"))]
pub fn camera_effects(sky: Handle<Image>, intensity: f32) -> impl Bundle {
    (
        bevy::light::GeneratedEnvironmentMapLight {
            environment_map: sky,
            intensity,
            ..default()
        },
        Tonemapping::AcesFitted,
        Msaa::Off,
        bevy::pbr::ScreenSpaceReflections::default(),
        bevy::pbr::ContactShadows::default(),
        bevy::pbr::ScreenSpaceAmbientOcclusion::default(),
        bevy::light::VolumetricFog::default(),
        bevy::anti_alias::taa::TemporalAntiAliasing::default(),
        bevy::camera::Hdr,
        bevy::post_process::bloom::Bloom {
            intensity: 0.07,
            ..default()
        },
    )
}

// WebGL 2 uses forward LDR rendering without compute or extra screen-space passes.
#[cfg(target_arch = "wasm32")]
pub fn camera_effects(_sky: Handle<Image>, _intensity: f32) -> impl Bundle {
    (
        Tonemapping::None,
        bevy::core_pipeline::tonemapping::DebandDither::Disabled,
        Msaa::Off,
    )
}

#[derive(Resource, Clone)]
pub struct SharedSky(pub Handle<Image>);
pub const SKY_BRIGHTNESS: f32 = 350.;
pub const ENVIRONMENT_BRIGHTNESS: f32 = 450.;
pub fn fog() -> DistanceFog {
    DistanceFog {
        color: Color::srgb(0.32, 0.38, 0.42),
        falloff: FogFalloff::ExponentialSquared { density: 0.004 },
        ..default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn mist(c: &mut Commands, images: &mut Assets<Image>) {
    use bevy::{
        asset::RenderAssetUsages,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };
    let n = 32;
    let mut data = Vec::new();
    for z in 0..n {
        for y in 0..n {
            for x in 0..n {
                let p = Vec3::new(x as f32, y as f32, z as f32) / (n - 1) as f32;
                let edge = (p.x * (1. - p.x) * p.z * (1. - p.z) * 16.).powi(2);
                let height = (p.y * (1. - p.y) * 4.).powi(3);
                let noise =
                    0.65 + 0.35 * (p.x * 17. + p.z * 11.).sin() * (p.z * 23. - p.y * 8.).cos();
                data.push((255. * edge * height * noise) as u8);
            }
        }
    }
    let image = images.add(Image::new_fill(
        Extent3d {
            width: n,
            height: n,
            depth_or_array_layers: n,
        },
        TextureDimension::D3,
        &data,
        TextureFormat::R8Unorm,
        RenderAssetUsages::default(),
    ));
    c.spawn((
        bevy::light::FogVolume {
            density_factor: 0.004,
            density_texture: Some(image),
            fog_color: Color::srgb(0.65, 0.70, 0.73),
            ..default()
        },
        Transform::from_xyz(35., 9., -145.).with_scale(Vec3::new(320., 40., 280.)),
    ));
}
