use crate::Hud;
use bevy::prelude::*;
use saltline_game::{Mission, Mode, visible};
use saltline_world::{EXIT, Port, RELAYS};
#[derive(Component)]
pub enum Value {
    Objective,
    Health,
    Ammo,
    Prompt,
    Overlay,
    Compass,
}
#[derive(Component)]
pub struct PlayingPanel;
#[derive(Component)]
pub struct Damage;
fn text(value: &str, size: f32, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(value),
        TextFont {
            font: font.clone().into(),
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.91, 0.84)),
    )
}
pub fn setup(
    mut c: Commands,
    mut fonts: ResMut<Assets<Font>>,
    output: Res<crate::capture::OutputCamera>,
) {
    let camera = output.0;
    let font = fonts.add(Font::from_bytes(
        include_bytes!("../../../assets/fonts/Oswald.ttf").to_vec(),
    ));
    c.spawn((
        Hud,
        UiTargetCamera(camera),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        },
    ))
    .with_children(|root| {
        root.spawn((
            Damage,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                border: UiRect::all(px(6)),
                ..default()
            },
            BorderColor::all(Color::NONE),
        ));
        for (value, left, right, top, bottom, width, size, label) in [
            (
                Value::Objective,
                px(32),
                Val::Auto,
                px(28),
                Val::Auto,
                px(270),
                18.,
                "OPERATION BREAKWATER",
            ),
            (
                Value::Health,
                px(32),
                Val::Auto,
                Val::Auto,
                px(28),
                px(150),
                24.,
                "SUIT INTEGRITY",
            ),
            (
                Value::Ammo,
                Val::Auto,
                px(32),
                Val::Auto,
                px(28),
                px(165),
                28.,
                "PAC-9  /  AUTO",
            ),
        ] {
            root.spawn((
                PlayingPanel,
                Node {
                    position_type: PositionType::Absolute,
                    left,
                    right,
                    top,
                    bottom,
                    width,
                    padding: UiRect::all(px(8)),
                    border: UiRect::left(px(2)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(3),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.018, 0.026, 0.03, 0.18)),
                BorderColor::all(Color::srgb(0.91, 0.57, 0.22)),
            ))
            .with_children(|panel| {
                panel.spawn(text(label, 11., &font));
                panel.spawn((value, text("", size, &font)));
            });
        }
        root.spawn((
            PlayingPanel,
            Value::Compass,
            text("", 15., &font),
            TextLayout::justify(Justify::Center),
            Node {
                position_type: PositionType::Absolute,
                left: percent(35.),
                top: px(25),
                width: percent(30.),
                ..default()
            },
        ));
        for i in 0..4 {
            root.spawn((
                PlayingPanel,
                Marker(i),
                text("", 14., &font),
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                TextLayout::justify(Justify::Center),
            ));
        }
        for i in 0..10 {
            root.spawn((
                PlayingPanel,
                IntegrityPip(i),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(40. + i as f32 * 13.),
                    bottom: px(24),
                    width: px(10),
                    height: px(4),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));
        }
        for i in 0..4 {
            root.spawn((
                PlayingPanel,
                CrossLine(i),
                Node {
                    position_type: PositionType::Absolute,
                    width: px(if i < 2 { 7. } else { 1. }),
                    height: px(if i < 2 { 1. } else { 7. }),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.91, 0.90, 0.85)),
            ));
        }
        root.spawn((
            PlayingPanel,
            HitMarker,
            text("×", 24., &font),
            Node {
                position_type: PositionType::Absolute,
                left: percent(49.5),
                top: percent(48.3),
                ..default()
            },
        ));
        root.spawn((
            PlayingPanel,
            Value::Prompt,
            text("", 19., &font),
            Node {
                position_type: PositionType::Absolute,
                top: percent(58),
                width: percent(100),
                justify_content: JustifyContent::Center,
                ..default()
            },
            TextLayout::justify(Justify::Center),
        ));
        root.spawn((
            Value::Overlay,
            text("", 32., &font),
            Node {
                position_type: PositionType::Absolute,
                top: percent(36),
                width: percent(100),
                padding: UiRect::vertical(px(24)),
                ..default()
            },
            TextLayout::justify(Justify::Center),
        ));
    });
}
pub fn update(
    m: Res<Mission>,
    port: Res<Port>,
    mut values: Query<(&Value, &mut Text)>,
    mut panels: Query<&mut Visibility, With<PlayingPanel>>,
    mut damage: Query<&mut BorderColor, With<Damage>>,
    performance: Option<Res<saltline_game::performance::PerformanceSettings>>,
    diagnostics: Option<bevy::diagnostic::Diagnostics>,
) {
    let _trace = info_span!("hud::update").entered();
    let _cpu = saltline_game::performance::CpuMeasurement::new(
        performance.as_deref(),
        diagnostics,
        "saltline/system/hud/update_ms",
    );
    let inspect = saltline_game::launch_args().any(|a| a == "--inspect");
    for mut visibility in &mut panels {
        *visibility = if m.mode == Mode::Playing && !inspect {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    let done = m.relays.iter().filter(|&&x| x).count();
    let target = RELAYS
        .iter()
        .enumerate()
        .find(|(i, _)| !m.relays[*i])
        .map(|(_, p)| *p)
        .unwrap_or(EXIT);
    let in_range = m.position.distance(target + Vec3::Y * 1.3) < 3.
        && visible(m.position, target + Vec3::Y * 1.3, &port);
    for (value, mut t) in &mut values {
        t.0 = if inspect {
            String::new()
        } else {
            match value {
                Value::Compass => {
                    let heading = (-m.yaw.to_degrees()).rem_euclid(360.);
                    let direction = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"]
                        [((heading + 22.5) / 45.) as usize % 8];
                    format!("·     ·     ·     {direction}  {heading:03.0}     ·     ·     ·")
                }
                Value::Objective => format!(
                    "UPLINKS  {done} / 3\n{}  ·  {:.0} m",
                    if done == 3 {
                        "EXTRACTION"
                    } else {
                        "NEXT UPLINK"
                    },
                    m.position.distance(target)
                ),
                Value::Health => format!("{:03.0}  /  100", m.health.max(0.)),
                Value::Ammo => format!(
                    "{:02}  /  {:03}{}",
                    m.ammo,
                    m.reserve,
                    if m.reload > 0. { "\nRELOADING" } else { "" }
                ),
                Value::Prompt => {
                    if in_range {
                        if done == 3 {
                            "[ E ]  EXTRACT"
                        } else {
                            "[ E ]  CUT UPLINK"
                        }
                        .into()
                    } else {
                        String::new()
                    }
                }
                Value::Overlay => match m.mode {
                    Mode::Paused => "OPERATION PAUSED\n\nENTER / ESC  —  RESUME".into(),
                    Mode::Dead => "SIGNAL LOST\n\nENTER  —  REDEPLOY".into(),
                    Mode::Victory => format!(
                        "EXTRACTION CONFIRMED\n\n{} threats neutralized  ·  {:.0} seconds\n\nENTER  —  NEW OPERATION",
                        m.kills, m.elapsed
                    ),
                    _ => String::new(),
                },
            }
        };
    }
    for mut border in &mut damage {
        border.set_all(Color::srgba(0.8, 0.13, 0.06, (m.hurt * 2.).min(0.65)));
    }
}

#[derive(Component)]
pub struct Marker(usize);
#[derive(Component)]
pub struct IntegrityPip(usize);
#[derive(Component)]
pub struct CrossLine(usize);
#[derive(Component)]
pub struct HitMarker;
pub fn feedback(
    m: Res<Mission>,
    actions: Res<saltline_game::Actions>,
    motion: Res<crate::presentation::WeaponMotion>,
    cameras: Query<(&Camera, &GlobalTransform), With<crate::title::GameplayCamera>>,
    mut markers: Query<(&Marker, &mut Node, &mut Text, &mut Visibility)>,
    mut pips: Query<(&IntegrityPip, &mut BackgroundColor)>,
    mut lines: Query<(&CrossLine, &mut Node), Without<Marker>>,
    mut hits: Query<&mut TextColor, With<HitMarker>>,
) {
    let inspect = saltline_game::launch_args().any(|a| a == "--inspect");
    if let Ok((camera, transform)) = cameras.single() {
        if let Some(size) = camera.logical_viewport_size() {
            for (marker, mut node, mut text, mut visibility) in &mut markers {
                let p = if marker.0 < 3 {
                    RELAYS[marker.0] + Vec3::Y * 3.
                } else {
                    EXIT + Vec3::Y * 2.
                };
                let active = if marker.0 < 3 {
                    !m.relays[marker.0]
                } else {
                    m.relays.iter().all(|x| *x)
                };
                let projected = camera.world_to_viewport(transform, p);
                if active && !inspect && m.mode == Mode::Playing {
                    if let Ok(screen) = projected {
                        let uv = screen / size;
                        if (0.04..0.94).contains(&uv.x) && (0.06..0.90).contains(&uv.y) {
                            node.left = percent(uv.x * 100.);
                            node.top = percent(uv.y * 100.);
                            text.0 = format!(
                                "◇ {}  {:.0} m",
                                if marker.0 < 3 { "UPLINK" } else { "EXTRACT" },
                                m.position.distance(p)
                            );
                            *visibility = Visibility::Inherited;
                            continue;
                        }
                    }
                }
                *visibility = Visibility::Hidden;
            }
        }
    }
    for (pip, mut color) in &mut pips {
        color.0 = if m.health > pip.0 as f32 * 10. {
            Color::srgb(0.91, 0.90, 0.85)
        } else {
            Color::srgba(0.91, 0.90, 0.85, 0.15)
        };
    }
    let gap = 5. + m.cooldown * 70. + if actions.sprint { 8. } else { 0. };
    for (line, mut node) in &mut lines {
        let (x, y) = match line.0 {
            0 => (-gap - 7., 0.),
            1 => (gap, 0.),
            2 => (0., -gap - 7.),
            _ => (0., gap),
        };
        node.left = Val::Percent(50.);
        node.top = Val::Percent(50.);
        node.margin = UiRect {
            left: px(x),
            top: px(y),
            ..default()
        };
        node.display = if actions.aim {
            Display::None
        } else {
            Display::Flex
        };
    }
    for mut color in &mut hits {
        color.0 = Color::srgba(1., 0.94, 0.8, (motion.hit / 0.08).min(1.));
    }
}
