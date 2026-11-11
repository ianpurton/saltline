use super::mesh::Builder;
use bevy::prelude::*;
pub fn tapered(bottom: Vec2, top: Vec2, height: f32) -> Mesh {
    assert!(bottom.min_element() > 0. && top.min_element() > 0. && height > 0.);
    let mut b = Builder::default();
    let ring = |s: Vec2, y: f32| {
        [
            Vec3::new(-s.x / 2., y, -s.y / 2.),
            Vec3::new(-s.x / 2., y, s.y / 2.),
            Vec3::new(s.x / 2., y, s.y / 2.),
            Vec3::new(s.x / 2., y, -s.y / 2.),
        ]
    };
    let a = ring(bottom, -height / 2.);
    let t = ring(top, height / 2.);
    b.quad([a[3], a[2], a[1], a[0]]);
    b.quad(t);
    for i in 0..4 {
        let j = (i + 1) % 4;
        b.quad([a[i], a[j], t[j], t[i]]);
    }
    b.finish()
}
