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
                // UVs cover the whole face, including its bevels. A subdivision
                // must not repeat the entire texture independently.
                for i in start..b.positions.len() {
                    let p = Vec3::from(b.positions[i]);
                    b.uvs[i] = [
                        p.dot(u) / u.abs().dot(size) + 0.5,
                        p.dot(v) / v.abs().dot(size) + 0.5,
                    ];
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::mesh::VertexAttributeValues;
    #[test]
    fn subdivisions_keep_continuous_face_uvs() {
        let m = beveled_box(Vec3::ONE, 0.12, 3, true);
        let Some(VertexAttributeValues::Float32x3(p)) = m.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("positions")
        };
        let Some(VertexAttributeValues::Float32x2(uv)) = m.attribute(Mesh::ATTRIBUTE_UV_0) else {
            panic!("UVs")
        };
        let mut checked = 0;
        for (p, uv) in p.iter().zip(uv) {
            if (p[0] - 0.5).abs() < 1e-5 && p[1].abs() < 0.37 && p[2].abs() < 0.37 {
                assert!((uv[0] - (p[1] + 0.5)).abs() < 1e-5);
                assert!((uv[1] - (p[2] + 0.5)).abs() < 1e-5);
                checked += 1;
            }
        }
        assert!(checked > 0);
    }
}
