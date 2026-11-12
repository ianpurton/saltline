//! Warmed, repeatable hardware route; captures are deliberately a separate mode.
#[cfg(not(target_arch = "wasm32"))]
use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;
use saltline_game::*;
use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[derive(Resource)]
pub struct Benchmark {
    pub enabled: bool,
    pub replay: bool,
    frame: usize,
    #[cfg(not(target_arch = "wasm32"))]
    started: Instant,
    began: bool,
    samples: BTreeMap<&'static str, Vec<f32>>,
    stage: &'static str,
    #[cfg(not(target_arch = "wasm32"))]
    cpu_start: Instant,
    cpu: Vec<f32>,
}
impl Default for Benchmark {
    fn default() -> Self {
        Self {
            enabled: launch_args().any(|a| a == "--benchmark"),
            replay: false,
            frame: 0,
            #[cfg(not(target_arch = "wasm32"))]
            started: Instant::now(),
            began: false,
            samples: BTreeMap::new(),
            stage: "title",
            #[cfg(not(target_arch = "wasm32"))]
            cpu_start: Instant::now(),
            cpu: Vec::new(),
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub fn begin(mut b: ResMut<Benchmark>) {
    if b.enabled {
        b.cpu_start = Instant::now();
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub fn end(mut b: ResMut<Benchmark>) {
    if b.enabled && b.began {
        let ms = b.cpu_start.elapsed().as_secs_f32() * 1000.;
        b.cpu.push(ms);
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub fn route(
    mut b: ResMut<Benchmark>,
    ready: Res<crate::capture::RenderReady>,
    time: Res<Time<Real>>,
    mut m: ResMut<Mission>,
    mut a: ResMut<Actions>,
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
) {
    if !b.enabled {
        return;
    }
    if !ready.ready() {
        *a = Actions::default();
        return;
    }
    if !b.began {
        info!(
            "BENCHMARK startup_ms={:.2}",
            b.started.elapsed().as_secs_f64() * 1000.
        );
        b.began = true;
    }
    let frame = b.frame;
    b.frame += 1;
    // Each scenic view gets 60 settling frames followed by 120 measured frames.
    if frame < 720 {
        let stage = frame / 180;
        b.stage = ["title", "corridor", "waterfront", "extraction"][stage];
        if stage == 0 {
            m.mode = Mode::Menu;
        } else {
            m.mode = Mode::Playing;
            m.health = 100.;
            m.position = match stage {
                1 => Vec3::new(0., 1.7, -28.),
                2 => Vec3::new(21., 1.7, -40.),
                _ => saltline_world::EXIT + Vec3::Y * 1.7,
            };
            m.yaw = if stage == 2 { -1.3 } else { 0. };
            m.pitch = 0.;
        }
        *a = Actions::default();
        if frame % 180 >= 60 {
            let stage = b.stage;
            b.samples
                .entry(stage)
                .or_default()
                .push(time.delta_secs() * 1000.);
        }
    } else {
        if !b.replay {
            *m = Mission::default();
            m.mode = Mode::Playing;
            for (mut e, mut t) in &mut enemies {
                e.health = 100.;
                e.cooldown = 2.;
                t.translation = e.home;
                t.rotation = Quat::IDENTITY;
            }
            b.replay = true;
        }
        b.stage = "combat-and-mission";
        if frame > 780 {
            b.samples
                .entry("combat-and-mission")
                .or_default()
                .push(time.delta_secs() * 1000.);
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub fn report(
    b: &Benchmark,
    quality: &crate::quality::RenderQuality,
    diagnostics: &DiagnosticsStore,
    meshes: usize,
) {
    if !b.enabled {
        return;
    }
    let mut all = Vec::new();
    for (stage, frames) in &b.samples {
        let mut frames = frames.clone();
        frames.sort_by(f32::total_cmp);
        print_stats(stage, &frames);
        all.extend(frames);
    }
    all.sort_by(f32::total_cmp);
    print_stats("all", &all);
    let mut cpu = b.cpu.clone();
    cpu.sort_by(f32::total_cmp);
    print_stats("main-world-cpu", &cpu);
    info!(
        "BENCHMARK quality={quality:?} output={:?} mesh_entities={meshes}",
        crate::capture::resolution()
    );
    for d in diagnostics.iter() {
        if d.path().as_str().starts_with("render/")
            && d.measurement()
                .is_some_and(|m| m.time.elapsed().as_secs_f32() < 1.)
        {
            if let Some(value) = d.average() {
                info!("RENDER DIAGNOSTIC {} = {value:.3}", d.path().as_str());
            }
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn print_stats(stage: &str, frames: &[f32]) {
    if frames.is_empty() {
        return;
    }
    let n = frames.len();
    info!(
        "BENCHMARK stage={stage} samples={n} median_ms={:.3} p95_ms={:.3} p99_ms={:.3}",
        frames[n / 2],
        frames[(n - 1) * 95 / 100],
        frames[(n - 1) * 99 / 100]
    );
}

/// CPU submissions are distinct from GPU indirect draws, which can be culled.
#[derive(Resource, Clone, Default)]
pub struct BatchCounters(pub std::sync::Arc<std::sync::atomic::AtomicUsize>);
pub fn install_counters(app: &mut App) {
    let counters = BatchCounters::default();
    app.insert_resource(counters.clone());
    if let Some(render) = app.get_sub_app_mut(bevy::render::RenderApp) {
        render.insert_resource(counters).add_systems(
            bevy::render::Render,
            count_batches.in_set(bevy::render::RenderSystems::Cleanup),
        );
    }
}
fn count_batches(
    opaque: Res<
        bevy::render::render_phase::ViewBinnedRenderPhases<bevy::core_pipeline::core_3d::Opaque3d>,
    >,
    transparent: Res<
        bevy::render::render_phase::ViewSortedRenderPhases<
            bevy::core_pipeline::core_3d::Transparent3d,
        >,
    >,
    counts: Res<BatchCounters>,
) {
    let count: usize = opaque
        .0
        .values()
        .map(|p| {
            p.multidrawable_meshes.len()
                + p.batchable_meshes.len()
                + p.unbatchable_meshes
                    .values()
                    .map(|b| b.entities.len())
                    .sum::<usize>()
        })
        .sum::<usize>()
        + transparent.0.values().map(|p| p.items.len()).sum::<usize>();
    counts.0.store(count, std::sync::atomic::Ordering::Relaxed);
}
