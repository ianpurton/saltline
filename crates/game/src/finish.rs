//! Reference-inspired display-linear finishing, after tone mapping and before UI.
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

const SHADER_ASSET_PATH: &str = "embedded://saltline_game/finish.wesl";

pub struct FinishPlugin;

impl Plugin for FinishPlugin {
    fn build(&self, app: &mut App) {
        bevy::asset::embedded_asset!(app, "finish.wesl");
        app.add_systems(PostStartup, attach.after(crate::title::setup));
        app.add_systems(Update, update.after(crate::presentation::clock));
        app.add_plugins((
            ExtractComponentPlugin::<FinishSettings>::default(),
            UniformComponentPlugin::<FinishSettings>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_systems(RenderStartup, init_finish_pipeline)
            .add_systems(
                Render,
                prepare_bind_groups.in_set(RenderSystems::PrepareBindGroups),
            )
            .add_systems(
                Core3d,
                finish_system
                    .in_set(Core3dSystems::PostProcess)
                    .after(bevy::core_pipeline::tonemapping::tonemapping),
            );
    }
}

#[derive(Component)]
struct FinishBindGroups {
    a: (TextureViewId, BindGroup),
    b: (TextureViewId, BindGroup),
}

fn prepare_bind_groups(
    mut commands: Commands,
    mut views: Query<(Entity, &ViewTarget, Option<&mut FinishBindGroups>)>,
    finish_pipeline: Option<Res<FinishPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    settings_uniforms: Res<ComponentUniforms<FinishSettings>>,
    render_device: Res<RenderDevice>,
) {
    let Some(finish_pipeline) = finish_pipeline else {
        return;
    };
    let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
        return;
    };

    let create_bind_group = |texture: &TextureView| {
        (
            texture.id(),
            render_device.create_bind_group(
                "finish_bind_group",
                &pipeline_cache.get_bind_group_layout(&finish_pipeline.layout),
                &BindGroupEntries::sequential((
                    texture,
                    &finish_pipeline.sampler,
                    settings_binding.clone(),
                )),
            ),
        )
    };

    for (entity, view_target, mut maybe_bind_groups) in &mut views {
        let main_texture_view = view_target.main_texture_view();
        let main_texture_other_view = view_target.main_texture_other_view();

        if let Some(bind_groups) = &mut maybe_bind_groups {
            if bind_groups.a.0 != main_texture_view.id() {
                bind_groups.a = create_bind_group(main_texture_view);
            }
            if bind_groups.b.0 != main_texture_other_view.id() {
                bind_groups.b = create_bind_group(main_texture_other_view);
            }
        } else {
            commands.entity(entity).insert(FinishBindGroups {
                a: create_bind_group(main_texture_view),
                b: create_bind_group(main_texture_other_view),
            });
        }
    }
}

fn finish_system(
    view: ViewQuery<(
        &ViewTarget,
        &DynamicUniformIndex<FinishSettings>,
        &FinishBindGroups,
    )>,
    finish_pipeline: Option<Res<FinishPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let Some(finish_pipeline) = finish_pipeline else {
        return;
    };

    let (view_target, settings_index, bind_groups) = view.into_inner();

    let Some(pipeline) = pipeline_cache.get_render_pipeline(finish_pipeline.pipeline_id) else {
        return;
    };

    let finish = view_target.post_process_write();

    let (_, bind_group) = if bind_groups.a.0 == finish.source.id() {
        &bind_groups.a
    } else {
        &bind_groups.b
    };

    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let span = diagnostics.time_span(ctx.command_encoder(), "saltline_finish");
    let mut render_pass = ctx
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("finish_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: finish.destination,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

    render_pass.set_pipeline(pipeline);
    render_pass.set_bind_group(0, bind_group, &[settings_index.index()]);
    render_pass.draw(0..3, 0..1);
    drop(render_pass);
    span.end(ctx.command_encoder());
}

#[derive(Resource)]
struct FinishPipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

fn init_finish_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    pipeline_cache: Res<PipelineCache>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "finish_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                uniform_buffer::<FinishSettings>(true),
            ),
        ),
    );
    let sampler = render_device.create_sampler(&SamplerDescriptor::default());

    let shader = asset_server.load(SHADER_ASSET_PATH);
    let vertex_state = fullscreen_shader.to_vertex_state();
    let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("finish_pipeline".into()),
        layout: vec![layout.clone()],
        vertex: vertex_state,
        fragment: Some(FragmentState {
            shader,
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::Rgba16Float,
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            ..default()
        }),
        ..default()
    });
    commands.insert_resource(FinishPipeline {
        layout,
        sampler,
        pipeline_id,
    });
}

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
#[extract_app(RenderApp)]
struct FinishSettings {
    params: Vec4, // time, damage, low health, ADS
}
fn attach(mut c: Commands, cameras: Query<Entity, With<Camera3d>>) {
    for e in &cameras {
        c.entity(e).insert(FinishSettings::default());
    }
}
fn update(
    clock: Res<crate::presentation::PresentationClock>,
    mission: Res<saltline_game::Mission>,
    actions: Res<saltline_game::Actions>,
    mut settings: Query<&mut FinishSettings>,
) {
    let damage = crate::capture::option("--inspect").as_deref() == Some("damage");
    for mut s in &mut settings {
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
