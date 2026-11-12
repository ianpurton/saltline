//! Shared, explicit quality policy. UI is composited at the output resolution.
use bevy::prelude::*;
#[derive(Resource, Clone, Debug)]
pub struct RenderQuality {
    pub high: bool,
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
    over: f32,
    under: f32,
    recent: std::collections::VecDeque<f32>,
}
impl RenderQuality {
    pub fn from_args() -> Self {
        Self::parse(&saltline_game::launch_args().collect::<Vec<_>>())
    }
    fn parse(args: &[String]) -> Self {
        let option = |key: &str| args.windows(2).find(|w| w[0] == key).map(|w| w[1].as_str());
        let high = option("--quality") == Some("high");
        let flag = |key: &str, default: bool| match option(key) {
            Some("on") => true,
            Some("off") => false,
            _ => default,
        };
        Self {
            high,
            #[cfg(not(target_arch = "wasm32"))]
            ssr: flag("--ssr", high),
            #[cfg(not(target_arch = "wasm32"))]
            ao: flag("--ao", high),
            contact: flag("--contact-shadows", high),
            #[cfg(not(target_arch = "wasm32"))]
            volumetric: flag("--volumetric", high),
            #[cfg(not(target_arch = "wasm32"))]
            bloom: flag("--bloom", true),
            #[cfg(not(target_arch = "wasm32"))]
            taa: flag("--taa", true),
            shadows: option("--shadow-size")
                .and_then(|s| s.parse::<u32>().ok())
                .filter(|n| [512, 1024, 2048, 4096].contains(n))
                .unwrap_or(if high { 4096 } else { 2048 }),
            scale: option("--render-scale")
                .and_then(|s| s.parse::<f32>().ok())
                .filter(|v| v.is_finite())
                .unwrap_or(1.)
                .clamp(0.7, 1.),
            adaptive: option("--render-scale").is_none()
                && (!args.iter().any(|s| {
                    s == "--benchmark"
                        || s == "--inspect"
                        || s == "--title-capture"
                        || s == "--mission-test"
                }) || args.iter().any(|s| s == "--adaptive")),
            over: 0.,
            under: 0.,
            recent: std::collections::VecDeque::with_capacity(60),
        }
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
            self.scale = (self.scale - 0.05).max(0.7);
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
    fn scaling_has_hysteresis_and_bounds() {
        let mut q = RenderQuality::parse(&[]);
        for _ in 0..100 {
            q.adapt(0.02);
        }
        assert_eq!(q.scale, 1.);
        for _ in 0..2000 {
            q.adapt(0.02);
        }
        assert_eq!(q.scale, 0.7);
        for _ in 0..5000 {
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
