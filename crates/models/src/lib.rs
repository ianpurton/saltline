use bevy::prelude::*;
/// Shared primitive cache: every procedural object uses these meshes and materials.
pub mod geometry;
pub mod library;
pub mod materials;
pub struct SurfacePalette {
    pub asphalt: Handle<StandardMaterial>,
    pub concrete: Handle<StandardMaterial>,
    pub steel: Handle<StandardMaterial>,
    pub graphite: Handle<StandardMaterial>,
    pub water: Handle<StandardMaterial>,
}
pub struct Kit {
    pub surfaces: SurfacePalette,
    pub ship_hull: [Handle<Mesh>; 2],
    pub hard: geometry::HardSurface,
    pub labels: std::collections::HashMap<&'static str, Handle<Mesh>>,
    pub optic_glass: Handle<StandardMaterial>,
    pub stencil_paint: Handle<StandardMaterial>,
    pub industrial: Vec<Handle<StandardMaterial>>,
    pub cube: Handle<Mesh>,
    pub ball: Handle<Mesh>,
    pub sea: Handle<StandardMaterial>,
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
        let mut mats = materials::legacy(a, images);
        for (_, rgb) in library::map_props::PAINTS {
            let mut paint = a.get(&mats[1]).cloned().unwrap_or_default();
            // Catalogue paints are solid pigments, not the legacy hazard-stripe texture.
            paint.base_color_texture = None;
            paint.base_color = Color::srgb(rgb[0], rgb[1], rgb[2]);
            paint.perceptual_roughness = 0.52;
            mats.push(a.add(paint));
        }
        let mut sea_material = a.get(&mats[8]).cloned().unwrap_or_default();
        // Ocean UVs cover kilometres; puddle UVs cover metres.
        sea_material.uv_transform = bevy::math::Affine2::from_scale(Vec2::splat(480.));
        let sea = a.add(sea_material);
        let hard = geometry::HardSurface::new(m);
        let industrial = materials::industrial::create(a);
        // Painted robot armor shares paint wear; rubber and optics retain their own response.
        for index in [0usize, 1, 2, 4] {
            let source = a.get(&mats[if index == 0 { 4 } else { 5 }]).cloned();
            if let (Some(source), Some(mut target)) = (source, a.get_mut(&industrial[index])) {
                target.base_color_texture = source.base_color_texture;
                target.normal_map_texture = source.normal_map_texture;
            }
        }
        Self {
            ship_hull: [
                m.add(library::ship::berth_hull(false)),
                m.add(library::ship::berth_hull(true)),
            ],
            surfaces: SurfacePalette {
                asphalt: mats[0].clone(),
                concrete: mats[9].clone(),
                steel: mats[4].clone(),
                graphite: mats[5].clone(),
                water: sea.clone(),
            },
            labels: ["SALTLINE", "B4-07", "H-03", "UPLINK", "PORT 04", "CARGO"]
                .into_iter()
                .map(|s| (s, m.add(geometry::stencil::mesh(s))))
                .collect(),
            optic_glass: a.add(StandardMaterial {
                base_color: Color::srgba(0.12, 0.27, 0.35, 0.16),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            stencil_paint: a.add(StandardMaterial {
                base_color: Color::srgb(0.77, 0.75, 0.62),
                perceptual_roughness: 0.72,
                ..default()
            }),
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
            sea,
            hard,
            industrial,
        }
    }
    pub fn label(
        &self,
        c: &mut Commands,
        parent: Entity,
        text: &'static str,
        transform: Transform,
    ) {
        if let Some(mesh) = self.labels.get(text) {
            geometry::assembly::component(
                c,
                parent,
                text,
                mesh.clone(),
                self.stencil_paint.clone(),
                transform,
            );
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
    /// Cached shaped parts retain the established material palette.
    pub fn shaped(&self, c: &mut Commands, parent: Entity, p: Vec3, s: Vec3, mat: usize) -> Entity {
        geometry::assembly::component(
            c,
            parent,
            "formed steel",
            self.hard.armor.clone(),
            self.mats[mat].clone(),
            Transform::from_translation(p).with_scale(s),
        )
    }
    pub fn rod(
        &self,
        c: &mut Commands,
        parent: Entity,
        a: Vec3,
        b: Vec3,
        diameter: f32,
        mat: usize,
    ) {
        let d = b - a;
        geometry::assembly::component(
            c,
            parent,
            "tube or cable",
            self.hard.rod.clone(),
            self.mats[mat].clone(),
            Transform::from_translation(a)
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, d.normalize()))
                .with_scale(Vec3::new(diameter, d.length(), diameter)),
        );
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

#[derive(Component)]
pub struct Rotor(pub f32);

#[derive(Component)]
pub struct ReloadMagazine;
