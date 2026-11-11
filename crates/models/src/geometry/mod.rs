pub mod assembly;
pub mod bevel;
pub mod extrusion;
pub mod lathe;
pub mod mesh;
pub mod tapered;
pub mod tube;
use bevy::prelude::*;
/// Shared immutable mesh palette, allocated once per Kit, reused by all instances.
pub struct HardSurface {
    pub hull: Handle<Mesh>,
    pub armor: Handle<Mesh>,
    pub panel: Handle<Mesh>,
    pub limb: Handle<Mesh>,
    pub hub: Handle<Mesh>,
    pub rod: Handle<Mesh>,
    pub cable: Handle<Mesh>,
    pub taper: Handle<Mesh>,
}
impl HardSurface {
    pub fn new(m: &mut Assets<Mesh>) -> Self {
        Self {
            hull: m.add(extrusion::extrude(
                &[
                    Vec2::new(-0.5, -0.5),
                    Vec2::new(0.5, -0.5),
                    Vec2::new(0.5, 0.32),
                    Vec2::new(0.36, 0.44),
                    Vec2::new(0., 0.54),
                    Vec2::new(-0.36, 0.44),
                    Vec2::new(-0.5, 0.32),
                ],
                1.,
            )),

            armor: m.add(bevel::beveled_box(Vec3::ONE, 0.12, 3, true)),
            panel: m.add(extrusion::extrude(
                &[
                    Vec2::new(-0.5, -0.35),
                    Vec2::new(-0.35, -0.5),
                    Vec2::new(0.35, -0.5),
                    Vec2::new(0.5, -0.28),
                    Vec2::new(0.5, 0.3),
                    Vec2::new(0.3, 0.5),
                    Vec2::new(-0.4, 0.5),
                    Vec2::new(-0.5, 0.3),
                ],
                1.,
            )),
            limb: m.add(extrusion::extrude(
                &[
                    Vec2::new(-0.28, -0.5),
                    Vec2::new(0.22, -0.5),
                    Vec2::new(0.5, 0.28),
                    Vec2::new(0.28, 0.5),
                    Vec2::new(-0.3, 0.5),
                    Vec2::new(-0.5, 0.25),
                ],
                1.,
            )),
            hub: m.add(lathe::lathe(
                &[
                    Vec2::new(0., -0.5),
                    Vec2::new(0.4, -0.5),
                    Vec2::new(0.5, -0.35),
                    Vec2::new(0.5, 0.35),
                    Vec2::new(0.4, 0.5),
                    Vec2::new(0., 0.5),
                ],
                32,
            )),
            rod: m.add(tube::tube(&[Vec3::ZERO, Vec3::Y], 0.5, 16)),
            cable: m.add(tube::tube(
                &[
                    Vec3::ZERO,
                    Vec3::new(0.15, -0.2, 0.05),
                    Vec3::new(0.16, -0.45, 0.),
                    Vec3::new(0., -0.65, 0.),
                ],
                0.018,
                10,
            )),
            taper: m.add(tapered::tapered(Vec2::new(0.6, 0.7), Vec2::ONE, 1.)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::mesh::{Indices, VertexAttributeValues};
    fn valid(m: Mesh) {
        let Some(VertexAttributeValues::Float32x3(p)) = m.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("positions")
        };
        let Some(VertexAttributeValues::Float32x3(n)) = m.attribute(Mesh::ATTRIBUTE_NORMAL) else {
            panic!("normals")
        };
        let Some(VertexAttributeValues::Float32x2(uv)) = m.attribute(Mesh::ATTRIBUTE_UV_0) else {
            panic!("UVs")
        };
        assert_eq!(p.len(), n.len());
        assert_eq!(p.len(), uv.len());
        assert!(m.attribute(Mesh::ATTRIBUTE_TANGENT).is_some());
        for (p, n) in p.iter().zip(n) {
            assert!(Vec3::from(*p).is_finite());
            assert!((Vec3::from(*n).length() - 1.).abs() < 0.001);
        }
        let Some(Indices::U32(i)) = m.indices() else {
            panic!("indices")
        };
        assert_eq!(i.len() % 3, 0);
        for t in i.chunks_exact(3) {
            assert!(t.iter().all(|&i| (i as usize) < p.len()));
            let a = Vec3::from(p[t[0] as usize]);
            let b = Vec3::from(p[t[1] as usize]);
            let c = Vec3::from(p[t[2] as usize]);
            assert!((b - a).cross(c - a).length_squared() > 1e-14);
        }
    }
    #[test]
    fn all_generators_have_valid_triangles_and_attributes() {
        valid(bevel::beveled_box(Vec3::new(1., 2., 3.), 0.15, 3, true));
        valid(bevel::beveled_box(Vec3::ONE, 0.1, 1, false));
        valid(extrusion::extrude(
            &[Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::splat(0.4), Vec2::Y],
            0.2,
        ));
        valid(lathe::lathe(
            &[
                Vec2::new(0., -1.),
                Vec2::new(1., -1.),
                Vec2::new(1., 1.),
                Vec2::new(0., 1.),
            ],
            24,
        ));
        valid(tapered::tapered(Vec2::ONE, Vec2::splat(0.5), 1.));
        valid(tube::tube(
            &[Vec3::ZERO, Vec3::Y, Vec3::new(0.3, 2., 0.2)],
            0.1,
            16,
        ));
    }
    #[test]
    fn hound_has_independent_joint_hierarchies_and_cached_meshes() {
        let mut world = World::new();
        let mut meshes = Assets::<Mesh>::default();
        let mut mats = Assets::<StandardMaterial>::default();
        let mut images = Assets::<Image>::default();
        let kit = crate::Kit::new(&mut meshes, &mut mats, &mut images);
        let count = meshes.len();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        {
            let mut c = Commands::new(&mut queue, &world);
            for _ in 0..2 {
                let root = kit.root(&mut c, Vec3::ZERO);
                kit.hound(&mut c, root);
            }
        }
        queue.apply(&mut world);
        assert_eq!(meshes.len(), count);
        assert_eq!(world.query::<&crate::Leg>().iter(&world).count(), 8);
        let mut q = world.query::<(&crate::Knee, &ChildOf)>();
        let parents: Vec<_> = q.iter(&world).map(|(_, p)| p.parent()).collect();
        assert_eq!(parents.len(), 8);
        for p in parents {
            assert!(world.get::<crate::Leg>(p).is_some());
        }
    }
}
