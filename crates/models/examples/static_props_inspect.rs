//! Headless catalogue overview: rows follow `PropKind::ALL`, six props per row.
use bevy::{
    camera::RenderTarget,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use saltline_models::{
    Kit,
    library::map_props::{self, PropKind, PropOptions},
    static_props::{StaticPropsPlugin, StaticPropsReady},
};

#[derive(Resource)]
struct Capture {
    target: Handle<Image>,
    frame: u32,
    started: std::time::Instant,
}
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/../../assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(bevy::render::RenderPlugin {
                    synchronous_pipeline_compilation: true,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    ..default()
                })
                .disable::<bevy::winit::WinitPlugin>()
                .disable::<bevy::audio::AudioPlugin>(),
        )
        .add_plugins((
            StaticPropsPlugin,
            bevy::app::ScheduleRunnerPlugin::run_loop(std::time::Duration::from_millis(1)),
        ))
        .insert_resource(ClearColor(Color::srgb(0.18, 0.21, 0.24)))
        .insert_resource(bevy::light::GlobalAmbientLight {
            brightness: 350.,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, capture)
        .run();
}
fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let kit = Kit::new(&mut meshes, &mut mats, &mut images);
    for (i, kind) in PropKind::ALL.into_iter().enumerate() {
        let parent = kit.root(
            &mut c,
            Vec3::new((i % 6) as f32 * 4. - 10., 0., (i / 6) as f32 * 4. - 10.),
        );
        let root = map_props::build(&mut c, &kit, parent, kind, PropOptions::default());
        c.entity(root).insert(Transform::from_scale(Vec3::splat(
            2.8 / kind.size().max_element(),
        )));
        c.spawn((
            Mesh3d(meshes.add(Cuboid::new(3.7, 0.1, 3.7))),
            MeshMaterial3d(mats.add(Color::srgb(0.28, 0.31, 0.34))),
            Transform::from_xyz((i % 6) as f32 * 4. - 10., -0.1, (i / 6) as f32 * 4. - 10.),
        ));
    }
    c.spawn((
        DirectionalLight {
            illuminance: 12000.,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-8., 20., 10.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let target = images.add(Image::new_target_texture(
        1600,
        1200,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        None,
    ));
    c.spawn((
        Camera3d::default(),
        RenderTarget::Image(target.clone().into()),
        Transform::from_xyz(19., 28., 30.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    c.insert_resource(Capture {
        target,
        frame: 0,
        started: std::time::Instant::now(),
    });
}
fn capture(
    mut c: Commands,
    ready: Res<StaticPropsReady>,
    mut state: ResMut<Capture>,
    mut exit: MessageWriter<AppExit>,
) {
    assert!(
        state.started.elapsed().as_secs() < 90,
        "catalogue capture timed out"
    );
    if !ready.ready() {
        return;
    }
    state.frame += 1;
    if state.frame == 60 {
        c.spawn(Screenshot::image(state.target.clone()))
            .observe(save_to_disk("tmp/static-props/catalogue.png"));
    }
    if state.frame == 90 {
        exit.write(AppExit::Success);
    }
}
