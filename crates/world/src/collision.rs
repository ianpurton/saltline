//! Map collision uses authored, yaw-oriented boxes, independent of render detail.
use bevy::prelude::*;
use std::collections::VecDeque;
#[derive(Clone, Copy, Debug)]
pub struct CollisionFlags {
    pub player: bool,
    pub enemy: bool,
    pub bullets: bool,
    pub navigation: bool,
}
impl CollisionFlags {
    pub const SOLID: Self = Self {
        player: true,
        enemy: true,
        bullets: true,
        navigation: true,
    };
    pub const PLAYER_ONLY: Self = Self {
        player: true,
        enemy: false,
        bullets: false,
        navigation: false,
    };
    pub const BULLET_ONLY: Self = Self {
        player: false,
        enemy: false,
        bullets: true,
        navigation: false,
    };
}
#[derive(Clone, Copy, Debug)]
pub struct Collider {
    pub center: Vec3,
    pub half: Vec3,
    pub yaw: f32,
    pub flags: CollisionFlags,
    pub surface: &'static str,
}
impl Collider {
    pub fn local(&self, p: Vec3) -> Vec3 {
        Quat::from_rotation_y(-self.yaw) * (p - self.center)
    }
    fn overlaps(&self, feet: Vec3, height: f32, radius: f32) -> bool {
        let p = self.local(feet);
        let dx = (p.x.abs() - self.half.x).max(0.);
        let dz = (p.z.abs() - self.half.z).max(0.);
        dx * dx + dz * dz < radius * radius
            && p.y + height > -self.half.y + 0.005
            && p.y < self.half.y - 0.005
    }
    fn intersects_segment(&self, a: Vec3, b: Vec3) -> bool {
        let a = self.local(a);
        let d = self.local(b) - a;
        let mut enter = 0_f32;
        let mut leave = 1_f32;
        for i in 0..3 {
            if d[i].abs() < 0.000001 {
                if a[i].abs() > self.half[i] {
                    return false;
                }
            } else {
                let x = (-self.half[i] - a[i]) / d[i];
                let y = (self.half[i] - a[i]) / d[i];
                enter = enter.max(x.min(y));
                leave = leave.min(x.max(y));
                if enter > leave {
                    return false;
                }
            }
        }
        leave > 0.001 && enter < 0.999
    }
}
#[derive(Resource, Default)]
pub struct Port {
    pub colliders: Vec<Collider>,
}
impl Port {
    pub fn add_box(
        &mut self,
        center: Vec3,
        size: Vec3,
        yaw: f32,
        flags: CollisionFlags,
        surface: &'static str,
    ) {
        self.colliders.push(Collider {
            center,
            half: size * 0.5,
            yaw,
            flags,
            surface,
        });
    }
    pub fn clear(&self, feet: Vec3, height: f32, radius: f32, player: bool) -> bool {
        if feet.x < -39.6 || feet.x > 39.6 || feet.z < -29.6 || feet.z > 29.6 {
            return false;
        }
        !self.colliders.iter().any(|c| {
            (if player {
                c.flags.player
            } else {
                c.flags.enemy
            }) && c.overlaps(feet, height, radius)
        })
    }
    pub fn visible(&self, a: Vec3, b: Vec3) -> bool {
        !self
            .colliders
            .iter()
            .any(|c| c.flags.bullets && c.intersects_segment(a, b))
    }
    pub fn ground_height(&self, p: Vec3, max_y: f32) -> Option<f32> {
        let mut height =
            if (-40. ..=30.).contains(&p.x) && (-30. ..=30.).contains(&p.z) && max_y >= -0.005 {
                Some(0_f32)
            } else {
                None
            };
        for c in &self.colliders {
            if !c.flags.player || !c.flags.enemy {
                continue;
            }
            let top = c.center.y + c.half.y;
            let local = c.local(p);
            if (local.x.abs() - c.half.x).max(0.).powi(2)
                + (local.z.abs() - c.half.z).max(0.).powi(2)
                <= 0.32_f32.powi(2)
                && top <= max_y + 0.005
            {
                height = Some(height.map_or(top, |old| old.max(top)));
            }
        }
        height
    }
    /// Horizontal movement is subdivided to avoid tunnelling at low frame rates.
    pub fn move_player(&self, mut feet: Vec3, displacement: Vec3, height: f32, step: bool) -> Vec3 {
        let count = (displacement.xz().length() / 0.12).ceil().max(1.) as usize;
        for _ in 0..count {
            for axis in [
                Vec3::X * displacement.x / count as f32,
                Vec3::Z * displacement.z / count as f32,
            ] {
                let next = feet + axis;
                if self.clear(next, height, 0.32, true) {
                    feet = next;
                } else if step {
                    let raised = next + Vec3::Y * 0.23;
                    if self.clear(raised, height, 0.32, true) {
                        if let Some(y) = self.ground_height(next, raised.y) {
                            let candidate = Vec3::new(next.x, y, next.z);
                            if y >= feet.y - 0.005 && self.clear(candidate, height, 0.32, true) {
                                feet = candidate;
                            }
                        }
                    }
                }
            }
        }
        feet
    }
    pub fn walkable(&self, p: Vec3) -> bool {
        if p.x < -39.6 || p.x >= 29. || p.z < -29.6 || p.z > 29.6 {
            return false;
        }
        self.ground_height(p, 0.3).is_some_and(|y| {
            let feet = Vec3::new(p.x, y, p.z);
            !self
                .colliders
                .iter()
                .any(|c| c.flags.navigation && c.overlaps(feet, 1.8, 0.4))
        })
    }
    pub fn repair_spawn(&self, p: Vec3) -> Vec3 {
        if self.walkable(p) {
            return Vec3::new(p.x, self.ground_height(p, 0.3).unwrap_or(0.), p.z);
        }
        for r in 1_i32..=8 {
            for z in -r..=r {
                for x in -r..=r {
                    if x.abs() != r && z.abs() != r {
                        continue;
                    }
                    let q = p + Vec3::new(x as f32 * 0.5, 0., z as f32 * 0.5);
                    if self.walkable(q) {
                        return Vec3::new(q.x, self.ground_height(q, 0.3).unwrap_or(0.), q.z);
                    }
                }
            }
        }
        crate::layout::PLAYER
    }
    /// Ground route on a 0.5 m grid, shared by the replay and ground navigation.
    pub fn route(&self, start: Vec3, end: Vec3) -> Vec<Vec3> {
        const W: usize = 160;
        const H: usize = 120;
        let index = |p: Vec3| {
            ((p.z + 30.)
                .mul_add(2., 0.)
                .round()
                .clamp(0., (H - 1) as f32) as usize)
                * W
                + ((p.x + 40.) * 2.).round().clamp(0., (W - 1) as f32) as usize
        };
        let point =
            |i: usize| Vec3::new((i % W) as f32 * 0.5 - 40., 0., (i / W) as f32 * 0.5 - 30.);
        let a = index(start);
        let b = index(self.repair_spawn(end));
        let mut parent = vec![usize::MAX; W * H];
        let mut queue = VecDeque::from([a]);
        parent[a] = a;
        while let Some(i) = queue.pop_front() {
            if i == b {
                break;
            }
            for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let x = (i % W) as i32 + dx;
                let z = (i / W) as i32 + dz;
                if x < 0 || x >= W as i32 || z < 0 || z >= H as i32 {
                    continue;
                }
                let j = z as usize * W + x as usize;
                if parent[j] == usize::MAX && self.walkable(point(j)) {
                    parent[j] = i;
                    queue.push_back(j);
                }
            }
        }
        if parent[b] == usize::MAX {
            return vec![];
        }
        let mut result = vec![];
        let mut i = b;
        while i != a {
            let mut p = point(i);
            p.y = self.ground_height(p, 0.3).unwrap_or(0.);
            result.push(p);
            i = parent[i];
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotated_collision_flags_and_ground_queries() {
        let mut port = Port::default();
        port.add_box(
            Vec3::new(0., 1., 0.),
            Vec3::new(4., 2., 1.),
            std::f32::consts::FRAC_PI_2,
            CollisionFlags::SOLID,
            "steel",
        );
        assert!(!port.clear(Vec3::new(0., 0., 1.5), 1.8, 0.32, true));
        assert!(port.clear(Vec3::new(1.5, 0., 0.), 1.8, 0.32, true));
        assert_eq!(port.ground_height(Vec3::ZERO, 2.01), Some(2.));
        port.add_box(
            Vec3::new(5., 1., 0.),
            Vec3::splat(2.),
            0.,
            CollisionFlags::PLAYER_ONLY,
            "barrier",
        );
        assert!(!port.clear(Vec3::new(5., 0., 0.), 1.8, 0.32, true));
        assert!(port.clear(Vec3::new(5., 0., 0.), 1.8, 0.32, false));
        assert!(port.visible(Vec3::new(3., 1., 0.), Vec3::new(7., 1., 0.)));
        port.add_box(
            Vec3::new(5., 1., 0.),
            Vec3::splat(1.),
            0.,
            CollisionFlags::BULLET_ONLY,
            "glass",
        );
        assert!(!port.visible(Vec3::new(3., 1., 0.), Vec3::new(7., 1., 0.)));
    }
}
