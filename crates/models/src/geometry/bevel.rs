use super::mesh::Builder;
use bevy::prelude::*;
/// Rounded box via six subdivided faces projected onto a rounded cuboid.
/// Smooth normals follow the bevel; flat mode preserves each facet.
pub fn beveled_box(size: Vec3, radius: f32, segments: usize, smooth: bool) -> Mesh {
    assert!(size.min_element() > 0. && segments > 0);
    let r = radius.clamp(0.0001, size.min_element() * 0.499);
    let inner = size / 2. - Vec3::splat(r);
    let mut b = Builder::default();
    for (n, u, v) in [
        (Vec3::X, Vec3::Y, Vec3::Z),
        (-Vec3::X, Vec3::Z, Vec3::Y),
        (Vec3::Y, Vec3::Z, Vec3::X),
        (-Vec3::Y, Vec3::X, Vec3::Z),
        (Vec3::Z, Vec3::X, Vec3::Y),
        (-Vec3::Z, Vec3::Y, Vec3::X),
    ] {
        let axis = |a: Vec3| {
            let h = a.abs().dot(size / 2.);
            let k = h - r;
            let mut t = vec![-h];
            for i in 0..=segments {
                t.push(-k + r * i as f32 / segments as f32);
            }
            t.push(k - r);
            for i in 0..=segments {
                t.push(k + r * i as f32 / segments as f32);
            }
            t.sort_by(f32::total_cmp);
            t.dedup();
            t
        };
        let us = axis(u);
        let vs = axis(v);
        for x in us.windows(2) {
            for y in vs.windows(2) {
                let project = |x: f32, y: f32| {
                    let p = n * n.abs().dot(size / 2.) + u * x + v * y;
                    let q = p.clamp(-inner, inner);
                    q + (p - q).normalize() * r
                };
                let start = b.positions.len();
                b.quad([
                    project(x[0], y[0]),
                    project(x[1], y[0]),
                    project(x[1], y[1]),
                    project(x[0], y[1]),
                ]);
                if smooth {
                    for i in start..b.positions.len() {
                        let p = Vec3::from(b.positions[i]);
                        b.normals[i] = (p - p.clamp(-inner, inner)).normalize().to_array();
                    }
                }
            }
        }
    }
    b.finish()
}
