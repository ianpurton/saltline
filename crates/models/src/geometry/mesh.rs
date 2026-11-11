//! Face-local vertices preserve hard edges and UV seams.
use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology,
};
#[derive(Default)]
pub struct Builder {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}
impl Builder {
    pub fn triangle(&mut self, p: [Vec3; 3], uv: [Vec2; 3]) {
        let n = (p[1] - p[0]).cross(p[2] - p[0]);
        if n.length_squared() < 1e-14 {
            return;
        }
        let base = self.positions.len() as u32;
        for i in 0..3 {
            self.positions.push(p[i].to_array());
            self.normals.push(n.normalize().to_array());
            self.uvs.push(uv[i].to_array());
        }
        self.indices.extend([base, base + 1, base + 2]);
    }
    pub fn quad(&mut self, p: [Vec3; 4]) {
        self.triangle([p[0], p[1], p[2]], [Vec2::ZERO, Vec2::X, Vec2::ONE]);
        self.triangle([p[0], p[2], p[3]], [Vec2::ZERO, Vec2::ONE, Vec2::Y]);
    }
    pub fn finish(self) -> Mesh {
        let mut m = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        m.insert_indices(Indices::U32(self.indices));
        m.generate_tangents()
            .expect("procedural mesh UVs must support tangents");
        m
    }
}
