mod driver;
mod effects;
mod rendering;
mod title;
use bevy::pbr::DefaultOpaqueRendererMethod;
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
    window::{CursorGrabMode, CursorOptions},
};
use saltline_game::*;
use saltline_models::{Kit, Knee, Leg};
use saltline_world::{EXIT, RELAYS};
#[derive(Resource)]
struct Visuals {
    rifle: Entity,
    relays: Vec<Entity>,
    capture: u32,
    view: String,
}
#[derive(Component)]
struct Hud;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SALTLINE / port security incident".into(),
                canvas: cfg!(target_arch = "wasm32").then(|| "#game".into()),
                fit_canvas_to_parent: cfg!(target_arch = "wasm32"),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.16, 0.22, 0.28)))
        .insert_resource(bevy::light::GlobalAmbientLight {
            color: Color::srgb(0.48, 0.65, 0.85),
            brightness: 2000.,
            ..default()
        })
        .insert_resource(if cfg!(target_arch = "wasm32") {
            DefaultOpaqueRendererMethod::forward()
        } else {
            DefaultOpaqueRendererMethod::deferred()
        })
        .init_resource::<Mission>()
        .init_resource::<Actions>()
        .add_plugins(title::TitlePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (controls, driver::drive, simulate, visuals).chain())
        .add_systems(Update, (surface_ripples, effects::combat_effects))
        .run();
}
fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let sky = saltline_models::materials::sky::overcast(&mut images);
    let k = Kit::new(&mut meshes, &mut mats, &mut images);
    c.insert_resource(effects::DebrisAssets {
        mesh: k.hard.armor.clone(),
        material: k.mats[7].clone(),
    });
    #[cfg(not(target_arch = "wasm32"))]
    for (y, height, density) in [(0.7, 1.4, 0.008), (4., 4., 0.002)] {
        c.spawn((
            bevy::light::FogVolume {
                density_factor: density,
                fog_color: Color::srgb(0.57, 0.65, 0.7),
                ..default()
            },
            Transform::from_xyz(0., y, -48.).with_scale(Vec3::new(180., height, 220.)),
        ));
    }
    let port = saltline_world::build(&mut c, &k);
    // Foreground stacks frame the title shot; excluded from gameplay collision.
    for z in [-7., -17.] {
        for layer in 0..4 {
            let e = k.container(
                &mut c,
                Vec3::new(-7., layer as f32 * 2.85, z),
                1 + layer % 3,
            );
            c.entity(e).insert(title::TitleDressing);
        }
    }
    c.insert_resource(port);
    c.spawn((
        #[cfg(not(target_arch = "wasm32"))]
        bevy::light::VolumetricLight,
        DirectionalLight {
            illuminance: 10500.,
            color: Color::srgb(1., 0.87, 0.73),
            shadow_maps_enabled: true,
            contact_shadows_enabled: !cfg!(target_arch = "wasm32"),
            ..default()
        },
        Transform::from_xyz(65., 35., -80.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let cam = c
        .spawn((
            Camera3d::default(),
            title::GameplayCamera,
            bevy::light::Skybox {
                image: Some(sky.clone()),
                brightness: 2800.,
                ..default()
            },
            rendering::camera_effects(sky, 1400.),
            DistanceFog {
                color: Color::srgb(0.19, 0.26, 0.32),
                falloff: FogFalloff::Linear {
                    start: 35.,
                    end: 220.,
                },
                ..default()
            },
            Transform::from_xyz(0., 1.7, 8.),
        ))
        .id();
    let rifle = k.rifle(&mut c, cam);
    if !saltline_game::launch_args().any(|a| a == "rifle-side") {
        k.part(
            &mut c,
            rifle,
            Vec3::new(0.05, -0.26, 0.24),
            Vec3::new(0.14, 0.12, 0.4),
            2,
        );
        k.part(
            &mut c,
            rifle,
            Vec3::new(-0.08, -0.18, -0.25),
            Vec3::new(0.12, 0.11, 0.22),
            2,
        );
    }
    c.spawn((
        title::Crosshair,
        Text::new("+"),
        if saltline_game::launch_args().any(|a| a == "--inspect") {
            Visibility::Hidden
        } else {
            Visibility::Visible
        },
        TextFont {
            font_size: FontSize::Px(24.),
            ..default()
        },
        TextColor(Color::srgba(0.8, 0.95, 0.9, 0.6)),
        Node {
            position_type: PositionType::Absolute,
            left: percent(49.6),
            top: percent(48.),
            ..default()
        },
    ));

    let flash = c
        .spawn((
            Muzzle,
            PointLight {
                color: Color::srgb(1., 0.5, 0.1),
                intensity: 0.,
                range: 6.,
                ..default()
            },
            Transform::from_xyz(0.24, -0.15, -1.2),
        ))
        .id();
    c.entity(cam).add_child(flash);
    let mut relays = vec![];
    for p in RELAYS {
        relays.push(saltline_models::library::relay::build(&mut c, &k, p));
    }
    for i in 0..10 {
        let drone = i % 3 == 0;
        let p = Vec3::new(
            if i % 2 == 0 { -4. } else { 5. },
            if drone { 4. } else { 0. },
            -16. - i as f32 * 6.,
        );
        let e = k.root(&mut c, p);
        if drone {
            k.drone(&mut c, e)
        } else {
            k.hound(&mut c, e)
        }
        c.entity(e).insert(Enemy {
            drone,
            health: 100.,
            home: p,
            cooldown: 2.,
        });
    }

    for i in 0..96 {
        c.spawn((
            SurfaceRipple(i as f32 * 0.618),
            Mesh3d(k.ring.clone()),
            MeshMaterial3d(k.mats[4].clone()),
            Transform::from_xyz(
                ((i * 37) % 45) as f32 - 22.,
                0.035,
                10. - ((i * 19) % 94) as f32,
            )
            .with_scale(Vec3::new(0.2, 0.015, 0.2)),
        ));
    }
    // A pale translucent material keeps backlit rain visible against the stacks.
    let rain_material = mats.add(StandardMaterial {
        base_color: Color::srgba(0.67, 0.74, 0.78, 0.32),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    // Rain streaks use a single cached mesh.
    for i in 0..650 {
        let p = Vec3::new(
            ((i * 73) % 480) as f32 * 0.1 - 24.,
            ((i * 31) % 160) as f32 * 0.1,
            12. - ((i * 43) % 1000) as f32 * 0.1,
        );
        let e = k.root(&mut c, p);
        let streak = k.part(&mut c, e, Vec3::ZERO, Vec3::new(0.009, 0.45, 0.009), 4);
        c.entity(streak)
            .insert(MeshMaterial3d(rain_material.clone()));
        c.entity(e).insert(Rain);
    }
    c.spawn((
        Hud,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(20.),
            ..default()
        },
        TextColor(Color::srgb(0.77, 0.9, 0.89)),
        Node {
            position_type: PositionType::Absolute,
            left: px(35),
            top: px(28),
            ..default()
        },
    ));
    let view = saltline_game::launch_args()
        .skip_while(|x| x != "--inspect")
        .nth(1)
        .unwrap_or_default();
    c.insert_resource(Visuals {
        rifle,
        relays,
        capture: 0,
        view,
    });
}
#[derive(Component)]
struct Rain;
#[derive(Component)]
struct Muzzle;
fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    mut cursor: Query<&mut CursorOptions>,
    mut m: ResMut<Mission>,
    mut a: ResMut<Actions>,
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
) {
    if keys.just_pressed(KeyCode::Enter) && m.mode != Mode::Playing {
        if m.mode != Mode::Paused {
            *m = Mission::default();
            for (mut e, mut t) in &mut enemies {
                e.health = 100.;
                e.cooldown = 2.;
                t.translation = e.home;
                t.rotation = Quat::IDENTITY;
            }
        }
        m.mode = Mode::Playing;
        *a = Actions::default();
        return;
    }
    if keys.just_pressed(KeyCode::Escape) {
        m.mode = if m.mode == Mode::Playing {
            Mode::Paused
        } else if m.mode == Mode::Paused {
            Mode::Playing
        } else {
            Mode::Menu
        };
        *a = Actions::default();
        return;
    }
    for mut c in &mut cursor {
        c.visible = m.mode != Mode::Playing;
        c.grab_mode = if m.mode == Mode::Playing {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
        };
    }
    if m.mode != Mode::Playing {
        return;
    }
    m.yaw -= motion.delta.x * 0.002;
    m.pitch = (m.pitch - motion.delta.y * 0.002).clamp(-1.45, 1.45);
    a.movement = Vec2::new(
        (keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32) as f32,
        (keys.pressed(KeyCode::KeyW) as i32 - keys.pressed(KeyCode::KeyS) as i32) as f32,
    );
    a.fire = mouse.pressed(MouseButton::Left);
    a.aim = mouse.pressed(MouseButton::Right);
    a.sprint = keys.pressed(KeyCode::ShiftLeft);
    a.crouch = keys.pressed(KeyCode::KeyC);
    a.jump |= keys.just_pressed(KeyCode::Space);
    a.reload |= keys.just_pressed(KeyCode::KeyR);
    a.interact |= keys.just_pressed(KeyCode::KeyE);
}
fn visuals(
    mut c: Commands,
    time: Res<Time>,
    m: Res<Mission>,
    a: Res<Actions>,
    mut v: ResMut<Visuals>,
    mut transforms: Query<&mut Transform>,
    camera: Query<Entity, With<title::GameplayCamera>>,
    mut hud: Query<&mut Text, With<Hud>>,
    rain: Query<Entity, With<Rain>>,
    legs: Query<(Entity, &Leg)>,
    knees: Query<(Entity, &Knee)>,
    mut lights: Query<&mut PointLight, With<Muzzle>>,
) {
    let Ok(cam) = camera.single() else { return };
    if let Ok(mut t) = transforms.get_mut(cam) {
        *t = Transform::from_translation(m.position).with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            m.yaw,
            m.pitch,
            0.,
        ));
        if !v.view.is_empty() {
            *t =
                match v.view.as_str() {
                    "port" => Transform::from_xyz(-38., 32., 25.)
                        .looking_at(Vec3::new(8., 2., -40.), Vec3::Y),
                    "warehouse" => Transform::from_xyz(-26., 1.8, -23.)
                        .looking_at(Vec3::new(-32., 2., -27.), Vec3::Y),
                    "hound" => Transform::from_xyz(7., 1.8, -25.)
                        .looking_at(Vec3::new(5., 0.8, -22.), Vec3::Y),
                    "drone" => Transform::from_xyz(-1., 5., -19.)
                        .looking_at(Vec3::new(-4., 4., -16.), Vec3::Y),
                    "relay" => {
                        Transform::from_xyz(-2., 2.5, -9.).looking_at(RELAYS[0] + Vec3::Y, Vec3::Y)
                    }
                    _ => Transform::from_xyz(0., 1.7, 8.),
                };
        }
    }
    if let Ok(mut t) = transforms.get_mut(v.rifle) {
        t.translation = Vec3::new(
            if a.aim { 0. } else { 0.24 },
            if a.aim { -0.13 } else { -0.23 },
            -0.48 + m.cooldown * 0.4,
        );
        t.rotation = Quat::from_rotation_x(if m.reload > 0. {
            -0.6
        } else {
            m.cooldown * 0.4
        });
        if (m.mode == Mode::Menu && v.view.is_empty())
            || (!v.view.is_empty() && v.view != "rifle" && v.view != "rifle-side")
        {
            t.scale = Vec3::ZERO;
        } else {
            t.scale = Vec3::splat(0.75);
        }
    }
    if v.view == "rifle-side" {
        if let Ok(mut t) = transforms.get_mut(v.rifle) {
            t.translation = Vec3::new(0., 0., -1.65);
            t.rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
            t.scale = Vec3::splat(0.9);
        }
    }
    for mut light in &mut lights {
        light.intensity = if m.cooldown > 0.07 { 1800. } else { 0. };
    }
    if m.mode == Mode::Playing {
        for (e, leg) in &legs {
            if let Ok(mut t) = transforms.get_mut(e) {
                t.rotation = Quat::from_rotation_x((m.elapsed * 9. + leg.phase).sin() * 0.35);
            }
        }
    }
    if m.mode == Mode::Playing {
        for (e, knee) in &knees {
            if let Ok(mut t) = transforms.get_mut(e) {
                t.rotation =
                    Quat::from_rotation_x((m.elapsed * 9. + knee.phase + 0.7).sin() * 0.22);
            }
        }
    }
    if m.mode == Mode::Playing || m.mode == Mode::Menu {
        for e in &rain {
            if let Ok(mut t) = transforms.get_mut(e) {
                t.translation.y -= time.delta_secs() * 14.;
                if t.translation.y < 0. {
                    t.translation.y = 16.;
                }
            }
        }
    }
    for (i, e) in v.relays.iter().enumerate() {
        if let Ok(mut t) = transforms.get_mut(*e) {
            t.scale.y = if m.relays[i] {
                0.01
            } else {
                0.1 * (1.
                    + (if v.view.is_empty() {
                        time.elapsed_secs()
                    } else {
                        1.
                    } * 4.)
                        .sin()
                        * 0.3)
            };
        }
    }
    for mut text in &mut hud {
        if !v.view.is_empty() {
            text.0 = String::new();
            continue;
        }
        let done = m.relays.iter().filter(|x| **x).count();
        text.0 = match m.mode {
            Mode::Menu => String::new(),
            Mode::Paused => "S A L T L I N E  /  PAUSED\n\n[ ENTER / ESC ] RESUME".into(),
            Mode::Dead => "SIGNAL LOST\n\n[ ENTER ] REDEPLOY".into(),
            Mode::Victory => format!(
                "EXTRACTION CONFIRMED\n\nPort communications neutralized.\n{} threats destroyed · {:.0}s\n\n[ ENTER ] NEW OPERATION",
                m.kills, m.elapsed
            ),
            Mode::Playing => {
                let target = RELAYS
                    .iter()
                    .enumerate()
                    .find(|(i, _)| !m.relays[*i])
                    .map(|(_, p)| *p)
                    .unwrap_or(EXIT);
                format!(
                    "S A L T L I N E    /    OPERATION BREAKWATER\n\nRELAYS  {done} / 3     {}  {:.0}m\n\nHP {:03.0}    |    {:02} / {}   {}\n\n{}",
                    if done == 3 { "EXTRACTION" } else { "UPLINK" },
                    m.position.distance(target),
                    m.health,
                    m.ammo,
                    m.reserve,
                    if m.reload > 0. { "RELOADING" } else { "" },
                    if m.position.distance(target + Vec3::Y * 1.3) < 3. {
                        "[ E ] INTERACT"
                    } else {
                        ""
                    }
                )
            }
        };
    }
    if !v.view.is_empty() {
        v.capture += 1;
        if v.capture == 40 {
            c.spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!("tmp/screenshots/{}.png", v.view)));
        }
        if v.capture > 65 {
            c.write_message(AppExit::Success);
        }
    }
}

#[derive(Component)]
struct SurfaceRipple(f32);
fn surface_ripples(
    time: Res<Time>,
    mission: Res<Mission>,
    mut ripples: Query<(&SurfaceRipple, &mut Transform)>,
) {
    for (r, mut t) in &mut ripples {
        // Inspection views use a fixed phase for repeatable comparisons.
        let elapsed = if saltline_game::launch_args().any(|a| a == "--inspect") {
            1.
        } else {
            if mission.mode == Mode::Menu {
                time.elapsed_secs()
            } else {
                mission.elapsed
            }
        };
        let phase = (elapsed * 1.7 + r.0).fract();
        t.scale = Vec3::new(0.05 + phase * 0.7, 0.008 * (1. - phase), 0.05 + phase * 0.7);
    }
}
