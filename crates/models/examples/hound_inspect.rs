//! Uses the same Bevy screenshot observer as the game, with a neutral isolated stage.
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use saltline_models::Kit;
#[derive(Resource)]
struct Capture {
    frame: u32,
    view: String,
}
fn main() {
    let view = std::env::args().nth(1).unwrap_or("three-quarter".into());
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280, 720).into(),
                title: "Hound inspection".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.18, 0.21, 0.24)))
        .insert_resource(bevy::light::GlobalAmbientLight {
            brightness: 350.,
            ..default()
        })
        .insert_resource(Capture { frame: 0, view })
        .add_systems(Startup, setup)
        .add_systems(Update, capture)
        .run();
}
fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    capture: Res<Capture>,
) {
    let k = Kit::new(&mut meshes, &mut mats, &mut images);
    let root = k.root(&mut c, Vec3::ZERO);
    k.hound(&mut c, root);
    c.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200., 200.))),
        MeshMaterial3d(mats.add(Color::srgb(0.19, 0.22, 0.24))),
    ));
    c.spawn((
        DirectionalLight {
            illuminance: 10000.,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(3., 6., -4.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let eye = match capture.view.as_str() {
        "side" => Vec3::new(3.8, 1.55, 0.),
        "front" => Vec3::new(0., 1.55, -3.8),
        "rear" => Vec3::new(0., 1.55, 3.8),
        _ => Vec3::new(2.8, 2.1, -3.1),
    };
    c.spawn((
        Camera3d::default(),
        Transform::from_translation(eye).looking_at(Vec3::new(0., 0.95, -0.05), Vec3::Y),
    ));
}
fn capture(mut c: Commands, mut state: ResMut<Capture>, mut exit: MessageWriter<AppExit>) {
    state.frame += 1;
    if state.frame == 40 {
        c.spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "tmp/screenshots/hound-{}.png",
                state.view
            )));
    }
    if state.frame == 65 {
        exit.write(AppExit::Success);
    }
}
