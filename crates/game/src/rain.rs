//! One draw and one uniform update for camera-local rain; motion runs on the GPU.
use bevy::{
    asset::embedded_asset, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef,
};
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct RainMaterial {
    #[uniform(0)]
    motion: Vec4,
    #[uniform(0)]
    right: Vec4,
}
impl Material for RainMaterial {
    fn vertex_shader() -> ShaderRef {
        "embedded://saltline_game/rain.wesl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "embedded://saltline_game/rain.wesl".into()
    }
    fn enable_prepass() -> bool {
        false
    }
    fn enable_shadows() -> bool {
        false
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
pub struct RainPlugin;
impl Plugin for RainPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "rain.wesl");
        app.add_plugins(MaterialPlugin::<RainMaterial>::default());
    }
}
pub fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RainMaterial>>,
    quality: Res<crate::quality::RenderQuality>,
) {
    use bevy::{
        asset::RenderAssetUsages,
        mesh::{Indices, PrimitiveTopology},
    };
    let count = if quality.high { 1400 } else { 800 };
    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    for i in 0..count {
        let p = Vec3::new(
            ((i * 73) % 440) as f32 * 0.1 - 22.,
            ((i * 31) % 180) as f32 * 0.1,
            ((i * 43) % 440) as f32 * 0.1 - 22.,
        );
        let base = positions.len() as u32;
        for uv in [Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y] {
            positions.push(p.to_array());
            uvs.push(uv.to_array());
        }
        indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices));
    c.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(RainMaterial {
            motion: Vec4::ZERO,
            right: Vec4::X,
        })),
        Transform::default(),
        bevy::camera::visibility::NoFrustumCulling,
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
    ));
}
pub fn update(
    clock: Res<crate::presentation::PresentationClock>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut materials: ResMut<Assets<RainMaterial>>,
    performance: Option<Res<saltline_game::performance::PerformanceSettings>>,
    diagnostics: Option<bevy::diagnostic::Diagnostics>,
) {
    let _trace = info_span!("rain::update").entered();
    let _cpu = saltline_game::performance::CpuMeasurement::new(
        performance.as_deref(),
        diagnostics,
        "saltline/system/rain/update_ms",
    );
    let Some((_, camera)) = cameras.iter().find(|(c, _)| c.is_active) else {
        return;
    };
    let p = camera.translation();
    for (_, m) in materials.iter_mut() {
        m.motion = Vec4::new(clock.elapsed, p.x, p.z, 0.);
        m.right = camera.right().extend(0.);
    }
}
