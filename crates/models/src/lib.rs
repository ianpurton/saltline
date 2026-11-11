use bevy::prelude::*;
/// Shared primitive cache: every procedural object uses these meshes and materials.
pub mod geometry;
pub mod library;
pub mod materials;
pub struct Kit {
    pub hard: geometry::HardSurface,
    pub industrial: Vec<Handle<StandardMaterial>>,
    pub cube: Handle<Mesh>,
    pub ball: Handle<Mesh>,
    pub puddle: Handle<Mesh>,
    pub ring: Handle<Mesh>,
    pub mats: Vec<Handle<StandardMaterial>>,
}
impl Kit {
    pub fn new(
        m: &mut Assets<Mesh>,
        a: &mut Assets<StandardMaterial>,
        images: &mut Assets<Image>,
    ) -> Self {
        let mats = materials::legacy(a, images);
        let hard = geometry::HardSurface::new(m);
        let industrial = materials::industrial::create(a);
        Self {
            cube: m.add({
                let mut mesh = Mesh::from(Cuboid::default());
                let _ = mesh.generate_tangents();
                mesh
            }),
            ring: m.add(Torus::new(0.28, 0.36)),
            puddle: m.add({
                let mut mesh = Mesh::from(Circle::new(0.5).mesh().resolution(40));
                if let Some(bevy::mesh::VertexAttributeValues::Float32x3(positions)) =
                    mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
                {
                    for p in positions {
                        let angle = p[1].atan2(p[0]);
                        let r = 1. + 0.16 * (angle * 5.).sin() + 0.09 * (angle * 9.).cos();
                        p[0] *= r;
                        p[1] *= r;
                    }
                }
                let _ = mesh.generate_tangents();
                mesh
            }),
            ball: m.add(Sphere::new(0.5).mesh().uv(12, 8)),
            mats,
            hard,
            industrial,
        }
    }
    pub fn part(&self, c: &mut Commands, parent: Entity, p: Vec3, s: Vec3, mat: usize) -> Entity {
        let e = c
            .spawn((
                Mesh3d(self.cube.clone()),
                MeshMaterial3d(self.mats[mat].clone()),
                Transform::from_translation(p).with_scale(s),
            ))
            .id();
        c.entity(parent).add_child(e);
        e
    }
    pub fn joint(&self, c: &mut Commands, parent: Entity, p: Vec3, r: f32) -> Entity {
        let e = c
            .spawn((
                Mesh3d(self.ball.clone()),
                MeshMaterial3d(self.mats[4].clone()),
                Transform::from_translation(p).with_scale(Vec3::splat(r)),
            ))
            .id();
        c.entity(parent).add_child(e);
        e
    }
    pub fn root(&self, c: &mut Commands, p: Vec3) -> Entity {
        c.spawn((Transform::from_translation(p), Visibility::default()))
            .id()
    }
}
#[derive(Component)]
pub struct Leg {
    pub phase: f32,
}
#[derive(Component)]
pub struct Knee {
    pub phase: f32,
}
