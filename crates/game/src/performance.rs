//! Stable application metrics; benchmark distributions are retained separately.
use bevy::{
    diagnostic::{
        Diagnostic, DiagnosticPath, Diagnostics, EntityCountDiagnosticsPlugin,
        FrameTimeDiagnosticsPlugin, RegisterDiagnostic,
    },
    prelude::*,
};

pub const SYSTEM_PATHS: [&str; 7] = [
    "saltline/system/lib/simulate_ms",
    "saltline/system/main/controls_ms",
    "saltline/system/main/visuals_ms",
    "saltline/system/hud/update_ms",
    "saltline/system/presentation/rigs_ms",
    "saltline/system/rain/update_ms",
    "saltline/system/effects/combat_effects_ms",
];
pub const MAIN_CPU: DiagnosticPath = DiagnosticPath::const_new("saltline/main_world_ms");
pub const SUBMISSIONS: DiagnosticPath = DiagnosticPath::const_new("saltline/render/submissions");

#[derive(Resource)]
pub struct PerformanceSettings {
    pub enabled: bool,
}
pub struct PerformancePlugin {
    pub enabled: bool,
}
impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PerformanceSettings {
            enabled: self.enabled,
        });
        for (path, unit) in SYSTEM_PATHS
            .into_iter()
            .map(|p| (DiagnosticPath::new(p), " ms"))
            .chain([(MAIN_CPU, " ms"), (SUBMISSIONS, " submissions")])
        {
            let mut diagnostic = Diagnostic::new(path)
                .with_suffix(unit)
                .with_max_history_length(120);
            diagnostic.is_enabled = self.enabled;
            app.register_diagnostic(diagnostic);
        }
        if self.enabled {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                EntityCountDiagnosticsPlugin::default(),
                bevy::render::diagnostic::RenderDiagnosticsPlugin,
            ));
        }
    }
}

/// Records on scope exit, including early returns. Disabled mode reads no clock.
pub struct CpuMeasurement<'w, 's> {
    diagnostics: Option<Diagnostics<'w, 's>>,
    started: Option<std::time::Instant>,
    path: &'static str,
}
impl<'w, 's> CpuMeasurement<'w, 's> {
    pub fn new(
        settings: Option<&PerformanceSettings>,
        diagnostics: Option<Diagnostics<'w, 's>>,
        path: &'static str,
    ) -> Self {
        Self {
            diagnostics,
            started: settings
                .filter(|s| s.enabled)
                .map(|_| std::time::Instant::now()),
            path,
        }
    }
}
impl Drop for CpuMeasurement<'_, '_> {
    fn drop(&mut self) {
        if let (Some(started), Some(diagnostics)) = (self.started, self.diagnostics.as_mut()) {
            diagnostics.add_measurement(&DiagnosticPath::new(self.path), || {
                started.elapsed().as_secs_f64() * 1000.
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn record(settings: Res<PerformanceSettings>, diagnostics: Diagnostics) {
        let _sample = CpuMeasurement::new(Some(&settings), Some(diagnostics), SYSTEM_PATHS[0]);
    }
    #[test]
    fn disabled_metrics_are_registered_but_never_measured() {
        let mut app = App::new();
        app.add_plugins(PerformancePlugin { enabled: false })
            .add_systems(Update, record);
        app.update();
        let store = app.world().resource::<bevy::diagnostic::DiagnosticsStore>();
        let d = store.get(&DiagnosticPath::new(SYSTEM_PATHS[0])).unwrap();
        assert_eq!(d.suffix, " ms");
        assert!(d.measurement().is_none());
    }
    #[test]
    fn scope_exit_records_after_deferred_application() {
        let mut app = App::new();
        app.add_plugins(PerformancePlugin { enabled: false });
        app.world_mut()
            .resource_mut::<PerformanceSettings>()
            .enabled = true;
        app.world_mut()
            .resource_mut::<bevy::diagnostic::DiagnosticsStore>()
            .get_mut(&DiagnosticPath::new(SYSTEM_PATHS[0]))
            .unwrap()
            .is_enabled = true;
        app.add_systems(Update, record);
        app.update();
        assert_eq!(
            app.world()
                .resource::<bevy::diagnostic::DiagnosticsStore>()
                .get(&DiagnosticPath::new(SYSTEM_PATHS[0]))
                .unwrap()
                .history_len(),
            1
        );
    }
}
