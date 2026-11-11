use super::mesh::Builder;
use bevy::prelude::*;
/// Parallel-transported frames avoid twists along curved cable paths. Ends are capped.
pub fn tube(path: &[Vec3], radius: f32, segments: usize) -> Mesh {
    assert!(path.len() > 1 && radius > 0. && segments >= 3);
    let mut b = Builder::default();
    let mut rings = vec![];
    let mut u = Vec3::X;
    for i in 0..path.len() {
        let t = (path[(i + 1).min(path.len() - 1)] - path[i.saturating_sub(1)]).normalize();
        u = (u - t * u.dot(t))
            .try_normalize()
            .unwrap_or_else(|| t.any_orthonormal_vector());
        let v = t.cross(u);
        rings.push(
            (0..segments)
                .map(|j| {
                    let a = j as f32 * std::f32::consts::TAU / segments as f32;
                    path[i] + radius * (u * a.cos() + v * a.sin())
                })
                .collect::<Vec<_>>(),
        );
    }
    for i in 0..rings.len() - 1 {
        for j in 0..segments {
            let k = (j + 1) % segments;
            b.quad([rings[i][j], rings[i][k], rings[i + 1][k], rings[i + 1][j]]);
        }
    }
    for j in 0..segments {
        let k = (j + 1) % segments;
        b.triangle(
            [path[0], rings[0][k], rings[0][j]],
            [Vec2::splat(0.5), Vec2::X, Vec2::Y],
        );
        let n = path.len() - 1;
        b.triangle(
            [path[n], rings[n][j], rings[n][k]],
            [Vec2::splat(0.5), Vec2::X, Vec2::Y],
        );
    }
    b.finish()
}
