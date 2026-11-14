//! Shared, explicit quality policy. UI is composited at the output resolution.
use bevy::prelude::*;
const MIN_SCALE: f32 = if cfg!(target_os = "linux") { 0.5 } else { 0.7 };
#[derive(Resource, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenderQuality {
    pub high: bool,
    pub preset: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub ssr: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pub ao: bool,
    pub contact: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pub volumetric: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pub bloom: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pub taa: bool,
    pub shadows: u32,
    pub scale: f32,
    pub adaptive: bool,
    pub rain: bool,
    pub detailed_wet: bool,
    pub detailed_containers: bool,
    #[serde(skip)]
    over: f32,
    #[serde(skip)]
    under: f32,
    #[serde(skip)]
    recent: std::collections::VecDeque<f32>,
}
impl RenderQuality {
    pub fn from_args() -> Self {
        let args = saltline_game::launch_args().collect::<Vec<_>>();
        #[cfg(target_os = "linux")]
        if !crate::graphics::isolated(&args) {
            return Self::with_saved(&args, crate::graphics::load());
        }
        Self::parse(&args)
    }
    pub(crate) fn preset(name: &str) -> Self {
        let high = name == "high";
        let low = name == "low";
        Self {
            high,
            preset: if high {
                "high"
            } else if low {
                "low"
            } else {
                "medium"
            }
            .into(),
            #[cfg(not(target_arch = "wasm32"))]
            ssr: high,
            #[cfg(not(target_arch = "wasm32"))]
            ao: high,
            contact: high,
            #[cfg(not(target_arch = "wasm32"))]
            volumetric: high,
            #[cfg(not(target_arch = "wasm32"))]
            bloom: true,
            #[cfg(not(target_arch = "wasm32"))]
            taa: true,
            shadows: if high {
                4096
            } else if low {
                512
            } else {
                2048
            },
            scale: if low { 0.7 } else { 1. },
            adaptive: true,
            rain: true,
            detailed_wet: !low,
            detailed_containers: !low,
            over: 0.,
            under: 0.,
            recent: Default::default(),
        }
    }
    fn parse(args: &[String]) -> Self {
        Self::with_saved(args, None)
    }
    pub(crate) fn with_saved(args: &[String], saved: Option<Self>) -> Self {
        let option = |key: &str| args.windows(2).find(|w| w[0] == key).map(|w| w[1].as_str());
        let saved = if crate::graphics_isolated(args) {
            None
        } else {
            saved
        };
        let mut q = if let Some(name) = option("--quality") {
            Self::preset(name)
        } else {
            saved.unwrap_or_else(|| Self::preset("medium"))
        };
        let flag = |key, old| match option(key) {
            Some("on") => true,
            Some("off") => false,
            _ => old,
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            q.ssr = flag("--ssr", q.ssr);
            q.ao = flag("--ao", q.ao);
            q.volumetric = flag("--volumetric", q.volumetric);
            q.bloom = flag("--bloom", q.bloom);
            q.taa = flag("--taa", q.taa);
        }
        q.contact = flag("--contact-shadows", q.contact);
        if let Some(size) = option("--shadow-size")
            .and_then(|v| v.parse().ok())
            .filter(|v| [0, 512, 1024, 2048, 4096].contains(v))
        {
            q.shadows = size;
        }
        if let Some(scale) = option("--render-scale") {
            q.scale = scale
                .parse::<f32>()
                .ok()
                .filter(|v| v.is_finite())
                .unwrap_or(1.)
                .clamp(MIN_SCALE, 1.);
            q.adaptive = false;
        }
        if crate::graphics_isolated(args) {
            q.adaptive =
                args.iter().any(|a| a == "--adaptive") && option("--render-scale").is_none();
        }
        if args.iter().any(|a| {
            matches!(
                a.as_str(),
                "--ssr"
                    | "--ao"
                    | "--contact-shadows"
                    | "--volumetric"
                    | "--bloom"
                    | "--taa"
                    | "--shadow-size"
                    | "--render-scale"
                    | "--adaptive"
            )
        }) {
            q.preset = "custom".into();
        }
        q.reset_history();
        q
    }
    pub fn reset_history(&mut self) {
        self.over = 0.;
        self.under = 0.;
        self.recent.clear();
    }
    pub fn valid(&self) -> bool {
        self.scale.is_finite()
            && (0.5..=1.).contains(&self.scale)
            && [0, 512, 1024, 2048, 4096].contains(&self.shadows)
            && ["low", "medium", "high", "custom"].contains(&self.preset.as_str())
    }
    pub fn adapt(&mut self, dt: f32) {
        if !self.adaptive || !(0.001..0.1).contains(&dt) {
            return;
        }
        if self.recent.len() == 60 {
            self.recent.pop_front();
        }
        self.recent.push_back(dt);
        if self.recent.len() < 60 {
            return;
        }
        let mut window = [0_f32; 60];
        for (dst, src) in window.iter_mut().zip(&self.recent) {
            *dst = *src;
        }
        window.sort_by(f32::total_cmp);
        let p95 = window[56];
        if p95 > 0.0169 {
            self.over += dt;
            self.under = 0.;
        } else if p95 < 0.0135 {
            self.under += dt;
            self.over = 0.;
        } else {
            self.over = 0.;
            self.under = 0.;
        }
        let old = self.scale;
        if self.over > 2. {
            self.scale = (self.scale - 0.05).max(MIN_SCALE);
            self.over = 0.;
        }
        if self.under > 5. {
            self.scale = (self.scale + 0.05).min(1.);
            self.under = 0.;
        }
        if old != self.scale {
            info!("RENDER SCALE {:.2}", self.scale);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn presets_and_overrides() {
        let q = RenderQuality::parse(&[]);
        assert!(!q.ssr && !q.volumetric);
        assert_eq!(q.shadows, 2048);
        let q = RenderQuality::parse(
            &["--quality", "high", "--ssr", "off", "--render-scale", "nan"].map(String::from),
        );
        assert!(!q.ssr && q.ao);
        assert_eq!(q.scale, 1.);
        assert!(!q.adaptive);
    }
    #[test]
    fn saved_settings_cli_precedence_and_isolation() {
        let mut saved = RenderQuality::preset("low");
        saved.bloom = false;
        let q = RenderQuality::with_saved(&[], Some(saved.clone()));
        assert!(!q.bloom);
        assert_eq!(q.shadows, 512);
        let args = ["--quality", "high", "--ssr", "off"].map(String::from);
        let q = RenderQuality::with_saved(&args, Some(saved.clone()));
        assert!(q.high && !q.ssr && q.bloom);
        assert_eq!(q.shadows, 4096);
        for mode in [
            "--benchmark",
            "--inspect",
            "--title-capture",
            "--headless-render",
            "--mission-test",
        ] {
            let q = RenderQuality::with_saved(&[mode.into()], Some(saved.clone()));
            assert!(q.bloom);
            assert_eq!(q.shadows, 2048);
            assert!(!q.adaptive);
        }
    }
    #[test]
    fn scaling_has_hysteresis_and_bounds() {
        let mut q = RenderQuality::parse(&[]);
        for _ in 0..100 {
            q.adapt(0.02);
        }
        assert_eq!(q.scale, 1.);
        for _ in 0..2000 {
            q.adapt(0.02);
        }
        assert_eq!(q.scale, MIN_SCALE);
        for _ in 0..6000 {
            q.adapt(0.01);
        }
        assert_eq!(q.scale, 1.);
    }
    #[test]
    fn mixed_slow_frames_reduce_scale_when_p95_exceeds_budget() {
        let mut q = RenderQuality::parse(&[]);
        for i in 0..400 {
            q.adapt(if i % 10 == 0 { 0.021 } else { 0.015 });
        }
        assert!(q.scale < 1.);
    }
}
