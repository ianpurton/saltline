//! Warmed, repeatable hardware route; captures are deliberately a separate mode.
#[cfg(not(target_arch = "wasm32"))]
use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;
use saltline_game::*;
#[cfg(not(target_arch = "wasm32"))]
use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(not(target_arch = "wasm32"))]
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
    pub completed: bool,
    measuring: bool,
    observations: Vec<Observation>,
    seen: BTreeMap<String, Instant>,
    boundaries: Vec<(String, usize)>,
}
#[cfg(not(target_arch = "wasm32"))]
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
            completed: false,
            measuring: false,
            observations: Vec::new(),
            seen: BTreeMap::new(),
            boundaries: Vec::new(),
        }
    }
}
// The replay driver shares these flags; browser builds have no native benchmark state.
#[cfg(target_arch = "wasm32")]
#[derive(Resource, Default)]
pub struct Benchmark {
    pub enabled: bool,
    pub replay: bool,
    pub completed: bool,
}
#[cfg(not(target_arch = "wasm32"))]
pub fn begin(
    mut b: ResMut<Benchmark>,
    settings: Res<saltline_game::performance::PerformanceSettings>,
) {
    if b.enabled || settings.enabled {
        b.cpu_start = Instant::now();
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub fn end(
    mut b: ResMut<Benchmark>,
    mut diagnostics: bevy::diagnostic::Diagnostics,
    settings: Res<saltline_game::performance::PerformanceSettings>,
    counters: Res<BatchCounters>,
) {
    if settings.enabled {
        diagnostics.add_measurement(&saltline_game::performance::SUBMISSIONS, || {
            counters.0.load(std::sync::atomic::Ordering::Relaxed) as f64
        });
    }
    if (b.enabled && b.measuring) || (!b.enabled && settings.enabled) {
        let ms = b.cpu_start.elapsed().as_secs_f32() * 1000.;
        if b.enabled {
            b.cpu.push(ms);
        }
        if settings.enabled {
            diagnostics.add_measurement(&saltline_game::performance::MAIN_CPU, || ms as f64);
        }
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
    let previous_stage = b.stage;
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
                1 => saltline_world::layout::view("corridor").0,
                2 => saltline_world::layout::view("waterfront").0,
                _ => saltline_world::EXIT + Vec3::Y * 1.7,
            };
            let (_, target) = saltline_world::layout::view(if stage == 1 {
                "corridor"
            } else if stage == 2 {
                "waterfront"
            } else {
                "extraction"
            });
            let direction = target - m.position;
            m.yaw = (-direction.x).atan2(-direction.z);
            m.pitch = 0.;
        }
        *a = Actions::default();
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
    }
    if let Some(stage) = frame.checked_sub(1).and_then(frame_sample_stage) {
        b.samples
            .entry(stage)
            .or_default()
            .push(time.delta_secs() * 1000.);
    }
    b.measuring = if frame < 720 {
        frame % 180 >= 60
    } else {
        frame > 780
    };
    if previous_stage != b.stage || frame == 0 {
        let stage = b.stage.to_owned();
        b.boundaries.push((stage, frame));
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
        let frames = frames.clone();
        print_stats(stage, &frames);
        all.extend(frames);
    }
    print_stats("all", &all);
    let cpu = b.cpu.clone();
    print_stats("main-world-cpu", &cpu);
    info!(
        "BENCHMARK quality={quality:?} output={:?} mesh_entities={meshes}",
        crate::capture::resolution()
    );
    let _ = diagnostics; // Raw observations, rather than rolling averages, are exported below.
}
#[cfg(not(target_arch = "wasm32"))]
fn print_stats(stage: &str, frames: &[f32]) {
    if let Some(stats) = stats(frames) {
        info!(
            "BENCHMARK stage={stage} samples={} median_ms={:.3} p95_ms={:.3} p99_ms={:.3} max_ms={:.3} worst_second_mean_ms={:.3}",
            stats.samples,
            stats.median_ms,
            stats.p95_ms,
            stats.p99_ms,
            stats.max_ms,
            stats.worst_second_mean_ms
        );
    }
}

