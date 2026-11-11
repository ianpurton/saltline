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
        bevy::light::VolumetricFog::default(),
        bevy::anti_alias::taa::TemporalAntiAliasing::default(),
        bevy::camera::Hdr,
        bevy::post_process::bloom::Bloom::default(),
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
