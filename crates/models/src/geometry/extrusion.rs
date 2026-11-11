use super::mesh::Builder;
use bevy::prelude::*;
/// Extrudes a simple counterclockwise polygon along Z. Ear clipping supports concave profiles.
pub fn extrude(profile: &[Vec2], depth: f32) -> Mesh {
    assert!(profile.len() >= 3 && depth > 0.);
    let mut b = Builder::default();
    let mut ring: Vec<usize> = (0..profile.len()).collect();
    let point = |i: usize, z: f32| profile[i].extend(z);
    while ring.len() > 2 {
        let mut ear = None;
        for j in 0..ring.len() {
            let a = ring[(j + ring.len() - 1) % ring.len()];
            let c = ring[(j + 1) % ring.len()];
            let v = ring[j];
            let cross = |a: Vec2, b: Vec2, c: Vec2| (b - a).perp_dot(c - a);
            if cross(profile[a], profile[v], profile[c]) <= 1e-7 {
                continue;
            }
            if ring.iter().any(|&k| {
                k != a
                    && k != v
                    && k != c
                    && cross(profile[a], profile[v], profile[k]) >= 0.
                    && cross(profile[v], profile[c], profile[k]) >= 0.
                    && cross(profile[c], profile[a], profile[k]) >= 0.
            }) {
                continue;
            }
            ear = Some((j, a, v, c));
            break;
        }
        let (j, a, v, c) = ear.expect("profile must be simple, CCW and nondegenerate");
        b.triangle(
            [
                point(a, depth / 2.),
                point(v, depth / 2.),
                point(c, depth / 2.),
            ],
            [profile[a], profile[v], profile[c]],
        );
        b.triangle(
            [
                point(c, -depth / 2.),
                point(v, -depth / 2.),
                point(a, -depth / 2.),
            ],
            [profile[c], profile[v], profile[a]],
        );
        ring.remove(j);
    }
    for i in 0..profile.len() {
        let j = (i + 1) % profile.len();
        b.quad([
            point(i, -depth / 2.),
            point(j, -depth / 2.),
            point(j, depth / 2.),
            point(i, depth / 2.),
        ]);
    }
    b.finish()
}
