//! Native player graphics controls. Benchmarks and captures never load preferences.
use crate::{capture, quality::RenderQuality};
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};
use saltline_game::{Actions, Mission, Mode};
use std::path::PathBuf;

pub fn isolated(args: &[String]) -> bool {
    crate::graphics_isolated(args)
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Saved {
    version: u32,
    quality: RenderQuality,
}
fn path() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .filter(|p| !p.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".config")))
        .map(|p| p.join("saltline/graphics.json"))
}
fn decode(bytes: &[u8]) -> Result<RenderQuality, String> {
    let saved: Saved = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    if saved.version != 1 || !saved.quality.valid() {
        return Err("unsupported version or invalid quality values".into());
    }
    Ok(saved.quality)
}
pub fn load() -> Option<RenderQuality> {
    let path = path()?;
    match std::fs::read(&path) {
        Ok(bytes) => match decode(&bytes) {
            Ok(q) => Some(q),
            Err(e) => {
                eprintln!("WARN Ignoring graphics preferences: {e}");
                None
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => {
            eprintln!("WARN Cannot read graphics preferences: {e}");
            None
        }
    }
}
fn save(q: &RenderQuality) -> Result<(), String> {
    let path = path().ok_or("No graphics preferences directory available")?;
    save_to(&path, q)
}
fn save_to(path: &std::path::Path, q: &RenderQuality) -> Result<(), String> {
    let parent = path.parent().ok_or("Invalid preferences path")?;
    std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
    let bytes = serde_json::to_vec_pretty(&Saved {
        version: 1,
        quality: q.clone(),
    })
    .map_err(|e| e.to_string())?;
    let result = (|| -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create(&temporary)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        std::fs::rename(&temporary, &path)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    result.map_err(|e| e.to_string())
}
#[derive(Resource, Default)]
pub struct GraphicsState {
    pub open: bool,
    pub consumed: bool,
    selected: usize,
    pending: Option<(usize, i32)>,
    status: String,
}
impl GraphicsState {
    pub fn open_from_home(&mut self) {
        self.open = true;
        self.selected = 0;
        self.pending = None;
    }
}
#[derive(Component)]
struct Root;
#[derive(Component)]
struct Page(bool);
#[derive(Component)]
struct Row(usize);
#[derive(Component)]
struct RowText(usize);
#[derive(Component)]
struct Status;
const LABELS: [&str; 15] = [
    "Preset",
    "Render scale",
    "Adaptive resolution",
    "Shadows",
    "Reflections",
    "Ambient occlusion",
    "Contact shadows",
    "Volumetric fog",
    "Bloom",
    "TAA",
    "Rain",
    "Wet surfaces",
    "Container detail",
    "Reset defaults",
    "Back",
];
const AMBER: Color = Color::srgb(0.9, 0.63, 0.25);
pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GraphicsState>()
            .add_systems(PostStartup, setup.after(capture::attach))
            .add_systems(Update, input.before(crate::controls))
            .add_systems(Update, display.after(input).after(crate::controls));
    }
}
fn setup(mut c: Commands, output: Res<capture::OutputCamera>) {
    c.spawn((Root, UiTargetCamera(output.0), GlobalZIndex(1_000_010), Visibility::Hidden,
        Node { position_type: PositionType::Absolute, right: px(16), top: px(24), width: px(490), padding: UiRect::all(px(18)), flex_direction: FlexDirection::Column, row_gap: px(5), ..default() },
        BackgroundColor(Color::srgba(0.025,0.035,0.045,0.96)),
    )).with_children(|root| {
        root.spawn((Text::new("SALTLINE / GRAPHICS"), TextFont { font_size: FontSize::Px(22.), ..default() }, TextColor(AMBER)));
        root.spawn((Page(false), Node { flex_direction: FlexDirection::Column, row_gap: px(10), ..default() })).with_children(|page| {
            for (row,label) in [(100,"Resume"),(101,"Graphics")] { spawn_button(page, row, label); }
        });
        root.spawn((Page(true), Node { flex_direction: FlexDirection::Column, row_gap: px(3), ..default() })).with_children(|page| {
            for (row,label) in LABELS.iter().enumerate() { spawn_button(page, row, label); }
            page.spawn((Status, Text::new(""), TextFont {font_size: FontSize::Px(14.), ..default()}, TextColor(AMBER)));
            page.spawn((Text::new("↑ ↓ select   ← → change   Esc back\nChanges apply live and save automatically.\nEffects may briefly compile new shaders."), TextFont {font_size: FontSize::Px(14.), ..default()}));
        });
    });
}
fn spawn_button(parent: &mut ChildSpawnerCommands, row: usize, label: &str) {
    if row < 13 {
        parent
            .spawn((
                Row(row),
                Hovered::default(),
                Node {
                    height: px(30),
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(px(5)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.15, 0.18, 0.21, 0.8)),
            ))
            .with_children(|line| {
                arrow(line, row, -1, "‹", label);
                line.spawn((
                    RowText(row),
                    Text::new(label),
                    TextFont {
                        font_size: FontSize::Px(16.),
                        ..default()
                    },
                    Node {
                        flex_grow: 1.,
                        ..default()
                    },
                ));
                arrow(line, row, 1, "›", label);
            });
    } else {
        parent
            .spawn((
                Button,
                AccessibleLabel(label.into()),
                Hovered::default(),
                Row(row),
                Node {
                    height: px(30),
                    padding: UiRect::horizontal(px(8)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.15, 0.18, 0.21, 0.8)),
                children![(
                    RowText(row),
                    Text::new(label),
                    TextFont {
                        font_size: FontSize::Px(16.),
                        ..default()
                    }
                )],
            ))
            .observe(move |_: On<Activate>, mut state: ResMut<GraphicsState>| {
                state.pending = Some((row, 1));
            });
    }
}
fn arrow(parent: &mut ChildSpawnerCommands, row: usize, delta: i32, glyph: &str, label: &str) {
    parent
        .spawn((
            Button,
            AccessibleLabel(format!(
                "{label}: {}",
                if delta < 0 { "previous" } else { "next" }
            )),
            Node {
                width: px(26),
                height: px(28),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                Text::new(glyph),
                TextFont {
                    font_size: FontSize::Px(20.),
                    ..default()
                }
            )],
        ))
        .observe(move |_: On<Activate>, mut state: ResMut<GraphicsState>| {
            state.pending = Some((row, delta));
        });
}
fn preset(q: &RenderQuality) -> &str {
    &q.preset
}
fn cycle(value: usize, count: usize, delta: i32) -> usize {
    (value as i32 + delta).rem_euclid(count as i32) as usize
}
fn change(q: &mut RenderQuality, row: usize, delta: i32) -> bool {
    match row {
        0 => {
            let names = ["low", "medium", "high"];
            let index = names.iter().position(|&n| n == preset(q)).unwrap_or(2);
            *q = RenderQuality::preset(names[cycle(index, 3, delta)]);
        }
        1 if !q.adaptive => {
            let step = (q.scale * 10.).round() as i32;
            q.scale = ((step + delta).clamp(5, 10) as f32) / 10.;
        }
        1 => return false,
        2 => q.adaptive = !q.adaptive,
        3 => {
            let sizes = [0, 512, 1024, 2048, 4096];
            let i = sizes.iter().position(|&n| n == q.shadows).unwrap_or(3);
            q.shadows = sizes[cycle(i, 5, delta)];
        }
        4 => q.ssr = !q.ssr,
        5 => q.ao = !q.ao,
        6 => q.contact = !q.contact,
        7 => q.volumetric = !q.volumetric,
        8 => q.bloom = !q.bloom,
        9 => q.taa = !q.taa,
        10 => q.rain = !q.rain,
        11 => q.detailed_wet = !q.detailed_wet,
        12 => q.detailed_containers = !q.detailed_containers,
        13 => *q = RenderQuality::preset("medium"),
        _ => return false,
    }
    if row != 0 && row != 13 {
        q.preset = "custom".into();
    }
    q.reset_history();
    true
}
fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GraphicsState>,
    mut q: ResMut<RenderQuality>,
    mut mission: ResMut<Mission>,
    mut actions: ResMut<Actions>,
) {
    state.consumed = false;
    if mission.mode == Mode::Menu && !state.open && keys.just_pressed(KeyCode::KeyG) {
        state.open_from_home();
    }
    if mission.mode != Mode::Paused && !(mission.mode == Mode::Menu && state.open) {
        state.open = false;
        state.pending = None;
        return;
    }
    // Own pause-page input so Enter never leaks through to gameplay.
    state.consumed = true;
    *actions = Actions::default();
    if keys.just_pressed(KeyCode::Escape) {
        if state.open {
            state.open = false;
            state.selected = 101;
        } else if mission.mode == Mode::Paused {
            mission.mode = Mode::Playing;
        }
        state.pending = None;
        return;
    }
    let rows: Vec<usize> = if state.open {
        (0..15).collect()
    } else {
        vec![100, 101]
    };
    if !rows.contains(&state.selected) {
        state.selected = rows[0];
    }
    let index = rows.iter().position(|&r| r == state.selected).unwrap_or(0);
    if keys.just_pressed(KeyCode::ArrowDown) {
        state.selected = rows[cycle(index, rows.len(), 1)];
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        state.selected = rows[cycle(index, rows.len(), -1)];
    }
    if keys.just_pressed(KeyCode::KeyG) && !state.open {
        state.pending = Some((101, 1));
    }
    if keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::ArrowRight)
    {
        state.pending = Some((state.selected, 1));
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        state.pending = Some((state.selected, -1));
    }
    if let Some((row, delta)) = state.pending.take() {
        if !rows.contains(&row) {
            return;
        }
        state.selected = row;
        match row {
            100 => mission.mode = Mode::Playing,
            101 => {
                state.open = true;
                state.selected = 0;
            }
            14 => {
                state.open = false;
                state.selected = 101;
            }
            _ => {
                if change(&mut q, row, delta) {
                    state.status = match save(&q) {
                        Ok(()) => "Saved".into(),
                        Err(e) => {
                            warn!("Cannot save graphics settings: {e}");
                            "Applied; preferences could not be saved".into()
                        }
                    };
                }
            }
        }
    }
}
fn display(
    state: Res<GraphicsState>,
    q: Res<RenderQuality>,
    mission: Res<Mission>,
    mut root: Query<&mut Visibility, With<Root>>,
    mut pages: Query<(&Page, &mut Node)>,
    mut texts: Query<(&RowText, &mut Text), Without<Status>>,
    mut status: Query<&mut Text, With<Status>>,
    mut buttons: Query<(&Row, &Hovered, &mut BackgroundColor)>,
) {
    for mut v in &mut root {
        *v = if mission.mode == Mode::Paused || (mission.mode == Mode::Menu && state.open) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for (page, mut node) in &mut pages {
        node.display = if page.0 == state.open {
            Display::Flex
        } else {
            Display::None
        };
    }
    let on = |v| if v { "On" } else { "Off" };
    for (row, mut text) in &mut texts {
        let value = match row.0 {
            0 => preset(&q).into(),
            1 => format!(
                "{:.0}%{}",
                q.scale * 100.,
                if q.adaptive { " (automatic)" } else { "" }
            ),
            2 => on(q.adaptive).into(),
            3 => {
                if q.shadows == 0 {
                    "Off".into()
                } else {
                    q.shadows.to_string()
                }
            }
            4 => on(q.ssr).into(),
            5 => on(q.ao).into(),
            6 => on(q.contact).into(),
            7 => on(q.volumetric).into(),
            8 => on(q.bloom).into(),
            9 => on(q.taa).into(),
            10 => on(q.rain).into(),
            11 => if q.detailed_wet { "Detailed" } else { "Simple" }.into(),
            12 => if q.detailed_containers {
                "Standard"
            } else {
                "Low"
            }
            .into(),
            _ => String::new(),
        };
        let label = if row.0 == 100 {
            "Resume"
        } else if row.0 == 101 {
            "Graphics"
        } else {
            LABELS[row.0]
        };
        text.0 = if value.is_empty() {
            label.into()
        } else {
            format!("{label}   {value}")
        };
    }
    for mut text in &mut status {
        text.0 = state.status.clone();
    }
    for (row, hovered, mut color) in &mut buttons {
        *color = if row.0 == state.selected || hovered.get() {
            BackgroundColor(Color::srgba(0.45, 0.3, 0.12, 1.))
        } else {
            BackgroundColor(Color::srgba(0.15, 0.18, 0.21, 0.8))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preferences_round_trip_and_reject_invalid_data() {
        let bytes = serde_json::to_vec(&Saved {
            version: 1,
            quality: RenderQuality::preset("low"),
        })
        .unwrap();
        let q = decode(&bytes).unwrap();
        assert_eq!(q.shadows, 512);
        assert!(!q.detailed_wet);
        assert!(decode(b"{}").is_err());
        let mut saved: Saved = serde_json::from_slice(&bytes).unwrap();
        saved.version = 2;
        assert!(decode(&serde_json::to_vec(&saved).unwrap()).is_err());
        saved.version = 1;
        saved.quality.scale = 0.2;
        assert!(decode(&serde_json::to_vec(&saved).unwrap()).is_err());
    }
    #[test]
    fn preferences_replace_atomically_and_report_io_failure() {
        let directory =
            std::env::temp_dir().join(format!("saltline-graphics-test-{}", std::process::id()));
        let path = directory.join("graphics.json");
        save_to(&path, &RenderQuality::preset("low")).unwrap();
        save_to(&path, &RenderQuality::preset("high")).unwrap();
        assert_eq!(
            decode(&std::fs::read(&path).unwrap()).unwrap().preset,
            "high"
        );
        assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 1);
        assert!(
            save_to(
                &path.join("invalid-child.json"),
                &RenderQuality::preset("medium")
            )
            .is_err()
        );
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn menu_input_cannot_resume_when_returning_to_pause_page() {
        let mut app = App::new();
        app.insert_resource(GraphicsState {
            open: true,
            selected: 14,
            ..default()
        })
        .insert_resource(RenderQuality::preset("medium"))
        .insert_resource(Mission {
            mode: Mode::Paused,
            ..default()
        })
        .init_resource::<Actions>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<bevy::input::mouse::AccumulatedMouseMotion>()
        .add_systems(Update, (input, crate::controls).chain());
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        app.update();
        assert!(!app.world().resource::<GraphicsState>().open);
        assert_eq!(app.world().resource::<Mission>().mode, Mode::Paused);
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.clear();
            keys.press(KeyCode::Escape);
        }
        app.update();
        assert_eq!(app.world().resource::<Mission>().mode, Mode::Playing);
        assert!(!app.world().resource::<Actions>().fire);
    }
    #[test]
    fn controls_and_presets() {
        let mut q = RenderQuality::preset("medium");
        assert_eq!(preset(&q), "medium");
        change(&mut q, 11, 1);
        assert!(!q.detailed_wet);
        assert_eq!(preset(&q), "custom");
        assert!(!change(&mut q, 1, -1));
        change(&mut q, 2, 1);
        for _ in 0..20 {
            change(&mut q, 1, -1);
        }
        assert_eq!(q.scale, 0.5);
        change(&mut q, 13, 1);
        assert_eq!(preset(&q), "medium");
    }
}
