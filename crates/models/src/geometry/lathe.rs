use super::mesh::Builder;
use bevy::prelude::*;
/// Radius/height profile revolved about Y; include radius-zero endpoints for caps.
pub fn lathe(profile: &[Vec2], segments: usize) -> Mesh {
    assert!(segments >= 3 && profile.len() >= 2);
    let mut b = Builder::default();
    for j in 0..segments {
        let a = j as f32 * std::f32::consts::TAU / segments as f32;
        let c = (j + 1) as f32 * std::f32::consts::TAU / segments as f32;
        let p = |v: Vec2, t: f32| Vec3::new(v.x * t.cos(), v.y, v.x * t.sin());
        for v in profile.windows(2) {
            b.quad([p(v[0], a), p(v[1], a), p(v[1], c), p(v[0], c)]);
        }
    }
    b.finish()
}
