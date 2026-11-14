//! Native offscreen rendering and reproducible inspection settings.
use bevy::{camera::RenderTarget, prelude::*, render::view::screenshot::Screenshot};

#[derive(Resource, Default)]
pub struct CaptureTarget(pub Option<Handle<Image>>);
pub fn option(name: &str) -> Option<String> {
    saltline_game::launch_args()
        .skip_while(|a| a != name)
        .nth(1)
}
pub fn headless() -> bool {
    saltline_game::launch_args().any(|a| a == "--headless-render")
}
pub fn resolution() -> (u32, u32) {
    option("--resolution")
        .and_then(|v| {
            let (w, h) = v.split_once('x')?;
            Some((w.parse().ok()?, h.parse().ok()?))
        })
        .filter(|(w, h)| *w > 0 && *h > 0)
        .unwrap_or((1920, 1080))
}
pub fn output(name: &str) -> String {
    format!(
        "{}/{}.png",
        option("--output-dir").unwrap_or_else(|| "tmp/screenshots".into()),
        name
    )
}
pub fn screenshot(target: &CaptureTarget) -> Screenshot {
    target
        .0
        .as_ref()
        .map(|h| Screenshot::image(h.clone()))
        .unwrap_or_else(Screenshot::primary_window)
}
#[derive(Resource)]
pub struct OutputCamera(pub Entity);
#[derive(Component)]
pub struct Composite;
#[derive(Resource)]
pub struct SceneTarget(Handle<Image>);

/// Preserve aspect ratio when a browser window exceeds the negotiated GPU limit.
#[cfg(any(target_arch = "wasm32", test))]
fn bounded_resolution(width: u32, height: u32, limit: u32) -> (u32, u32) {
    let limit = limit.max(1);
    let largest = width.max(height).max(1);
    let scale = (limit as f64 / largest as f64).min(1.);
    (
        ((width as f64 * scale).floor() as u32).max(1),
        ((height as f64 * scale).floor() as u32).max(1),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn limit_browser_surface(
    device: Res<bevy::render::renderer::RenderDevice>,
    mut windows: Query<&mut Window>,
) {
    for mut window in &mut windows {
        let size = window.resolution.physical_size();
        let (w, h) = bounded_resolution(size.x, size.y, device.limits().max_texture_dimension_2d);
        if (w, h) != (size.x, size.y) {
            window.resolution.set_physical_resolution(w, h);
        }
    }
}

pub fn attach(
    mut c: Commands,
    mut images: ResMut<Assets<Image>>,
    cameras: Query<Entity, With<Camera3d>>,
    quality: Res<crate::quality::RenderQuality>,
    #[cfg(target_arch = "wasm32")] device: Res<bevy::render::renderer::RenderDevice>,
) {
    use bevy::camera::visibility::RenderLayers;
    let (w, h) = resolution();
    #[cfg(target_arch = "wasm32")]
    let (w, h) = bounded_resolution(w, h, device.limits().max_texture_dimension_2d);
    let scene = images.add(Image::new_target_texture(
        (w as f32 * quality.scale) as u32,
        (h as f32 * quality.scale) as u32,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        None,
    ));
    for e in &cameras {
        c.entity(e)
            .insert(RenderTarget::Image(scene.clone().into()));
    }
    let mut output = c.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        RenderLayers::layer(31),
    ));
    let mut target = CaptureTarget::default();
    if headless() {
        let image = images.add(Image::new_target_texture(
            w,
            h,
            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            None,
        ));
        output.insert(RenderTarget::Image(image.clone().into()));
        target.0 = Some(image);
    }
    let output = output.id();
    c.spawn((
        Sprite {
            image: scene.clone(),
            custom_size: Some(Vec2::new(w as f32, h as f32)),
            ..default()
        },
        Transform::default(),
        Composite,
        RenderLayers::layer(31),
    ));
    c.insert_resource(OutputCamera(output));
    c.insert_resource(SceneTarget(scene));
    c.insert_resource(target);
}
pub fn resize(
    time: Res<Time<Real>>,
    ready: Res<RenderReady>,
    mut quality: ResMut<crate::quality::RenderQuality>,
    windows: Query<&Window>,
    scene: Res<SceneTarget>,
    mut images: ResMut<Assets<Image>>,
    mut sprites: Query<&mut Sprite, With<Composite>>,
    #[cfg(target_arch = "wasm32")] device: Res<bevy::render::renderer::RenderDevice>,
) {
    if ready.ready() {
        quality.adapt(time.delta_secs());
    }
    let (w, h) = windows
        .iter()
        .next()
        .map(|w| (w.physical_width(), w.physical_height()))
        .unwrap_or_else(resolution);
    #[cfg(target_arch = "wasm32")]
    let (w, h) = bounded_resolution(w, h, device.limits().max_texture_dimension_2d);
    let size = bevy::render::render_resource::Extent3d {
        width: ((w as f32 * quality.scale) as u32).max(1),
        height: ((h as f32 * quality.scale) as u32).max(1),
        depth_or_array_layers: 1,
    };
    if let Some(mut image) = images.get_mut(&scene.0) {
        if image.texture_descriptor.size != size {
            image.resize(size);
        }
    }
    let display_size = windows
        .iter()
        .next()
        .map(|window| Vec2::new(window.width(), window.height()))
        .unwrap_or(Vec2::new(w as f32, h as f32));
    for mut sprite in &mut sprites {
        sprite.custom_size = Some(display_size);
    }
}

/// Capture clocks start only after GPU pipelines have settled.
#[derive(Resource, Clone, Default)]
pub struct RenderReady(pub std::sync::Arc<std::sync::atomic::AtomicBool>);
impl RenderReady {
    pub fn ready(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
}
pub fn readiness(app: &mut App) {
    let ready = RenderReady::default();
    app.insert_resource(ready.clone());
    if let Some(render) = app.get_sub_app_mut(bevy::render::RenderApp) {
        render
            .insert_resource(ready)
            .add_systems(bevy::render::Render, settled);
    }
}
fn settled(
    cache: Res<bevy::render::render_resource::PipelineCache>,
    ready: Res<RenderReady>,
    mut quiet: Local<u32>,
) {
    if ready.ready() {
        return;
    }
    if cache.waiting_pipelines().next().is_none() {
        *quiet += 1;
    } else {
        *quiet = 0;
    }
    if *quiet >= 60 {
        ready.0.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
mod resolution_tests {
    use super::bounded_resolution;
    #[test]
    fn high_dpi_and_portrait_surfaces_fit_device_limits() {
        assert_eq!(bounded_resolution(3840, 2160, 2048), (2048, 1152));
        assert_eq!(bounded_resolution(2160, 3840, 2048), (1152, 2048));
        assert_eq!(bounded_resolution(3840, 2160, 8192), (3840, 2160));
        assert_eq!(bounded_resolution(7680, 4320, 4096), (4096, 2304));
        assert_eq!(bounded_resolution(0, 0, 2048), (1, 1));
    }
}
