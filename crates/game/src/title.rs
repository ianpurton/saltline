//! Title presentation shares the procedural port, but owns its camera and UI.
use crate::{Hud, Visuals};
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, ActivateOnPress, Button},
};
use saltline_game::{Enemy, Mission, Mode};
#[derive(Component)]
pub struct GameplayCamera;
#[derive(Component)]
pub struct Crosshair;
#[derive(Component)]
pub struct TitleDressing;
#[derive(Component)]
struct TitleCamera;
#[derive(Component)]
struct TitleRoot;
#[derive(Component)]
struct Panel;
#[derive(Component)]
struct PanelText;
#[derive(Component)]
struct MenuButton(bool);
#[derive(Resource, Default)]
struct TitleState {
    panel: u8,
    clock: f32,
    reduced_motion: bool,
}
pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TitleState>()
            .add_systems(PostStartup, setup)
            .add_systems(Update, update.after(crate::visuals))
            .add_systems(Update, capture);
    }
}
const INK: Color = Color::srgb(0.91, 0.90, 0.85);
const AMBER: Color = Color::srgb(1., 0.65, 0.23);
fn label(text: &str, size: f32, font: &Handle<Font>, color: Color) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone().into(),
            font_size: FontSize::Px(size),
            weight: if size > 100. {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            },
            ..default()
        },
        TextColor(color),
    )
}
fn setup(
    mut c: Commands,
    mut fonts: ResMut<Assets<Font>>,
    visuals: Res<Visuals>,
    mut images: ResMut<Assets<Image>>,
) {
    let sky = saltline_models::materials::sky::overcast(&mut images);
    // Keep menu text available in copied builds and from any working directory.
    let font = fonts.add(Font::from_bytes(
        include_bytes!("../../../assets/fonts/Oswald.ttf").to_vec(),
    ));
    let title_camera = c
        .spawn((
            TitleCamera,
            Camera3d::default(),
            bevy::light::Skybox {
                image: Some(sky.clone()),
                brightness: 1800.,
                ..default()
            },
            bevy::light::GeneratedEnvironmentMapLight {
                environment_map: sky,
                intensity: 1000.,
                ..default()
            },
            Camera {
                is_active: false,
                ..default()
            },
            bevy::camera::Hdr,
            bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
            Msaa::Off,
            bevy::pbr::ScreenSpaceReflections::default(),
            bevy::pbr::ContactShadows::default(),
            bevy::light::VolumetricFog::default(),
            bevy::anti_alias::taa::TemporalAntiAliasing::default(),
            bevy::post_process::bloom::Bloom::default(),
            DistanceFog {
                color: Color::srgb(0.42, 0.38, 0.30),
                falloff: FogFalloff::Linear {
                    start: 24.,
                    end: 150.,
                },
                ..default()
            },
            Transform::default(),
        ))
        .id();
    if !visuals.view.is_empty() {
        return;
    }
    c.spawn((UiTargetCamera(title_camera), TitleRoot, Node { width: percent(100), height: percent(100), ..default() })).with_children(|root| {
        // Graduated scrim preserves the live skyline and anchors the typography.
        root.spawn((Node { position_type: PositionType::Absolute, width: percent(100), height: percent(100), ..default() },
            BackgroundGradient::from(LinearGradient::to_top(vec![
                ColorStop::new(Color::srgba(0.01,0.018,0.022,0.88), percent(0)),
                ColorStop::new(Color::srgba(0.01,0.018,0.022,0.45), percent(35)),
                ColorStop::new(Color::NONE, percent(75)),
            ]))));
        root.spawn((Node { position_type: PositionType::Absolute, left: percent(5.5), bottom: percent(5.5), flex_direction: FlexDirection::Column, width: percent(69), ..default() },)).with_children(|left| {
            left.spawn(label("B4   /   P O R T  A U T H O R I T Y  B R E A C H  U N I T     /     SALTLINE TERMINAL · 05:42", 11., &font, INK));
            left.spawn((label("S A L T L I N E", 136., &font, INK), Node { margin: UiRect::vertical(px(-8.)), ..default() }));
            left.spawn((Node { width: px(56.), height: px(3.), margin: UiRect::vertical(px(10.)), ..default() }, BackgroundColor(AMBER)));
            left.spawn(label("The port’s security net has gone feral. Cut its three uplinks. Reach the boat.", 18., &font, INK));
            left.spawn(Node { flex_direction: FlexDirection::Row, column_gap: px(8.), margin: UiRect::top(px(22.)), ..default() }).with_children(|row| {
                for (index, name) in ["D E P L O Y", "S E T T I N G S", "C O N T R O L S"].iter().enumerate() {
                    row.spawn((Button, ActivateOnPress, Hovered::default(), MenuButton(index==0), Node { width: px(if index==0 {132.} else {104.}), height: px(43.), border: UiRect::all(px(1.)), align_items: AlignItems::Center, justify_content: JustifyContent::Center, ..default() }, BorderColor::all(INK.with_alpha(0.5)), BackgroundColor(if index==0 {AMBER} else {Color::srgba(0.03,0.04,0.04,0.7)})))
                    .observe(move |_: On<Activate>, mut state: ResMut<TitleState>, mut m: ResMut<Mission>, mut enemies: Query<(&mut Enemy, &mut Transform)>| {
                        if index==0 {
                            *m=Mission::default(); m.mode=Mode::Playing; state.panel=0;
                            for (mut enemy, mut transform) in &mut enemies {
                                enemy.health=100.; enemy.cooldown=2.;
                                transform.translation=enemy.home; transform.rotation=Quat::IDENTITY;
                            }
                        } else {state.panel=index as u8;}
                    }).with_children(|b| { b.spawn(label(name, 13., &font, if index==0 {Color::BLACK} else {INK})); });
                }
            });
            left.spawn((label("Every surface has a story. Every signal has a source.\nOPERATION BREAKWATER   /   ENTER TO DEPLOY", 10., &font, Color::srgb(0.54,0.62,0.64)), Node { margin: UiRect::top(px(16.)), ..default() }));
        });
        root.spawn((label("[W A S D] MOVE    [MOUSE] LOOK    [LMB] FIRE    [RMB] AIM\n[R] RELOAD    [E] CUT RELAY    [SPACE] JUMP\n[SHIFT] SPRINT    [C] CROUCH    [ESC] PAUSE", 11., &font, INK), Node { position_type: PositionType::Absolute, right: percent(4.5), bottom: percent(6.), ..default() }));
        root.spawn((Panel, Visibility::Hidden, Node { position_type: PositionType::Absolute, left: percent(27.), top: percent(16.), width: percent(46.), padding: UiRect::all(px(30.)), border: UiRect::all(px(1.)), ..default() }, BorderColor::all(AMBER), BackgroundColor(Color::srgba(0.018,0.027,0.032,0.97)))).with_children(|panel| {
            panel.spawn((PanelText, label("", 22., &font, INK)));
        });
    });
}
fn update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    m: Res<Mission>,
    v: Res<Visuals>,
    mut state: ResMut<TitleState>,
    mut cameras: Query<
        (&mut Camera, &mut Transform, Has<TitleCamera>),
        Or<(With<TitleCamera>, With<GameplayCamera>)>,
    >,
    mut visibility: Query<(
        &mut Visibility,
        Has<TitleRoot>,
        Has<Panel>,
        Has<Enemy>,
        Has<Crosshair>,
        Has<Hud>,
        Has<TitleDressing>,
    )>,
    mut panels: Query<&mut Text, With<PanelText>>,
    mut buttons: Query<(&Hovered, &MenuButton, &mut BackgroundColor)>,
) {
    let menu = m.mode == Mode::Menu && v.view.is_empty();
    if keys.just_pressed(KeyCode::Escape) || !menu {
        state.panel = 0;
    }
    if menu && state.panel == 1 && keys.just_pressed(KeyCode::KeyM) {
        state.reduced_motion = !state.reduced_motion;
    }
    if menu && !state.reduced_motion {
        state.clock += time.delta_secs();
    }
    for (mut camera, mut transform, title) in &mut cameras {
        camera.is_active = if title { menu } else { !menu };
        if title {
            // Closed periodic dolly: continuous position and velocity at the seam.
            let phase = state.clock * std::f32::consts::TAU / 100.;
            *transform = Transform::from_xyz(
                1.5 + phase.sin() * 1.8,
                2.6 + phase.cos() * 0.18,
                -3. + phase.cos() * 3.,
            )
            .looking_at(Vec3::new(10., 7., -65.), Vec3::Y);
        }
    }
    for (mut visible, root, panel, enemy, crosshair, hud, dressing) in &mut visibility {
        if root || panel || enemy || crosshair || hud || dressing {
            let show = if root || dressing {
                menu
            } else if panel {
                menu && state.panel != 0
            } else if enemy {
                !menu
            } else {
                !menu && (hud || (crosshair && m.mode == Mode::Playing && v.view.is_empty()))
            };
            *visible = if show {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
    for mut text in &mut panels {
        text.0 = if state.panel == 1 {
            format!(
                "S E T T I N G S\n\nCinematic camera motion: {}\n[M] Toggle camera motion\n\nMouse sensitivity: 0.002\n\n[ESC] BACK",
                if state.reduced_motion { "OFF" } else { "ON" }
            )
        } else {
            "C O N T R O L S\n\nWASD   Move       MOUSE   Look\nLMB   Fire       RMB   Aim\nR   Reload       E   Cut relay\nSPACE   Jump       SHIFT   Sprint\nC   Crouch       ESC   Pause\n\n[ESC] BACK".into()
        };
    }
    for (hover, button, mut bg) in &mut buttons {
        bg.0 = if button.0 {
            if hover.get() {
                Color::srgb(1., 0.77, 0.4)
            } else {
                AMBER
            }
        } else if hover.get() {
            Color::srgb(0.25, 0.22, 0.16)
        } else {
            Color::srgba(0.03, 0.04, 0.04, 0.7)
        };
    }
}

// Bounded capture of the actual startup flow, including UI, for visual review.
fn capture(mut c: Commands, mut frame: Local<u32>) {
    if !std::env::args().any(|a| a == "--title-capture") {
        return;
    }
    *frame += 1;
    if *frame == 70 || *frame == 170 {
        c.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
            .observe(bevy::render::view::screenshot::save_to_disk(format!(
                "tmp/screenshots/title-{}.png",
                *frame
            )));
    }
    if *frame == 200 {
        c.write_message(AppExit::Success);
    }
}
