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
        .unwrap_or((1280, 720))
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
pub fn attach(
    mut c: Commands,
    mut images: ResMut<Assets<Image>>,
    cameras: Query<Entity, With<Camera3d>>,
) {
    let mut target = CaptureTarget::default();
    if headless() {
        let (w, h) = resolution();
        let image = images.add(Image::new_target_texture(
            w,
            h,
            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            None,
        ));
        for e in &cameras {
            c.entity(e)
                .insert(RenderTarget::Image(image.clone().into()));
        }
        target.0 = Some(image);
    }
    c.insert_resource(target);
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
