//! Reference ground and water, with forward PBR for planar reflection composition.
use bevy::{
    asset::embedded_asset,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
#[derive(Component)]
pub struct WetSurface(pub bool); // true for ocean, false for asphalt
#[derive(Component)]
pub struct GroundReflectionCamera;
#[derive(Asset, AsBindGroup, TypePath, Clone, Debug, Default)]
pub struct WetExtension {
    // x: time, y: ocean selector. Uniform is vec4 for portable alignment.
    #[uniform(100)]
    pub settings: Vec4,
    #[uniform(100)]
    pub reflection_matrix: Mat4,
    #[uniform(100)]
    pub reflection: Vec4,
    #[texture(101)]
    #[sampler(102)]
    pub mask: Handle<Image>,
    #[texture(103)]
    #[sampler(104)]
    pub marks: Handle<Image>,
    #[texture(105)]
    #[sampler(106)]
    pub detail: Handle<Image>,
    #[texture(107)]
    #[sampler(108)]
    pub normal: Handle<Image>,
    #[texture(109)]
    #[sampler(110)]
    pub reflected: Handle<Image>,
}
pub type WetMaterial = ExtendedMaterial<StandardMaterial, WetExtension>;
impl MaterialExtension for WetExtension {
    fn fragment_shader() -> ShaderRef {
        "embedded://saltline_models/materials/wet.wesl".into()
    }
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
            .add_systems(Update, switch_wet)
            .add_systems(PostUpdate, animate_ocean);
    }
}
fn install(
    mut c: Commands,
    mut sources: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<WetMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    surfaces: Query<(Entity, &MeshMaterial3d<StandardMaterial>, &WetSurface)>,
) {
    let maps = super::ground::GroundMaps::new(&mut images);
    let reflected = images.add(Image::default());
    c.insert_resource(maps.clone());
    for (e, handle, kind) in &surfaces {
        let Some(mut base) = sources.get(&handle.0).cloned() else {
            continue;
        };
        let simple = sources.add(base.clone());
        base.uv_transform = bevy::math::Affine2::IDENTITY;
        // Shader owns surface color, normals and roughness; do not multiply authored maps twice.
        base.base_color = Color::WHITE;
        base.base_color_texture = None;
        base.normal_map_texture = None;
        base.metallic_roughness_texture = None;
        base.occlusion_texture = None;
        base.perceptual_roughness = 0.7;
        base.opaque_render_method = bevy::material::OpaqueRendererMethod::Forward;
        if kind.0 {
            let (mesh, rest) = ocean_mesh();
            c.entity(e).insert((
                Mesh3d(meshes.add(mesh)),
                Transform::IDENTITY,
                OceanMesh(rest),
                bevy::camera::visibility::NoFrustumCulling,
            ));
        }
        c.entity(e).insert((
            bevy::light::NotShadowCaster,
            bevy::camera::visibility::RenderLayers::layer(1),
        ));
        let detailed = materials.add(WetMaterial {
            base,
            extension: WetExtension {
                settings: Vec4::new(1., kind.0 as u32 as f32, 0.6, 1.),
                mask: maps.mask.clone(),
                marks: maps.marks.clone(),
                detail: maps.detail.clone(),
                normal: maps.normal.clone(),
                reflected: reflected.clone(),
                ..default()
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
                ..default()
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

#[derive(Component)]
struct OceanMesh(Vec<Vec3>);
fn ocean_mesh() -> (Mesh, Vec<Vec3>) {
    use bevy::{
        asset::RenderAssetUsages,
        mesh::{Indices, PrimitiveTopology},
    };
    let axis = |start: f32, end: f32, step: f32, growth: f32| {
        let mut values = vec![start];
        let mut x = start;
        let mut delta = step;
        while x < end {
            x = (x + delta).min(end);
            values.push(x);
            delta = (delta * growth).min(120.);
        }
        values
    };
    let xs = axis(27.5, 880., 0.75, 1.075);
    let outer = axis(48., 880., 0.9, 1.085);
    let zs: Vec<_> = outer
        .iter()
        .rev()
        .map(|z| -z)
        .chain(axis(-47.1, 47.1, 0.9, 1.))
        .chain(outer.iter().copied())
        .collect();
    let mut rest: Vec<_> = zs
        .iter()
        .flat_map(|&z| xs.iter().map(move |&x| Vec3::new(x, -1.6, z)))
        .collect();
    let mut indices = Vec::new();
    for z in 0..zs.len() - 1 {
        for x in 0..xs.len() - 1 {
            let a = (z * xs.len() + x) as u32;
            let b = a + xs.len() as u32;
            indices.extend([a, b, a + 1, a + 1, b, b + 1]);
        }
    }
    // Same far-water skirt as the reference; keep the dense grid near the quay.
    for (x0, z0, x1, z1) in [
        (-880., -880., 27.5, -158.),
        (-880., 158., 27.5, 880.),
        (-880., -158., -158., 158.),
    ] {
        let base = rest.len() as u32;
        rest.extend([
            Vec3::new(x0, -1.6, z0),
            Vec3::new(x1, -1.6, z0),
            Vec3::new(x0, -1.6, z1),
            Vec3::new(x1, -1.6, z1),
        ]);
        indices.extend([base, base + 2, base + 1, base + 1, base + 2, base + 3]);
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        rest.iter().map(|p| p.to_array()).collect::<Vec<_>>(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; rest.len()]);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        rest.iter().map(|p| [p.x, p.z]).collect::<Vec<_>>(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, vec![[1., 0., 0., 1.]; rest.len()]);
    mesh.insert_indices(Indices::U32(indices));
    (mesh, rest)
}
fn animate_ocean(
    materials: Res<Assets<WetMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    oceans: Query<(&Mesh3d, &OceanMesh)>,
    mut previous: Local<Option<f32>>,
) {
    let Some((_, material)) = materials.iter().find(|(_, m)| m.extension.settings.y > 0.5) else {
        return;
    };
    let time = material.extension.settings.x;
    if *previous == Some(time) {
        return;
    }
    *previous = Some(time);
    for (handle, rest) in &oceans {
        let Some(mut mesh) = meshes.get_mut(&handle.0) else {
            continue;
        };
        let Some(bevy::mesh::VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        else {
            continue;
        };
        for (position, p) in positions.iter_mut().zip(&rest.0) {
            *position = (*p + super::ground::sea_offset(p.xz(), time)).to_array();
        }
    }
}
