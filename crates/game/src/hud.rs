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
    camera: Query<Entity, With<crate::title::GameplayCamera>>,
) {
    let Ok(camera) = camera.single() else { return };
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
                "B4-07  /  AUTO",
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
                    padding: UiRect::all(px(14)),
                    border: UiRect::left(px(2)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(3),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.018, 0.026, 0.03, 0.75)),
                BorderColor::all(Color::srgb(0.91, 0.57, 0.22)),
            ))
            .with_children(|panel| {
                panel.spawn(text(label, 11., &font));
                panel.spawn((value, text("", size, &font)));
            });
        }
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
) {
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