/// CPU submissions are distinct from GPU indirect draws, which can be culled.
#[derive(Resource, Clone, Default)]
pub struct BatchCounters(pub std::sync::Arc<std::sync::atomic::AtomicUsize>);
pub fn install_counters(app: &mut App) {
    let counters = BatchCounters::default();
    app.insert_resource(counters.clone());
    if !launch_args().any(|a| a == "--benchmark" || a == "--profile")
        || launch_args().any(|a| a == "--no-instrumentation")
    {
        return;
    }
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

#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Serialize)]
struct Observation {
    path: String,
    stage: String,
    observation_ms: f64,
    measurement_ms: f64,
    value: f64,
}
#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Serialize, Debug)]
struct Stats {
    samples: usize,
    median_ms: f32,
    p95_ms: f32,
    p99_ms: f32,
    max_ms: f32,
    worst_second_mean_ms: f32,
}
#[cfg(not(target_arch = "wasm32"))]
fn stats(values: &[f32]) -> Option<Stats> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f32::total_cmp);
    let n = sorted.len();
    let (mut start, mut sum, mut worst) = (0, 0., 0f32);
    for (end, value) in values.iter().enumerate() {
        sum += value;
        while start < end && sum - values[start] >= 1000. {
            sum -= values[start];
            start += 1;
        }
        if sum >= 1000. {
            worst = worst.max(sum / (end - start + 1) as f32);
        }
    }
    Some(Stats {
        samples: n,
        median_ms: sorted[n / 2],
        p95_ms: sorted[(n - 1) * 95 / 100],
        p99_ms: sorted[(n - 1) * 99 / 100],
        max_ms: sorted[n - 1],
        worst_second_mean_ms: if worst == 0. {
            values.iter().sum::<f32>() / n as f32
        } else {
            worst
        },
    })
}

