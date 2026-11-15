//! Load an exported GLB in Bevy and save a smoke-test capture.
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

#[derive(Resource)]
struct Inspect {
    scene: Handle<WorldAsset>,
    frames: u32,
    started: std::time::Instant,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: format!("{}/../../assets", env!("CARGO_MANIFEST_DIR")),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, capture)
        .run();
}
fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let id = std::env::args().nth(1).unwrap_or_else(|| "hound".into());
    let scene = server.load(format!("models/{id}.glb#Scene0"));
    commands.spawn(WorldAssetRoot(scene.clone()));
    commands.insert_resource(Inspect {
        scene,
        frames: 0,
        started: std::time::Instant::now(),
    });
    commands.spawn((
        DirectionalLight {
            illuminance: 12000.,
            ..default()
        },
        Transform::from_xyz(4., 7., 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let (eye, target) = match id.as_str() {
        "container20" => (Vec3::new(9., 6., 10.), Vec3::new(0., 1.3, 0.)),
        "pac9" => (Vec3::new(1.2, 0.7, 1.2), Vec3::ZERO),
        _ => (Vec3::new(3., 2.4, 3.), Vec3::new(0., 0.7, 0.)),
    };
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(eye).looking_at(target, Vec3::Y),
    ));
}
fn capture(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut inspect: ResMut<Inspect>,
    mut exit: MessageWriter<AppExit>,
) {
    if let bevy::asset::LoadState::Failed(error) = server.load_state(&inspect.scene) {
        panic!("GLB load failed: {error}");
    }
    assert!(
        inspect.started.elapsed().as_secs() < 90,
        "GLB smoke test timed out"
    );
    if !server.is_loaded_with_dependencies(&inspect.scene) {
        return;
    }
    inspect.frames += 1;
    if inspect.frames == 60 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk("target/asset-export/bevy.png"));
    }
    if inspect.frames == 90 {
        exit.write(AppExit::Success);
    }
}
