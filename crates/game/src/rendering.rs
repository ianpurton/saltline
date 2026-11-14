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
        bevy::camera::Hdr,
        bevy::core_pipeline::prepass::DeferredPrepass,
        bevy::core_pipeline::prepass::DepthPrepass,
    )
}
#[cfg(not(target_arch = "wasm32"))]
pub fn configure(
    mut c: Commands,
    quality: Res<crate::quality::RenderQuality>,
    cameras: Query<Entity, With<Camera3d>>,
    mut lights: Query<(&mut DirectionalLight, &mut bevy::light::CascadeShadowConfig)>,
    mut shadow_map: ResMut<bevy::light::DirectionalLightShadowMap>,
    mut rain: Query<&mut Visibility, With<crate::rain::Rain>>,
    mut previous: Local<Option<Vec<u32>>>,
) {
    let key = vec![
        quality.ssr as u32,
        quality.ao as u32,
        quality.contact as u32,
        quality.volumetric as u32,
        quality.taa as u32,
        quality.bloom as u32,
        quality.shadows,
        quality.rain as u32,
        quality.detailed_wet as u32,
        quality.detailed_containers as u32,
        quality.high as u32,
    ];
    if previous.as_ref() == Some(&key) {
        return;
    }
    *previous = Some(key);
    for e in &cameras {
        let mut camera = c.entity(e);
        macro_rules! effect {
            ($enabled:expr, $ty:ty, $value:expr) => {
                if $enabled {
                    camera.insert($value);
                } else {
                    camera.remove::<$ty>();
                }
            };
        }
        effect!(
            quality.ssr,
            bevy::pbr::ScreenSpaceReflections,
            bevy::pbr::ScreenSpaceReflections::default()
        );
        effect!(
            quality.ao,
            bevy::pbr::ScreenSpaceAmbientOcclusion,
            bevy::pbr::ScreenSpaceAmbientOcclusion::default()
        );
        effect!(
            quality.contact,
            bevy::pbr::ContactShadows,
            bevy::pbr::ContactShadows::default()
        );
        effect!(
            quality.volumetric,
            bevy::light::VolumetricFog,
            bevy::light::VolumetricFog::default()
        );
        effect!(
            quality.taa,
            bevy::anti_alias::taa::TemporalAntiAliasing,
            bevy::anti_alias::taa::TemporalAntiAliasing::default()
        );
        effect!(
            quality.bloom,
            bevy::post_process::bloom::Bloom,
            bevy::post_process::bloom::Bloom {
                intensity: 0.045,
                ..default()
            }
        );
    }
    for (mut light, mut cascades) in &mut lights {
        light.shadow_maps_enabled = quality.shadows != 0;
        *cascades = bevy::light::CascadeShadowConfigBuilder {
            maximum_distance: if quality.high { 170. } else { 100. },
            num_cascades: if quality.high { 4 } else { 2 },
            first_cascade_far_bound: 16.,
            ..default()
        }
        .build();
    }
    shadow_map.size = quality.shadows.max(512) as usize;
    for mut visibility in &mut rain {
        *visibility = if quality.rain {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    c.insert_resource(saltline_models::materials::wet::SimpleWetSurfaces(
        !quality.detailed_wet,
    ));
    c.insert_resource(saltline_models::geometry::batch::CoarseContainers(
        !quality.detailed_containers,
    ));
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

#[cfg(target_arch = "wasm32")]
pub fn configure() {}

/// The sky is static. Retain the filtered textures and retire the realtime GPU
/// convolution once startup pipelines have settled (for BOTH camera views).
#[cfg(not(target_arch = "wasm32"))]
pub fn freeze_environment_maps(
    mut c: Commands,
    ready: Res<crate::capture::RenderReady>,
    cameras: Query<
        (Entity, &bevy::light::EnvironmentMapLight),
        With<bevy::light::GeneratedEnvironmentMapLight>,
    >,
) {
    if !ready.ready() {
        return;
    }
    let Some((_, shared)) = cameras.iter().next() else {
        return;
    };
    for (entity, _) in &cameras {
        c.entity(entity)
            .remove::<bevy::light::GeneratedEnvironmentMapLight>()
            .insert(shared.clone());
    }
    info!("ENVIRONMENT cached static sky; disabled per-frame diffuse/specular convolution");
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    #[test]
    fn live_quality_updates_both_cameras_and_restores_effects() {
        let mut app = App::new();
        let cameras = [
            app.world_mut().spawn(Camera3d::default()).id(),
            app.world_mut().spawn(Camera3d::default()).id(),
        ];
        let light = app
            .world_mut()
            .spawn((
                DirectionalLight::default(),
                bevy::light::CascadeShadowConfig::default(),
            ))
            .id();
        let rain = app
            .world_mut()
            .spawn((crate::rain::Rain, Visibility::Inherited))
            .id();
        app.insert_resource(crate::quality::RenderQuality::preset("high"))
            .insert_resource(bevy::light::DirectionalLightShadowMap::default())
            .add_systems(Update, configure);
        app.update();
        for camera in cameras {
            assert!(
                app.world()
                    .get::<bevy::pbr::ScreenSpaceReflections>(camera)
                    .is_some()
            );
        }
        {
            let mut q = app
                .world_mut()
                .resource_mut::<crate::quality::RenderQuality>();
            q.ssr = false;
            q.ao = false;
            q.volumetric = false;
            q.bloom = false;
            q.taa = false;
            q.shadows = 0;
            q.rain = false;
        }
        app.update();
        for camera in cameras {
            assert!(
                app.world()
                    .get::<bevy::pbr::ScreenSpaceReflections>(camera)
                    .is_none()
            );
            assert!(
                app.world()
                    .get::<bevy::post_process::bloom::Bloom>(camera)
                    .is_none()
            );
            assert!(
                app.world()
                    .get::<bevy::anti_alias::taa::TemporalAntiAliasing>(camera)
                    .is_none()
            );
        }
        assert!(
            !app.world()
                .get::<DirectionalLight>(light)
                .unwrap()
                .shadow_maps_enabled
        );
        assert_eq!(
            *app.world().get::<Visibility>(rain).unwrap(),
            Visibility::Hidden
        );
        *app.world_mut()
            .resource_mut::<crate::quality::RenderQuality>() =
            crate::quality::RenderQuality::preset("high");
        app.update();
        for camera in cameras {
            assert!(
                app.world()
                    .get::<bevy::pbr::ScreenSpaceReflections>(camera)
                    .is_some()
            );
        }
        assert!(
            app.world()
                .get::<DirectionalLight>(light)
                .unwrap()
                .shadow_maps_enabled
        );
        assert_eq!(
            *app.world().get::<Visibility>(rain).unwrap(),
            Visibility::Inherited
        );
    }
}
