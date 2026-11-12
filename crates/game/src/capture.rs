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
pub fn attach(
    mut c: Commands,
    mut images: ResMut<Assets<Image>>,
    cameras: Query<Entity, With<Camera3d>>,
    quality: Res<crate::quality::RenderQuality>,
) {
    use bevy::camera::visibility::RenderLayers;
    let (w, h) = resolution();
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
) {
    if ready.ready() {
        quality.adapt(time.delta_secs());
    }
    let (w, h) = windows
        .iter()
        .next()
        .map(|w| (w.physical_width(), w.physical_height()))
        .unwrap_or_else(resolution);
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
