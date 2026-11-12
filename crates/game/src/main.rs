mod audio;
mod benchmark;
mod capture;
mod driver;
mod effects;
#[cfg(not(target_arch = "wasm32"))]
mod finish;
mod hud;
mod presentation;
mod quality;
mod rain;
mod rendering;
mod title;
use bevy::pbr::DefaultOpaqueRendererMethod;
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    render::view::screenshot::save_to_disk,
    window::{CursorGrabMode, CursorOptions},
};
use saltline_game::*;
use saltline_models::Kit;
use saltline_world::RELAYS;
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
    let mut app = App::new();
    let quality = quality::RenderQuality::from_args();
    info!("RENDER QUALITY {quality:?}");
    app.insert_resource(quality.clone());
    app.insert_resource(SystemTimings::new(
        saltline_game::launch_args().any(|a| a == "--benchmark" || a == "--profile"),
    ));
    let plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: (!capture::headless()).then_some(Window {
            title: "SALTLINE / port security incident".into(),
            canvas: cfg!(target_arch = "wasm32").then(|| "#game".into()),
            fit_canvas_to_parent: cfg!(target_arch = "wasm32"),
            resolution: capture::resolution().into(),
            ..default()
        }),
        exit_condition: if capture::headless() {
            bevy::window::ExitCondition::DontExit
        } else {
            bevy::window::ExitCondition::OnPrimaryClosed
        },
        ..default()
    });
    let plugins = if capture::headless() || saltline_game::launch_args().any(|a| a == "--no-audio")
    {
        plugins.disable::<bevy::audio::AudioPlugin>()
    } else {
        plugins
    };
    if capture::headless() {
        app.add_plugins(plugins.disable::<bevy::winit::WinitPlugin>())
            .add_plugins(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_millis(1),
            ));
    } else {
        app.add_plugins(plugins);
    }
    capture::readiness(&mut app);
    benchmark::install_counters(&mut app);
    app.add_plugins(rain::RainPlugin);
    if !capture::headless() && !saltline_game::launch_args().any(|a| a == "--no-audio") {
        app.add_plugins(audio::SoundPlugin);
    }
    if saltline_game::launch_args().any(|a| a == "--benchmark" || a == "--profile") {
        app.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin);
    }
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(finish::FinishPlugin);
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(saltline_models::materials::wet::WetPlugin)
        .add_systems(Update, presentation::water.after(presentation::clock))
        .add_systems(Update, rendering::freeze_environment_maps);
    if saltline_game::launch_args().any(|a| a == "--inspect" || a == "--title-capture") {
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1. / 60.),
        ));
    }
    app.insert_resource(ClearColor(Color::srgb(0.16, 0.22, 0.28)))
        .insert_resource(bevy::light::GlobalAmbientLight {
            color: Color::srgb(0.68, 0.73, 0.78),
            brightness: 160.,
            ..default()
        })
        .insert_resource(if cfg!(target_arch = "wasm32") {
            DefaultOpaqueRendererMethod::forward()
        } else {
            DefaultOpaqueRendererMethod::deferred()
        })
        .insert_resource(bevy::light::DirectionalLightShadowMap {
            size: quality.shadows as usize,
        })
        .add_message::<PresentationEvent>()
        .init_resource::<presentation::PresentationClock>()
        .init_resource::<presentation::WeaponMotion>()
        .init_resource::<Mission>()
        .init_resource::<benchmark::Benchmark>()
        .add_systems(First, benchmark::begin)
        .add_systems(Last, benchmark::end)
        .init_resource::<Actions>()
        .add_plugins(title::TitlePlugin)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, hud::setup.after(capture::attach))
        .add_systems(PostStartup, saltline_models::geometry::batch::assemble)
        .add_systems(Update, capture::resize)
        .add_systems(Update, (hud::update, hud::feedback).chain().after(visuals))
        .add_systems(
            PostStartup,
            (title::setup, capture::attach, title::target_output).chain(),
        )
        .add_systems(PostStartup, (rain::setup, effects::setup))
        .add_systems(
            PostStartup,
            (presentation::bind_rigs, presentation::bind_viewmodel),
        )
        .add_systems(PostStartup, rendering::configure.after(title::setup))
        .add_systems(
            Update,
            (
                controls,
                benchmark::route,
                driver::drive,
                simulate,
                presentation::clock,
                presentation::weapon,
                visuals,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                surface_ripples,
                effects::combat_effects,
                presentation::rigs,
                presentation::magazines,
                rain::update,
                saltline_models::geometry::batch::update_lod,
            )
                .after(visuals),
        )
        .run();
}
fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    quality: Res<quality::RenderQuality>,
) {
    #[cfg(target_arch = "wasm32")]
    let sky = saltline_models::materials::sky::overcast(&mut images);
    #[cfg(not(target_arch = "wasm32"))]
    let sky = saltline_models::materials::sky::photographic(&mut images);
    c.insert_resource(rendering::SharedSky(sky.clone()));
    info!("RENDER QUALITY {quality:?}");
    let k = Kit::new(&mut meshes, &mut mats, &mut images);
    c.insert_resource(effects::DebrisAssets {
        mesh: k.hard.armor.clone(),
        sphere: k.ball.clone(),
        smoke: mats.add(StandardMaterial {
            base_color: Color::srgba(0.12, 0.13, 0.14, 0.16),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        material: k.mats[7].clone(),
        glow: mats.add(StandardMaterial {
            emissive: LinearRgba::rgb(9., 4., 1.),
            unlit: true,
            ..default()
        }),
    });
    #[cfg(not(target_arch = "wasm32"))]
    if quality.volumetric {
        rendering::mist(&mut c, &mut images);
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
            illuminance: 9500.,
            color: Color::srgb(1., 0.72, 0.42),
            shadow_maps_enabled: true,
            contact_shadows_enabled: quality.contact && !cfg!(target_arch = "wasm32"),
            ..default()
        },
        bevy::light::CascadeShadowConfigBuilder {
            maximum_distance: if quality.high { 170. } else { 100. },
            num_cascades: if quality.high { 4 } else { 2 },
            first_cascade_far_bound: 16.,
            ..default()
        }
        .build(),
        Transform::from_translation(saltline_models::materials::sky::SUN_DIRECTION * 100.)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let cam = c
        .spawn((
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection {
                fov: 74_f32.to_radians(),
                ..default()
            }),
            title::GameplayCamera,
            bevy::light::Skybox {
                image: Some(sky.clone()),
                brightness: rendering::SKY_BRIGHTNESS,
                ..default()
            },
            rendering::camera_effects(sky, rendering::ENVIRONMENT_BRIGHTNESS),
            rendering::fog(),
            bevy::camera::ShadowLodOrigin,
            Transform::from_xyz(0., 1.7, 8.),
        ))
        .id();
    let fill = c
        .spawn((
            PointLight {
                color: Color::srgb(0.68, 0.77, 0.86),
                intensity: 100.,
                range: 1.8,
                ..default()
            },
            Transform::from_xyz(-0.15, 0.15, -0.15),
        ))
        .id();
    c.entity(cam).add_child(fill);
    let rifle = k.rifle(&mut c, cam);
    if !saltline_game::launch_args().any(|a| a == "rifle-side") {
        k.gloves(&mut c, rifle);
    }
    let flash = c
        .spawn((
            Muzzle,
            PointLight {
                color: Color::srgb(1., 0.5, 0.1),
                intensity: 0.,
                range: 6.,
                ..default()
            },
            Transform::from_xyz(0., 0., -1.02),
        ))
        .id();
    c.entity(rifle).add_child(flash);
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
        c.entity(e)
            .insert(presentation::RigMotion::default())
            .insert(Enemy {
                drone,
                health: 100.,
                home: p,
                cooldown: 2.,
            });
    }

    let splash_material = mats.add(StandardMaterial {
        base_color: Color::srgba(0.52, 0.62, 0.65, 0.13),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    for i in 0..96 {
        c.spawn((
            SurfaceRipple(i as f32 * 0.618),
            Mesh3d(k.ring.clone()),
            MeshMaterial3d(splash_material.clone()),
            bevy::light::NotShadowCaster,
            Transform::from_xyz(
                ((i * 37) % 45) as f32 - 22.,
                0.035,
                10. - ((i * 19) % 94) as f32,
            )
            .with_scale(Vec3::new(0.2, 0.015, 0.2)),
        ));
    }
    #[cfg(not(target_arch = "wasm32"))]
    std::fs::create_dir_all(
        capture::option("--output-dir").unwrap_or_else(|| "tmp/screenshots".into()),
    )
    .expect("create capture directory");
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
struct Muzzle;
fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    mut cursor: Query<&mut CursorOptions>,
    mut m: ResMut<Mission>,
    mut a: ResMut<Actions>,
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
    timings: Option<Res<saltline_game::SystemTimings>>,
) {
    let _cpu = timings.as_deref().and_then(|t| t.span("main::controls"));
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
    m: Res<Mission>,
    a: Res<Actions>,
    mut v: ResMut<Visuals>,
    capture_target: Res<capture::CaptureTarget>,
    ready: Res<capture::RenderReady>,
    mut transforms: Query<&mut Transform>,
    camera: Query<Entity, With<title::GameplayCamera>>,
    clock: Res<presentation::PresentationClock>,
    motion: Res<presentation::WeaponMotion>,
    mut lights: Query<&mut PointLight, With<Muzzle>>,
    timings: Option<Res<saltline_game::SystemTimings>>,
) {
    let _cpu = timings.as_deref().and_then(|t| t.span("main::visuals"));
    let Ok(cam) = camera.single() else { return };
    if let Ok(mut t) = transforms.get_mut(cam) {
        *t = Transform::from_translation(m.position).with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            m.yaw,
            m.pitch + motion.recoil,
            motion.phase.sin() * 0.003 * (1. - motion.ads),
        ));
        t.translation.y -= motion.land;
        if !v.view.is_empty() {
            *t =
                match v.view.as_str() {
                    "corridor" => Transform::from_xyz(0., 1.7, -28.)
                        .looking_at(Vec3::new(0., 1.7, -65.), Vec3::Y),
                    "waterfront" => Transform::from_xyz(21., 1.7, -40.)
                        .looking_at(Vec3::new(65., 4., -60.), Vec3::Y),
                    "extraction" => Transform::from_xyz(0., 1.7, -75.)
                        .looking_at(saltline_world::EXIT + Vec3::Y, Vec3::Y),
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
        let inspecting = !v.view.is_empty();
        let aim = a.aim || v.view == "aim";
        let reload = if v.view == "reload" { 0.7 } else { m.reload };
        let moving = a.movement.length().min(1.);
        let sway = Vec3::new(
            motion.phase.sin() * 0.012,
            (motion.phase * 2.).cos() * 0.009,
            0.,
        ) * moving;
        let ads = if inspecting {
            aim as u32 as f32
        } else {
            motion.ads
        };
        let reload_phase = (1. - reload / 1.8).clamp(0., 1.);
        let reload_dip = if reload > 0. {
            (reload_phase * std::f32::consts::PI).sin()
        } else {
            0.
        };
        let pose = Vec3::new(
            0.24 * (1. - ads),
            -0.25 + 0.04 * ads - reload_dip * 0.10,
            -0.78 + 0.12 * ads,
        ) + sway
            + Vec3::Z * motion.kick;
        let rotation = Quat::from_euler(
            EulerRot::YXZ,
            -0.10 * (1. - ads),
            if reload > 0. {
                -0.65 * reload_dip
            } else {
                motion.recoil * 2.
            },
            if reload > 0. {
                -0.55 * reload_dip
            } else {
                0.025 * (1. - ads)
            },
        );
        let blend = if inspecting {
            1.
        } else {
            1. - (-clock.dt * 17.).exp()
        };
        t.translation = t.translation.lerp(pose, blend);
        t.rotation = t.rotation.slerp(rotation, blend);
        if (m.mode == Mode::Menu && v.view.is_empty())
            || (!v.view.is_empty()
                && v.view != "rifle"
                && v.view != "rifle-side"
                && v.view != "aim"
                && v.view != "reload"
                && v.view != "deployment"
                && v.view != "damage"
                && v.view != "corridor"
                && v.view != "waterfront"
                && v.view != "extraction")
        {
            t.scale = Vec3::ZERO;
        } else {
            t.scale = Vec3::splat(1.05);
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
        light.intensity = if m.cooldown > 0.085 { 8500. } else { 0. };
    }
    for (i, e) in v.relays.iter().enumerate() {
        if let Ok(mut t) = transforms.get_mut(*e) {
            t.scale.y = if m.relays[i] {
                0.01
            } else {
                0.1 * (1. + (if v.view.is_empty() { clock.elapsed } else { 1. } * 4.).sin() * 0.3)
            };
        }
    }
    if !v.view.is_empty() && ready.ready() {
        v.capture += 1;
        if v.capture == 180 {
            c.spawn(capture::screenshot(&capture_target))
                .observe(save_to_disk(capture::output(&v.view)))
                .observe(
                    |_: On<bevy::render::view::screenshot::ScreenshotCaptured>, mut c: Commands| {
                        c.write_message(AppExit::Success);
                    },
                );
        }
        if v.capture > 3000 {
            panic!("inspection capture timed out");
        }
    }
}

#[derive(Component)]
struct SurfaceRipple(f32);
fn surface_ripples(
    clock: Res<presentation::PresentationClock>,
    mut ripples: Query<(&SurfaceRipple, &mut Transform)>,
) {
    for (r, mut t) in &mut ripples {
        let phase = (clock.elapsed * 1.7 + r.0).fract();
        t.scale = Vec3::new(
            0.03 + phase * 0.45,
            0.004 * (1. - phase),
            0.03 + phase * 0.45,
        );
    }
}
