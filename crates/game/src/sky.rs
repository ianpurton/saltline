//! Moving reference clouds; the static environment bake is shared by both cameras.
use crate::presentation::PresentationClock;
use bevy::{
    asset::{RenderAssetUsages, embedded_asset},
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    mesh::MeshVertexBufferLayoutRef,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::{
        AsBindGroup, CompareFunction, Extent3d, Face, RenderPipelineDescriptor,
        SpecializedMeshPipelineError, TextureDimension, TextureFormat,
    },
    shader::ShaderRef,
};

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct SkyMaterial {
    #[uniform(0)]
    sun: Vec4,
    // x: presentation seconds, y: environment mode; remaining lanes are padding.
    #[uniform(0)]
    params: Vec4,
    #[texture(1)]
    #[sampler(2)]
    noise: Handle<Image>,
}
impl Material for SkyMaterial {
    fn vertex_shader() -> ShaderRef {
        "embedded://saltline_game/sky.wesl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "embedded://saltline_game/sky.wesl".into()
    }
    fn enable_prepass() -> bool {
        false
    }
    fn enable_shadows() -> bool {
        false
    }
    fn opaque_render_method(&self) -> bevy::material::OpaqueRendererMethod {
        bevy::material::OpaqueRendererMethod::Forward
    }
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = Some(Face::Front);
        if let Some(depth) = &mut descriptor.depth_stencil {
            depth.depth_write_enabled = Some(false);
            depth.depth_compare = Some(CompareFunction::GreaterEqual);
        }
        Ok(())
    }
}
pub struct SkyPlugin;
impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "sky.wesl");
        app.add_plugins(MaterialPlugin::<SkyMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, animate.after(crate::presentation::clock));
    }
}
fn noise_bytes() -> Vec<u8> {
    let mut state = 2654435769u32;
    let mut pixels = Vec::with_capacity(256 * 256 * 4);
    for _ in 0..256 * 256 {
        for _ in 0..3 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            pixels.push((state >> 24) as u8);
        }
        pixels.push(255);
    }
    pixels
}
fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<SkyMaterial>>,
) {
    let mut noise = Image::new_fill(
        Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &noise_bytes(),
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    noise.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        ..default()
    });
    let material = materials.add(SkyMaterial {
        sun: saltline_models::materials::sky::SUN_DIRECTION
            .normalize()
            .extend(0.),
        params: Vec4::new(0., 0., 1., 0.),
        noise: images.add(noise),
    });
    c.spawn((
        Name::new("procedural storm sky"),
        Mesh3d(meshes.add(Cuboid::new(2., 2., 2.))),
        MeshMaterial3d(material),
        Transform::IDENTITY,
        bevy::camera::visibility::NoFrustumCulling,
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
    ));
}
fn animate(clock: Res<PresentationClock>, mut materials: ResMut<Assets<SkyMaterial>>) {
    for (_, material) in materials.iter_mut() {
        material.params.x = clock.elapsed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn noise_matches_javascript_reference() {
        let pixels = noise_bytes();
        let hash = pixels.iter().fold(2166136261u32, |hash, byte| {
            (hash ^ *byte as u32).wrapping_mul(16777619)
        });
        let reference: serde_json::Value =
            serde_json::from_str(include_str!("../../../assets/sky/reference-clouds.json"))
                .unwrap();
        assert_eq!(hash as u64, reference["noiseHash"].as_u64().unwrap());
        assert!(pixels.chunks_exact(4).all(|p| p[3] == 255));
    }
    #[test]
    fn sky_time_tracks_pause_and_restart() {
        let mut app = App::new();
        app.init_resource::<Assets<SkyMaterial>>()
            .init_resource::<PresentationClock>()
            .add_systems(Update, animate);
        let handle = app
            .world_mut()
            .resource_mut::<Assets<SkyMaterial>>()
            .add(SkyMaterial {
                sun: Vec4::X,
                params: Vec4::ZERO,
                noise: default(),
            });
        for seconds in [1., 30., 30., 0.] {
            app.world_mut().resource_mut::<PresentationClock>().elapsed = seconds;
            app.update();
            assert_eq!(
                app.world()
                    .resource::<Assets<SkyMaterial>>()
                    .get(&handle)
                    .unwrap()
                    .params
                    .x,
                seconds
            );
        }
    }
}
