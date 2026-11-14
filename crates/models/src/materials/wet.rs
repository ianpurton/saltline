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
#[derive(Resource, Default)]
pub struct SimpleWetSurfaces(pub bool);
#[derive(Component)]
struct WetVariants {
    simple: Handle<StandardMaterial>,
    detailed: Handle<WetMaterial>,
}
pub struct WetPlugin;
impl Plugin for WetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "wet.wesl");
        app.add_plugins(MaterialPlugin::<WetMaterial>::default())
            .init_resource::<SimpleWetSurfaces>()
            .add_systems(PostStartup, install)
            .add_systems(Update, switch_wet);
    }
}
fn install(
    mut c: Commands,
    mut sources: ResMut<Assets<StandardMaterial>>,
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
        let simple = sources.add(base.clone());
        let detailed = materials.add(WetMaterial {
            base,
            extension: WetExtension {
                settings: Vec4::new(1., kind.0 as u32 as f32, 0., 0.),
            },
        });
        c.entity(e)
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert((
                MeshMaterial3d(detailed.clone()),
                WetVariants { simple, detailed },
            ));
    }
}
fn switch_wet(
    mut commands: Commands,
    simple: Res<SimpleWetSurfaces>,
    surfaces: Query<(Entity, &WetVariants, Has<MeshMaterial3d<WetMaterial>>)>,
) {
    for (entity, variants, detailed) in &surfaces {
        if simple.0 && detailed {
            commands
                .entity(entity)
                .remove::<MeshMaterial3d<WetMaterial>>()
                .insert(MeshMaterial3d(variants.simple.clone()));
        } else if !simple.0 && !detailed {
            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .insert(MeshMaterial3d(variants.detailed.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_material_switch_restores_cached_detailed_handle() {
        let mut app = App::new();
        let mut standard = Assets::<StandardMaterial>::default();
        let simple = standard.add(StandardMaterial::default());
        let mut wet = Assets::<WetMaterial>::default();
        let detailed = wet.add(WetMaterial {
            base: StandardMaterial::default(),
            extension: WetExtension {
                settings: Vec4::ZERO,
            },
        });
        let entity = app
            .world_mut()
            .spawn((
                MeshMaterial3d(detailed.clone()),
                WetVariants {
                    simple: simple.clone(),
                    detailed: detailed.clone(),
                },
            ))
            .id();
        app.insert_resource(SimpleWetSurfaces(true))
            .add_systems(Update, switch_wet);
        app.update();
        assert_eq!(
            app.world()
                .get::<MeshMaterial3d<StandardMaterial>>(entity)
                .unwrap()
                .0,
            simple
        );
        assert!(
            app.world()
                .get::<MeshMaterial3d<WetMaterial>>(entity)
                .is_none()
        );
        app.world_mut().resource_mut::<SimpleWetSurfaces>().0 = false;
        app.update();
        assert_eq!(
            app.world()
                .get::<MeshMaterial3d<WetMaterial>>(entity)
                .unwrap()
                .0,
            detailed
        );
        assert!(
            app.world()
                .get::<MeshMaterial3d<StandardMaterial>>(entity)
                .is_none()
        );
    }
}
