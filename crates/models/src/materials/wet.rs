//! Native wet surfaces extend Bevy's deferred PBR path.
use bevy::{
    asset::embedded_asset,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
#[derive(Component)]
pub struct WetSurface(pub bool); // true for ocean, false for asphalt
#[derive(Asset, AsBindGroup, TypePath, Clone, Debug)]
pub struct WetExtension {
    // x: time, y: ocean selector. Uniform is vec4 for portable alignment.
    #[uniform(100)]
    pub settings: Vec4,
}
pub type WetMaterial = ExtendedMaterial<StandardMaterial, WetExtension>;
impl MaterialExtension for WetExtension {
    fn deferred_fragment_shader() -> ShaderRef {
        "embedded://saltline_models/materials/wet.wesl".into()
    }
}
pub struct WetPlugin;
impl Plugin for WetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "wet.wesl");
        app.add_plugins(MaterialPlugin::<WetMaterial>::default())
            .add_systems(PostStartup, install);
    }
}
fn install(
    mut c: Commands,
    sources: Res<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<WetMaterial>>,
    surfaces: Query<(Entity, &MeshMaterial3d<StandardMaterial>, &WetSurface)>,
) {
    for (e, handle, kind) in &surfaces {
        let Some(mut base) = sources.get(&handle.0).cloned() else {
            continue;
        };
        base.uv_transform = bevy::math::Affine2::IDENTITY;
        if kind.0 {
            base.base_color_texture = None;
            base.normal_map_texture = None;
            base.metallic_roughness_texture = None;
        }
        c.entity(e)
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(MeshMaterial3d(materials.add(WetMaterial {
                base,
                extension: WetExtension {
                    settings: Vec4::new(1., kind.0 as u32 as f32, 0., 0.),
                },
            })));
    }
}
