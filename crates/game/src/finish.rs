//! Reference-inspired cinematic pipeline. Fragment passes also work on WebGL2.
use bevy::render::diagnostic::RecordDiagnostics;
use bevy::{
    core_pipeline::{Core3dSystems, FullscreenShader, schedule::Core3d},
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup, RenderSystems,
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice, ViewQuery},
        view::ViewTarget,
    },
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
pub struct FinishPlugin;
impl Plugin for FinishPlugin {
    fn build(&self, app: &mut App) {
        bevy::asset::embedded_asset!(app, "finish.wesl");
        app.add_systems(PostStartup, attach.after(crate::title::setup))
            .add_systems(
                PostUpdate,
                update
                    .after(bevy::transform::TransformSystems::Propagate)
                    .after(bevy::camera::CameraUpdateSystems),
            )
            .add_plugins((
                ExtractComponentPlugin::<FinishSettings>::default(),
                UniformComponentPlugin::<FinishSettings>::default(),
            ));
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_systems(RenderStartup, init)
            .add_systems(Render, prepare.in_set(RenderSystems::PrepareBindGroups))
            .add_systems(
                Core3d,
                render
                    .in_set(Core3dSystems::PostProcess)
                    .after(bevy::core_pipeline::tonemapping::tonemapping),
            );
    }
}
#[derive(Resource)]
struct Pipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    shader: Handle<Shader>,
    vertex: VertexState,
}
#[derive(Component)]
struct Targets {
    size: UVec2,
    format: TextureFormat,
    divisor: u32,
    blur: [TextureView; 2],
    streak: TextureView,
    dirt: TextureView,
    history: [TextureView; 2],
    frame: AtomicUsize,
    dirt_ready: AtomicBool,
    pipelines: [CachedRenderPipelineId; 6],
}
fn init(
    mut c: Commands,
    device: Res<RenderDevice>,
    assets: Res<AssetServer>,
    fullscreen: Res<FullscreenShader>,
) {
    let tex = || texture_2d(TextureSampleType::Float { filterable: true });
    #[cfg(target_arch = "wasm32")]
    let entries = BindGroupLayoutEntries::sequential(
        ShaderStages::FRAGMENT,
        (
            tex(),
            sampler(SamplerBindingType::Filtering),
            uniform_buffer::<FinishSettings>(true),
            tex(),
            tex(),
            tex(),
            tex(),
        ),
    );
    #[cfg(not(target_arch = "wasm32"))]
    let entries = BindGroupLayoutEntries::sequential(
        ShaderStages::FRAGMENT,
        (
            tex(),
            sampler(SamplerBindingType::Filtering),
            uniform_buffer::<FinishSettings>(true),
            tex(),
            tex(),
            tex(),
            tex(),
            bevy::render::render_resource::binding_types::texture_depth_2d(),
        ),
    );
    let layout = BindGroupLayoutDescriptor::new("cinematic", &entries);
    c.insert_resource(Pipeline {
        layout,
        sampler: device.create_sampler(&SamplerDescriptor {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..default()
        }),
        shader: assets.load("embedded://saltline_game/finish.wesl"),
        vertex: fullscreen.to_vertex_state(),
    });
}
fn prepare(
    mut c: Commands,
    device: Res<RenderDevice>,
    pipeline: Option<Res<Pipeline>>,
    cache: Res<PipelineCache>,
    views: Query<(Entity, &ViewTarget, &FinishSettings, Option<&Targets>)>,
) {
    let Some(p) = pipeline else {
        return;
    };
    for (e, view, settings, old) in &views {
        let size = UVec2::new(view.main_texture().width(), view.main_texture().height());
        let format = view.main_texture_format();
        let divisor = settings.options.w as u32;
        if old.is_some_and(|o| o.size == size && o.format == format && o.divisor == divisor) {
            continue;
        }
        let texture = |size: UVec2, format| {
            device
                .create_texture(&TextureDescriptor {
                    label: Some("cinematic_target"),
                    size: Extent3d {
                        width: size.x.max(1),
                        height: size.y.max(1),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format,
                    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
                .create_view(&TextureViewDescriptor::default())
        };
        let small = size / divisor.max(1);
        let aux_format = if cfg!(target_arch = "wasm32") {
            TextureFormat::Rgba8Unorm
        } else {
            TextureFormat::Rgba16Float
        };
        let pipelines = ["blur", "meter", "dirt", "streak", "composite", "finish"].map(|entry| {
            cache.queue_render_pipeline(RenderPipelineDescriptor {
                label: Some(format!("cinematic_{entry}").into()),
                layout: vec![p.layout.clone()],
                vertex: p.vertex.clone(),
                fragment: Some(FragmentState {
                    shader: p.shader.clone(),
                    shader_defs: if cfg!(target_arch = "wasm32") {
                        vec![]
                    } else {
                        vec!["NATIVE_DEPTH".into()]
                    },
                    entry_point: Some(entry.into()),
                    targets: vec![Some(ColorTargetState {
                        format: if entry == "composite" || entry == "finish" {
                            format
                        } else {
                            aux_format
                        },
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                    ..default()
                }),
                ..default()
            })
        });
        c.entity(e).insert(Targets {
            size,
            format,
            divisor,
            blur: [texture(small, aux_format), texture(small, aux_format)],
            streak: texture(small, aux_format),
            dirt: texture(UVec2::splat(128), aux_format),
            history: [
                texture(UVec2::ONE, aux_format),
                texture(UVec2::ONE, aux_format),
            ],
            frame: AtomicUsize::new(0),
            dirt_ready: AtomicBool::new(false),
            pipelines,
        });
    }
}
fn render(
    view: ViewQuery<(
        &ViewTarget,
        &DynamicUniformIndex<FinishSettings>,
        &FinishSettings,
        &Targets,
        Option<&bevy::core_pipeline::prepass::ViewPrepassTextures>,
    )>,
    pipeline: Option<Res<Pipeline>>,
    cache: Res<PipelineCache>,
    uniforms: Res<ComponentUniforms<FinishSettings>>,
    device: Res<RenderDevice>,
    mut ctx: RenderContext,
) {
    let Some(p) = pipeline else {
        return;
    };
    let Some(uniform) = uniforms.uniforms().binding() else {
        return;
    };
    let (view, index, settings, t, _prepass) = view.into_inner();
    #[cfg(not(target_arch = "wasm32"))]
    let Some(depth) = _prepass.and_then(|p| p.depth_only_view()) else {
        return;
    };
    let pipelines = t.pipelines.map(|id| cache.get_render_pipeline(id));
    if pipelines.iter().any(Option::is_none) {
        return;
    }
    let frame = t.frame.fetch_add(1, Ordering::Relaxed);
    let current = frame % 2;
    let previous = 1 - current;
    let draw = |ctx: &mut RenderContext,
                pass: usize,
                source: &TextureView,
                target: &TextureView,
                blurred: &TextureView,
                history: &TextureView,
                dirt: &TextureView,
                streak: &TextureView| {
        let Some(pipeline) = pipelines[pass] else {
            return;
        };
        #[cfg(target_arch = "wasm32")]
        let entries = BindGroupEntries::sequential((
            source,
            &p.sampler,
            uniform.clone(),
            blurred,
            history,
            dirt,
            streak,
        ));
        #[cfg(not(target_arch = "wasm32"))]
        let entries = BindGroupEntries::sequential((
            source,
            &p.sampler,
            uniform.clone(),
            blurred,
            history,
            dirt,
            streak,
            depth,
        ));
        let bind = device.create_bind_group(
            "cinematic_bind",
            &cache.get_bind_group_layout(&p.layout),
            &entries,
        );
        let diagnostics = ctx.diagnostic_recorder();
        let diagnostics = diagnostics.as_deref();
        let span = diagnostics.time_span(
            ctx.command_encoder(),
            [
                "cinematic_blur",
                "cinematic_meter",
                "cinematic_dirt",
                "cinematic_streak",
                "cinematic_composite",
                "cinematic_finish",
            ][pass],
        );
        let mut render = ctx
            .command_encoder()
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("cinematic"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations::default(),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        render.set_pipeline(pipeline);
        render.set_bind_group(0, &bind, &[index.index()]);
        render.draw(0..3, 0..1);
        drop(render);
        span.end(ctx.command_encoder());
    };
    let source = view.main_texture_view();
    // Unused bindings deliberately reference source, never the attachment being written.
    if settings.lens.y + settings.lens.w > 0. && !t.dirt_ready.swap(true, Ordering::Relaxed) {
        draw(&mut ctx, 2, source, &t.dirt, source, source, source, source);
    }
    if settings.options.x > 0. || settings.lens.w > 0. || frame == 0 || settings.timing.y > 0. {
        draw(
            &mut ctx,
            1,
            source,
            &t.history[current],
            source,
            &t.history[previous],
            source,
            source,
        );
    }
    let auxiliary = settings.lens.x
        + settings.lens.y
        + settings.lens.z
        + settings.params.w * settings.options.y
        > 0.;
    if auxiliary {
        draw(
            &mut ctx, 0, source, &t.blur[0], source, source, source, source,
        );
        draw(
            &mut ctx, 0, &t.blur[0], &t.blur[1], source, source, source, source,
        );
    }
    if settings.lens.z > 0. {
        draw(
            &mut ctx, 3, &t.blur[1], &t.streak, source, source, source, source,
        );
    }
    let composite = view.post_process_write();
    draw(
        &mut ctx,
        4,
        composite.source,
        composite.destination,
        if auxiliary {
            &t.blur[1]
        } else {
            composite.source
        },
        &t.history[current],
        &t.dirt,
        if settings.lens.z > 0. {
            &t.streak
        } else {
            composite.source
        },
    );
    let finish = view.post_process_write();
    draw(
        &mut ctx,
        5,
        finish.source,
        finish.destination,
        finish.source,
        finish.source,
        finish.source,
        finish.source,
    );
}

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
#[extract_app(RenderApp)]
struct FinishSettings {
    params: Vec4,  // time, damage, low health, ADS
    lens: Vec4,    // bloom, dirt, streak, flare
    options: Vec4, // adaptation, ADS blur, aberration, auxiliary divisor
    detail: Vec4,  // grain, sharpen, FXAA, native HDR
    sun: Vec4,     // UV, smoothed visibility, aspect
    timing: Vec4,  // dt, reset history, unused
}
fn attach(
    mut c: Commands,
    cameras: Query<
        Entity,
        (
            With<Camera3d>,
            Without<saltline_models::materials::wet::GroundReflectionCamera>,
        ),
    >,
) {
    for e in &cameras {
        c.entity(e)
            .insert((FinishSettings::default(), History::default()));
    }
}
#[derive(Component, Default)]
struct History {
    active: bool,
    adaptation: bool,
    visibility: f32,
}
fn update(
    clock: Res<crate::presentation::PresentationClock>,
    mission: Res<saltline_game::Mission>,
    actions: Res<saltline_game::Actions>,
    quality: Res<crate::quality::RenderQuality>,
    port: Res<saltline_world::Port>,
    mut settings: Query<(&mut FinishSettings, &Camera, &GlobalTransform, &mut History)>,
) {
    let damage = crate::capture::option("--inspect").as_deref() == Some("damage");
    for (mut s, camera, transform, mut history) in &mut settings {
        let reset = clock.reset
            || (camera.is_active && !history.active)
            || (quality.finish.adaptation && !history.adaptation);
        history.adaptation = quality.finish.adaptation;
        history.active = camera.is_active;
        let dir = saltline_models::materials::sky::SUN_DIRECTION.normalize();
        let position = transform.translation();
        let ndc = camera.world_to_ndc(transform, position + dir * 1000.);
        let in_front = transform.forward().dot(dir) > 0.;
        let uv = ndc
            .map(|p| Vec2::new(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5))
            .unwrap_or(Vec2::splat(-10.));
        let visible = in_front
            && uv.cmpge(Vec2::ZERO).all()
            && uv.cmple(Vec2::ONE).all()
            && (!cfg!(target_arch = "wasm32") || port.visible(position, position + dir * 1000.));
        let target = if visible { 1. } else { 0. };
        history.visibility = if reset {
            target
        } else {
            history.visibility + (target - history.visibility) * (1. - (-clock.dt * 8.).exp())
        };
        let size = camera.physical_viewport_size().unwrap_or(UVec2::ONE);
        s.sun = Vec4::new(
            uv.x,
            uv.y,
            if in_front { history.visibility } else { 0. },
            size.x as f32 / size.y.max(1) as f32,
        );
        s.timing = Vec4::new(clock.dt, reset as u32 as f32, 0., 0.);
        s.lens = Vec4::new(
            if quality.bloom { 0.065 } else { 0. },
            if quality.finish.lens { 0.1 } else { 0. },
            if quality.finish.lens { 0.03 } else { 0. },
            if quality.finish.lens { 0.16 } else { 0. },
        );
        s.options = Vec4::new(
            quality.finish.adaptation as u32 as f32,
            quality.finish.ads_blur as u32 as f32,
            if quality.finish.aberration {
                0.0013
            } else {
                0.
            },
            if quality.high { 2. } else { 4. },
        );
        s.detail = Vec4::new(
            if quality.finish.grain { 0.02 } else { 0. },
            if quality.finish.sharpen { 0.3 } else { 0. },
            if cfg!(target_arch = "wasm32") { 1. } else { 0. },
            if cfg!(target_arch = "wasm32") { 0. } else { 1. },
        );
        s.params = Vec4::new(
            clock.elapsed,
            if damage { 0.7 } else { mission.hurt * 3. },
            if damage {
                0.75
            } else {
                (1. - mission.health / 35.).clamp(0., 1.)
            },
            actions.aim as u32 as f32,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn camera_activation_and_adaptation_reset_independent_histories() {
        let mut app = App::new();
        app.init_resource::<crate::presentation::PresentationClock>()
            .init_resource::<saltline_game::Mission>()
            .init_resource::<saltline_game::Actions>()
            .init_resource::<saltline_world::Port>()
            .insert_resource(crate::quality::RenderQuality::preset("medium"))
            .add_systems(Update, update);
        let a = app
            .world_mut()
            .spawn((
                Camera::default(),
                GlobalTransform::default(),
                FinishSettings::default(),
                History::default(),
            ))
            .id();
        let b = app
            .world_mut()
            .spawn((
                Camera {
                    is_active: false,
                    ..default()
                },
                GlobalTransform::default(),
                FinishSettings::default(),
                History::default(),
            ))
            .id();
        app.update();
        assert_eq!(app.world().get::<FinishSettings>(a).unwrap().timing.y, 1.);
        app.update();
        assert_eq!(app.world().get::<FinishSettings>(a).unwrap().timing.y, 0.);
        app.world_mut().get_mut::<Camera>(b).unwrap().is_active = true;
        app.update();
        assert_eq!(app.world().get::<FinishSettings>(b).unwrap().timing.y, 1.);
        assert_eq!(app.world().get::<FinishSettings>(a).unwrap().timing.y, 0.);
        app.world_mut()
            .resource_mut::<crate::quality::RenderQuality>()
            .finish
            .adaptation = false;
        app.update();
        assert_eq!(app.world().get::<FinishSettings>(a).unwrap().options.x, 0.);
        app.world_mut()
            .resource_mut::<crate::quality::RenderQuality>()
            .finish
            .adaptation = true;
        app.update();
        assert_eq!(app.world().get::<FinishSettings>(a).unwrap().timing.y, 1.);
        *app.world_mut()
            .resource_mut::<crate::quality::RenderQuality>() =
            crate::quality::RenderQuality::preset("low");
        app.update();
        let settings = app.world().get::<FinishSettings>(a).unwrap();
        assert_eq!(settings.lens.yzw(), Vec3::ZERO);
        assert_eq!(settings.options.w, 4.);
    }
}