/// Runs after deferred diagnostics. Render results are labeled by observation time.
#[cfg(not(target_arch = "wasm32"))]
pub fn collect(mut b: ResMut<Benchmark>, diagnostics: Res<DiagnosticsStore>) {
    if !b.enabled {
        return;
    }
    for diagnostic in diagnostics.iter() {
        let path = diagnostic.path().as_str();
        for sample in diagnostic.measurements() {
            if b.seen.get(path).is_some_and(|last| sample.time <= *last) {
                continue;
            }
            b.seen.insert(path.to_owned(), sample.time);
            if !b.measuring {
                continue;
            }
            let stage = b.stage.to_owned();
            let observation_ms = b.started.elapsed().as_secs_f64() * 1000.;
            let measurement_ms = sample
                .time
                .saturating_duration_since(b.started)
                .as_secs_f64()
                * 1000.;
            b.observations.push(Observation {
                path: path.to_owned(),
                stage,
                observation_ms,
                measurement_ms,
                value: sample.value,
            });
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn finalize(
    mut b: ResMut<Benchmark>,
    quality: Res<crate::quality::RenderQuality>,
    diagnostics: Res<DiagnosticsStore>,
    meshes: Query<(), With<Mesh3d>>,
    adapter: Option<Res<bevy::render::renderer::RenderAdapterInfo>>,
    settings: Res<saltline_game::performance::PerformanceSettings>,
    mut exit: MessageWriter<AppExit>,
) {
    if !b.completed {
        return;
    }
    b.completed = false;
    report(&b, &quality, &diagnostics, meshes.iter().count());
    let stages: BTreeMap<_, _> = b
        .samples
        .iter()
        .map(|(name, samples)| {
            (
                *name,
                serde_json::json!({"raw_ms": samples, "stats": stats(samples)}),
            )
        })
        .collect();
    let mut distributions: BTreeMap<String, Vec<f32>> = BTreeMap::new();
    for observation in &b.observations {
        if observation.path.ends_with("_ms")
            || observation.path.ends_with("elapsed_cpu")
            || observation.path.ends_with("elapsed_gpu")
        {
            distributions
                .entry(observation.path.clone())
                .or_default()
                .push(observation.value as f32);
        }
    }
    let summaries: BTreeMap<_, _> = distributions
        .iter()
        .map(|(path, values)| (path, stats(values)))
        .collect();
    let gpu = b
        .observations
        .iter()
        .any(|v| v.path.ends_with("elapsed_gpu"));
    let units: BTreeMap<_, _> = diagnostics
        .iter()
        .map(|d| {
            (
                d.path().as_str(),
                if d.path().as_str().ends_with("elapsed_cpu")
                    || d.path().as_str().ends_with("elapsed_gpu")
                {
                    " ms"
                } else {
                    d.suffix.as_ref()
                },
            )
        })
        .collect();
    let artifact = serde_json::json!({
        "version": 1, "instrumentation_version": 1, "mission_passed": true,
        "resolution": crate::capture::resolution(), "quality": format!("{quality:?}"),
        "args": launch_args().collect::<Vec<_>>(), "adapter": adapter.as_ref().map(|a| format!("{:?}", ***a)),
        "build_profile": if cfg!(debug_assertions) { "debug" } else { "release" },
        "render_scale": quality.scale, "adaptive": quality.adaptive,
        "instrumentation": settings.enabled, "trace_chrome": cfg!(feature="trace-chrome"), "trace_tracy": cfg!(feature="trace-tracy"),
        "gpu_timings_available": gpu,
        "gpu_timings_note": if gpu { "measured" } else { "unavailable: instrumentation disabled or adapter/backend supplied no GPU samples" },
        "stages": stages, "stage_boundaries": b.boundaries, "observations": b.observations,
        "units": units, "metric_stats": summaries, "main_world_cpu": stats(&b.cpu),
        "render_sample_alignment": "stage at main-world observation time; asynchronous, not exact frame association"
    });
    let args: Vec<_> = launch_args().collect();
    let path = args
        .windows(2)
        .find(|w| w[0] == "--benchmark-output")
        .map(|w| w[1].as_str())
        .unwrap_or("tmp/logs/benchmark.json");
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path);
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_vec(&artifact)?)?;
        Ok(())
    })();
    if let Err(error) = result {
        error!("benchmark artifact failed: {error}");
        exit.write(AppExit::error());
    } else {
        info!("BENCHMARK artifact={path}");
        exit.write(AppExit::Success);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    #[test]
    fn stage_windows_exclude_settling_frames() {
        for stage in ["title", "corridor", "waterfront", "extraction"] {
            assert_eq!(
                (0..720)
                    .filter(|&f| frame_sample_stage(f) == Some(stage))
                    .count(),
                120
            );
        }
        assert_eq!(frame_sample_stage(59), None);
        assert_eq!(frame_sample_stage(60), Some("title"));
        assert_eq!(frame_sample_stage(179), Some("title"));
        assert_eq!(frame_sample_stage(180), None);
        assert_eq!(frame_sample_stage(780), None);
        assert_eq!(frame_sample_stage(781), Some("combat-and-mission"));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn deferred_final_frame_is_collected_once_and_warmup_is_excluded() {
        use bevy::diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
        const PATH: DiagnosticPath = DiagnosticPath::const_new("test/ms");
        fn emit(mut diagnostics: Diagnostics) {
            diagnostics.add_measurement(&PATH, || 7.);
        }
        let mut app = App::new();
        let mut benchmark = Benchmark::default();
        benchmark.enabled = true;
        app.insert_resource(benchmark)
            .register_diagnostic(Diagnostic::new(PATH));
        app.add_systems(Update, (emit, ApplyDeferred, collect).chain());
        app.update();
        assert!(app.world().resource::<Benchmark>().observations.is_empty());
        app.world_mut().resource_mut::<Benchmark>().measuring = true;
        app.world_mut().resource_mut::<Benchmark>().completed = true;
        app.update();
        assert_eq!(app.world().resource::<Benchmark>().observations.len(), 1);
        app.world_mut().run_system_cached(collect).unwrap();
        assert_eq!(app.world().resource::<Benchmark>().observations.len(), 1);
    }
    #[test]
    fn distributions_keep_tail_and_sustained_cost() {
        let mut values = vec![10.; 100];
        values.extend(vec![30.; 100]);
        let result = stats(&values).unwrap();
        assert_eq!(result.p95_ms, 30.);
        assert_eq!(result.p99_ms, 30.);
        assert_eq!(result.worst_second_mean_ms, 30.);
        assert!(stats(&[]).is_none());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn frame_sample_stage(frame: usize) -> Option<&'static str> {
    if frame < 720 {
        (frame % 180 >= 60).then(|| ["title", "corridor", "waterfront", "extraction"][frame / 180])
    } else {
        (frame > 780).then_some("combat-and-mission")
    }
}
