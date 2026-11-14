pub mod collision;
mod dressing;
pub mod layout;
mod structures;
use bevy::prelude::*;
pub use collision::{Collider, CollisionFlags, Port};
pub use layout::{EXIT, RELAYS};
use saltline_models::{
    Kit,
    library::{
        container::ContainerSpec,
        map_props::{self, PropKind},
    },
};
#[derive(Component)]
pub struct AuthoredProp(pub usize);
#[derive(Component)]
pub struct AuthoredTier {
    pub stack: usize,
    pub tier: usize,
}
/// Reproducible Mulberry32 stream; cosmetic RNG never changes collision topology.
struct Rng(u32);
impl Rng {
    fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_add(0x6d2b79f5);
        let mut t = self.0;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        ((t ^ (t >> 14)) as f64 / 4294967296.) as f32
    }
}
pub fn build(c: &mut Commands, k: &Kit) -> Port {
    let mut port = Port::default();
    let root = k.root(c, Vec3::ZERO);
    c.entity(root)
        .insert(Name::new("SALTLINE / original berth"));
    structures::shell(c, k, root, &mut port);
    let mut rng = Rng(1337);
    for (stack_index, stack) in layout::STACKS.iter().enumerate() {
        let mut y = 0.;
        for (tier_index, tier) in stack.tiers.iter().enumerate() {
            let x = stack.center.x
                + tier.offset.x
                + if tier_index > 0 {
                    (rng.next() * 2. - 1.) * 0.05 * stack.jitter
                } else {
                    0.
                };
            let z = stack.center.y
                + tier.offset.y
                + if tier_index > 0 {
                    (rng.next() * 2. - 1.) * 0.04 * stack.jitter
                } else {
                    0.
                };
            let yaw = stack.yaw
                + (rng.next() * 2. - 1.)
                    * 0.022
                    * stack.jitter
                    * if tier_index > 0 { 0.6 } else { 1. }
                + if tier.flip { std::f32::consts::PI } else { 0. };
            let entity = k.container_spec(
                c,
                Vec3::new(x, y, z),
                ContainerSpec {
                    length: tier.length,
                    height: tier.height,
                    paint: tier.paint,
                    reefer: tier.reefer,
                    doors: tier.doors,
                },
            );
            c.entity(entity).insert((
                Transform::from_xyz(x, y, z).with_rotation(Quat::from_rotation_y(yaw)),
                AuthoredTier {
                    stack: stack_index,
                    tier: tier_index,
                },
                ChildOf(root),
            ));
            port.add_box(
                Vec3::new(x, y + tier.height * 0.5, z),
                Vec3::new(tier.length, tier.height, 2.438),
                yaw,
                CollisionFlags::SOLID,
                "metal",
            );
            y += tier.height;
        }
    }
    for (index, p) in layout::PLACEMENTS.iter().enumerate() {
        let entity = map_props::build(c, k, root, p.kind, p.options);
        c.entity(entity).insert((
            Transform::from_translation(p.position).with_rotation(Quat::from_rotation_y(p.yaw)),
            AuthoredProp(index),
        ));
        if p.no_collision {
            continue;
        }
        let size = p.kind.size();
        let transform = |offset: Vec3| p.position + Quat::from_rotation_y(p.yaw) * offset;
        match p.kind {
            PropKind::Stairs => {
                for i in 0..12 {
                    let h = (i + 1) as f32 * 0.2;
                    port.add_box(
                        transform(Vec3::new(-1.5 + i as f32 * 0.2 + 0.1, h * 0.5, 0.)),
                        Vec3::new(0.2, h, 1.),
                        p.yaw,
                        CollisionFlags::SOLID,
                        "metal",
                    );
                }
                port.add_box(
                    transform(Vec3::new(1.2, 1.3, 0.)),
                    Vec3::new(0.64, 2.6, 0.96),
                    p.yaw,
                    CollisionFlags::SOLID,
                    "metal",
                );
            }
            PropKind::Catwalk => {
                port.add_box(
                    transform(Vec3::new(0., 0.025, 0.)),
                    Vec3::new(3., 0.05, 1.05),
                    p.yaw,
                    CollisionFlags::SOLID,
                    "metal",
                );
                for z in [-0.565, 0.565] {
                    port.add_box(
                        transform(Vec3::new(0., 0.55, z)),
                        Vec3::new(3., 1., 0.04),
                        p.yaw,
                        CollisionFlags::SOLID,
                        "metal",
                    );
                }
            }
            PropKind::TrafficCone | PropKind::SignPost | PropKind::GateArm => {
                let stem = if p.kind == PropKind::SignPost {
                    Vec3::new(0.06, 2.4, 0.06)
                } else if p.kind == PropKind::GateArm {
                    Vec3::new(0.3, 1.1, 0.3)
                } else {
                    size
                };
                let offset = if p.kind == PropKind::GateArm {
                    Vec3::new(-2., 0.55, 0.)
                } else {
                    Vec3::Y * stem.y * 0.5
                };
                port.add_box(
                    transform(offset),
                    stem,
                    p.yaw,
                    CollisionFlags::SOLID,
                    "metal",
                );
            }
            _ => port.add_box(
                p.position + Vec3::Y * size.y * 0.5,
                size,
                p.yaw,
                CollisionFlags::SOLID,
                "prop",
            ),
        }
    }
    for x in [7.6, 10.36] {
        for z in [11.86, 12.98] {
            k.rod(c, root, Vec3::new(x, 0., z), Vec3::new(x, 2.5, z), 0.035, 4);
        }
    }
    for position in RELAYS {
        port.add_box(
            position + Vec3::Y * 1.2,
            Vec3::new(1.2, 2.4, 0.8),
            0.,
            CollisionFlags::SOLID,
            "metal",
        );
    }
    dressing::build(c, k, root, &mut port);
    port
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (World, Port) {
        let mut meshes = Assets::<Mesh>::default();
        let mut materials = Assets::<StandardMaterial>::default();
        let mut images = Assets::<Image>::default();
        let kit = Kit::new(&mut meshes, &mut materials, &mut images);
        let mut world = World::new();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        let port = build(&mut Commands::new(&mut queue, &world), &kit);
        queue.apply(&mut world);
        (world, port)
    }
    #[test]
    fn authored_schedule_and_routes_match_manual() {
        assert_eq!(layout::STACKS.len(), 26);
        assert_eq!(
            layout::STACKS.iter().map(|s| s.tiers.len()).sum::<usize>(),
            47
        );
        assert_eq!(layout::PLACEMENTS.len(), 232);
        assert_eq!(layout::SPAWNS.len(), 42);
        assert_eq!(layout::COVER.len(), 55);
        let (mut world, port) = fixture();
        assert_eq!(world.query::<&AuthoredTier>().iter(&world).count(), 47);
        assert_eq!(world.query::<&AuthoredProp>().iter(&world).count(), 232);
        assert!(
            port.clear(layout::PLAYER, 1.8, 0.32, true),
            "deployment blocked"
        );
        let mut previous = layout::PLAYER;
        for target in RELAYS.into_iter().chain([EXIT]) {
            let route = port.route(previous, target);
            assert!(!route.is_empty(), "unreachable {target:?}");
            previous = *route.last().unwrap();
        }
        assert!(port.clear(Vec3::new(-3.5, 0.03, -11.), 1.8, 0.32, true));
        assert!(port.clear(Vec3::new(12., 0.03, -19.5), 1.8, 0.32, true));
        let mut feet = Vec3::new(-2.2, 0., 12.42);
        for _ in 0..190 {
            feet = port.move_player(feet, Vec3::new(0.09, 0., 0.), 1.8, true);
        }
        assert!(
            feet.x > 13. && feet.y > 2.5,
            "elevated route failed: {feet:?}"
        );
    }
}
