//! Half-resolution planar reflection of layer 0; wet ground and water occupy layer 1.
use bevy::{
    camera::{CameraProjection, RenderTarget, SubCameraView, visibility::RenderLayers},
    core_pipeline::tonemapping::Tonemapping,
    math::Vec3A,
    prelude::*,
    render::render_resource::TextureFormat,
    transform::TransformSystems,
};
use saltline_models::materials::wet::{GroundReflectionCamera, WetMaterial};

#[derive(Resource)]
struct ReflectionTarget(Handle<Image>);
#[derive(Clone, Debug)]
struct ReflectedProjection {
    perspective: PerspectiveProjection,
    plane: Vec4,
}
impl CameraProjection for ReflectedProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        let projection = self.perspective.get_clip_from_view();
        let q =
            projection.inverse() * Vec4::new(self.plane.x.signum(), self.plane.y.signum(), 0., 1.);
        // Bevy's reversed Z near plane is row W minus row Z, and the far plane is Z=0.
        let c = self.plane / self.plane.dot(q);
        let row = projection.row(3) - c;
        let mut matrix = projection.to_cols_array_2d();
        for i in 0..4 {
            matrix[i][2] = row[i];
        }
        Mat4::from_cols_array_2d(&matrix)
    }
    fn get_clip_from_view_for_sub(&self, _sub: &SubCameraView) -> Mat4 {
        self.get_clip_from_view()
    }
    fn update(&mut self, width: f32, height: f32) {
        self.perspective.update(width, height);
    }
    fn far(&self) -> f32 {
        self.perspective.far
    }
    fn get_frustum_corners(&self, near: f32, far: f32) -> [Vec3A; 8] {
        self.perspective.get_frustum_corners(near, far)
    }
}
pub struct GroundReflectionsPlugin;
impl Plugin for GroundReflectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            PostUpdate,
            sync.after(TransformSystems::Propagate)
                .before(bevy::camera::CameraUpdateSystems),
        );
    }
}
fn setup(mut c: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_target_texture(640, 360, TextureFormat::Rgba16Float, None);
    image.sampler = bevy::image::ImageSampler::linear();
    let target = images.add(image);
    c.spawn((
        Name::new("ground planar reflection"),
        GroundReflectionCamera,
        Camera3d::default(),
        Camera {
            order: -2,
            is_active: false,
            ..default()
        },
        RenderTarget::Image(target.clone().into()),
        RenderLayers::layer(0),
        bevy::camera::Hdr,
        Msaa::Off,
        Tonemapping::Linear,
    ));
    c.insert_resource(ReflectionTarget(target));
}
fn sync(
    mut c: Commands,
    target: Res<ReflectionTarget>,
    quality: Res<crate::quality::RenderQuality>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<WetMaterial>>,
    sources: Query<
        (
            &Camera,
            &GlobalTransform,
            &Projection,
            Option<&bevy::light::EnvironmentMapLight>,
        ),
        (With<Camera3d>, Without<GroundReflectionCamera>),
    >,
    mut mirrors: Query<
        (
            Entity,
            &mut Camera,
            &mut Transform,
            &mut GlobalTransform,
            &mut Projection,
        ),
        With<GroundReflectionCamera>,
    >,
) {
    for (_, material) in materials.iter_mut() {
        material.extension.reflection.x = 0.;
    }
    let source = sources.iter().find(|(camera, _, _, _)| camera.is_active);
    for (entity, mut camera, mut transform, mut global, mut projection) in &mut mirrors {
        let Some((source, source_transform, Projection::Perspective(perspective), environment)) =
            source
        else {
            camera.is_active = false;
            continue;
        };
        camera.is_active = quality.detailed_wet && source_transform.translation().y > 0.03;
        for (_, material) in materials.iter_mut() {
            material.extension.reflection.x = camera.is_active as u32 as f32;
        }
        if !camera.is_active {
            continue;
        }
        let size = source
            .physical_viewport_size()
            .unwrap_or(UVec2::new(1280, 720));
        let desired = UVec2::new((size.x / 2).max(64), (size.y / 2).max(32));
        if let Some(mut image) = images.get_mut(&target.0) {
            if image.width() != desired.x || image.height() != desired.y {
                image.resize(bevy::render::render_resource::Extent3d {
                    width: desired.x,
                    height: desired.y,
                    depth_or_array_layers: 1,
                });
            }
        }
        let reflect = |p: Vec3| Vec3::new(p.x, -p.y, p.z);
        let position = reflect(source_transform.translation());
        let forward = reflect(*source_transform.forward());
        let up = reflect(*source_transform.up());
        *transform = Transform::from_translation(position).looking_to(forward, up);
        *global = GlobalTransform::from(*transform);
        let plane = global.to_matrix().transpose() * Vec4::new(0., 1., 0., -0.01);
        let mut perspective = perspective.clone();
        perspective.aspect_ratio = desired.x as f32 / desired.y as f32;
        let reflected = ReflectedProjection { perspective, plane };
        let matrix = reflected.get_clip_from_view() * global.to_matrix().inverse();
        *projection = Projection::custom(reflected);
        if let Some(environment) = environment {
            c.entity(entity).insert(environment.clone());
        }
        for (_, material) in materials.iter_mut() {
            material.extension.reflected = target.0.clone();
            material.extension.reflection_matrix = matrix;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oblique_reflection_clips_below_the_apron_with_reversed_depth() {
        let world = Transform::from_xyz(0., -2., 5.)
            .looking_at(Vec3::ZERO, Vec3::NEG_Y)
            .to_matrix();
        let projection = ReflectedProjection {
            perspective: PerspectiveProjection::default(),
            plane: world.transpose() * Vec4::new(0., 1., 0., -0.01),
        };
        let clip = projection.get_clip_from_view() * world.inverse();
        let above = clip * Vec4::new(0., 1., 0., 1.);
        let below = clip * Vec4::new(0., -1., 0., 1.);
        assert!(above.z <= above.w && above.z >= 0.);
        assert!(below.z > below.w);
    }
}
