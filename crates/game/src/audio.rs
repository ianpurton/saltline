//! Bounded procedural sound palette, embedded for portable native releases.
use bevy::{
    audio::{AudioSinkPlayback, Volume},
    prelude::*,
};
use saltline_game::*;
#[derive(Resource)]
struct Sounds {
    shot: Handle<AudioSource>,
    impact: Handle<AudioSource>,
    relay: Handle<AudioSource>,
    reload: Handle<AudioSource>,
    ui: Handle<AudioSource>,
    enemy: Handle<AudioSource>,
}
#[derive(Component)]
struct Voice;
#[derive(Component)]
struct Ambience(bool);
#[derive(Resource, Default)]
struct AudioState {
    mode: Option<Mode>,
    reload: bool,
    step: f32,
    enemy: f32,
}
pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioState>()
            .add_systems(PostStartup, setup)
            .add_systems(Update, update.after(crate::presentation::clock));
    }
}
fn setup(mut c: Commands, mut sources: ResMut<Assets<AudioSource>>) {
    let mut source = |bytes: &'static [u8]| {
        sources.add(AudioSource {
            bytes: bytes.into(),
        })
    };
    let sounds = Sounds {
        shot: source(include_bytes!("../../../assets/audio/shot.wav")),
        impact: source(include_bytes!("../../../assets/audio/impact.wav")),
        relay: source(include_bytes!("../../../assets/audio/relay.wav")),
        reload: source(include_bytes!("../../../assets/audio/reload.wav")),
        ui: source(include_bytes!("../../../assets/audio/ui.wav")),
        enemy: source(include_bytes!("../../../assets/audio/enemy.wav")),
    };
    for (source, mechanical) in [
        (
            source(include_bytes!("../../../assets/audio/rain.wav")),
            false,
        ),
        (
            source(include_bytes!("../../../assets/audio/machinery.wav")),
            true,
        ),
    ] {
        c.spawn((
            Ambience(mechanical),
            AudioPlayer::new(source),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(if mechanical {
                0.12
            } else {
                0.35
            })),
        ));
    }
    c.insert_resource(sounds);
}
fn update(
    mut c: Commands,
    m: Res<Mission>,
    actions: Res<Actions>,
    sounds: Res<Sounds>,
    clock: Res<crate::presentation::PresentationClock>,
    mut state: ResMut<AudioState>,
    mut events: MessageReader<PresentationEvent>,
    voices: Query<Entity, With<Voice>>,
    mut ambience: Query<(&Ambience, &mut AudioSink)>,
    mut sinks: Query<&mut AudioSink, (With<Voice>, Without<Ambience>)>,
    enemies: Query<(&Enemy, &Transform)>,
) {
    let paused = matches!(m.mode, Mode::Paused | Mode::Dead | Mode::Victory);
    let interior = m.position.x > -16.
        && m.position.x < 12.
        && m.position.z > -28.
        && m.position.z < -11.
        && m.position.y < 9.5;
    let alert = enemies
        .iter()
        .any(|(e, t)| e.health > 0. && t.translation.distance(m.position) < 18.);
    for (kind, mut sink) in &mut ambience {
        let level = if kind.0 {
            if alert { 0.20 } else { 0.12 }
        } else if interior {
            0.10
        } else {
            0.35
        };
        sink.set_volume(Volume::Linear(if paused { level * 0.2 } else { level }));
    }
    for sink in &mut sinks {
        if paused {
            sink.pause();
        } else {
            sink.play();
        }
    }
    let mut free = 24usize.saturating_sub(voices.iter().count());
    let mut play = |c: &mut Commands, source: Handle<AudioSource>, gain: f32| {
        if free > 0 {
            c.spawn((
                Voice,
                AudioPlayer::new(source),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(gain)),
            ));
            free -= 1;
        }
    };
    if state.mode != Some(m.mode) {
        play(&mut c, sounds.ui.clone(), 0.3);
        state.mode = Some(m.mode);
    }
    if clock.reset {
        for entity in &voices {
            c.entity(entity).despawn();
        }
        state.reload = false;
        state.step = 0.;
        state.enemy = 0.;
        events.clear();
    }
    if clock.dt == 0. {
        return;
    }
    if m.reload > 0. && !state.reload {
        play(&mut c, sounds.reload.clone(), 0.45);
    }
    state.reload = m.reload > 0.;
    state.enemy -= clock.dt;
    if alert && state.enemy <= 0. {
        play(&mut c, sounds.enemy.clone(), 0.12);
        state.enemy = 1.4;
    }
    state.step += clock.dt;
    if m.mode == Mode::Playing
        && actions.movement.length_squared() > 0.
        && m.vertical == 0.
        && state.step > if actions.sprint { 0.3 } else { 0.48 }
    {
        play(&mut c, sounds.impact.clone(), 0.08);
        state.step = 0.;
    }
    for event in events.read() {
        match event {
            PresentationEvent::Shot { .. } => play(&mut c, sounds.shot.clone(), 0.65),
            PresentationEvent::Impact { destroyed, .. } => play(
                &mut c,
                sounds.impact.clone(),
                if *destroyed { 0.55 } else { 0.28 },
            ),
            PresentationEvent::RelayCut { .. } => play(&mut c, sounds.relay.clone(), 0.45),
        }
    }
}
