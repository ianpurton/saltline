//! Merge only stationary container children. Repeated variants share assembled meshes.
use bevy::prelude::*;
use std::collections::HashMap;
#[derive(Resource, Default)]
pub struct CoarseContainers(pub bool);
#[derive(Component)]
pub struct StaticContainer(pub usize);
#[derive(Component)]
pub struct ContainerLod {
    near: Vec<Entity>,
    far: Entity,
    detailed: bool,
}
pub fn update_lod(
    coarse: Option<Res<CoarseContainers>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut roots: Query<(&GlobalTransform, &mut ContainerLod)>,
    mut visibility: Query<&mut Visibility>,
) {
    let Some((_, camera)) = cameras.iter().find(|(c, _)| c.is_active) else {
        return;
    };
    for (transform, mut lod) in &mut roots {
        let distance = transform.translation().distance(camera.translation());
        let detailed = if coarse.as_ref().is_some_and(|c| c.0) {
            false
        } else if lod.detailed {
            distance < 65.
        } else {
            distance < 55.
        };
        if detailed == lod.detailed {
            continue;
        }
        lod.detailed = detailed;
        for &e in &lod.near {
            if let Ok(mut v) = visibility.get_mut(e) {
                *v = if detailed {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
        if let Ok(mut v) = visibility.get_mut(lod.far) {
            *v = if detailed {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
        }
    }
}
pub fn assemble(world: &mut World) {
    let roots: Vec<_> = world
        .query::<(Entity, &StaticContainer)>()
        .iter(world)
        .map(|(e, k)| (e, k.0))
        .collect();
    let mut cache: HashMap<usize, Vec<(Handle<Mesh>, Handle<StandardMaterial>)>> = HashMap::new();
    let mut coarse: HashMap<usize, (Handle<Mesh>, Handle<StandardMaterial>)> = HashMap::new();
    let mut before = 0;
    let mut after = 0;
    for (root, key) in roots {
        let children: Vec<_> = world
            .get::<Children>(root)
            .map(|v| v.iter().collect())
            .unwrap_or_default();
        before += children.len();
        if !cache.contains_key(&key) {
            if let Some(&body) = children.first() {
                let mesh = world
                    .get::<Mesh3d>(body)
                    .expect("container body mesh")
                    .0
                    .clone();
                let mat = world
                    .get::<MeshMaterial3d<StandardMaterial>>(body)
                    .expect("container body material")
                    .0
                    .clone();
                let transform = *world
                    .get::<Transform>(body)
                    .expect("container body transform");
                let mesh = world
                    .resource::<Assets<Mesh>>()
                    .get(&mesh)
                    .expect("cached body mesh")
                    .clone()
                    .transformed_by(transform);
                let mesh = world.resource_mut::<Assets<Mesh>>().add(mesh);
                coarse.insert(key, (mesh, mat));
            }
            let mut groups: HashMap<AssetId<StandardMaterial>, (Handle<StandardMaterial>, Mesh)> =
                HashMap::new();
            for &child in &children {
                let Some(mesh) = world.get::<Mesh3d>(child) else {
                    continue;
                };
                let Some(mat) = world.get::<MeshMaterial3d<StandardMaterial>>(child) else {
                    continue;
                };
                let Some(source) = world.resource::<Assets<Mesh>>().get(&mesh.0) else {
                    continue;
                };
                let mut source = source.clone().transformed_by(
                    *world
                        .get::<Transform>(child)
                        .unwrap_or(&Transform::IDENTITY),
                );
                // All assembled parts use the same vertex layout; tangents are regenerated once.
                source.remove_attribute(Mesh::ATTRIBUTE_TANGENT);
                if let Some((_, combined)) = groups.get_mut(&mat.0.id()) {
                    combined
                        .merge(&source)
                        .expect("container meshes have compatible vertex layouts");
                } else {
                    groups.insert(mat.0.id(), (mat.0.clone(), source));
                }
            }
            let assembled = groups
                .into_values()
                .map(|(mat, mut mesh)| {
                    let _ = mesh.generate_tangents();
                    (world.resource_mut::<Assets<Mesh>>().add(mesh), mat)
                })
                .collect();
            cache.insert(key, assembled);
        }
        for child in children {
            world.despawn(child);
        }
        let mut near = Vec::new();
        for (mesh, mat) in &cache[&key] {
            let child = world
                .spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(mat.clone()),
                    Transform::default(),
                ))
                .id();
            world.entity_mut(root).add_child(child);
            near.push(child);
            after += 1;
        }
        if let Some((mesh, mat)) = coarse.get(&key) {
            let far = world
                .spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(mat.clone()),
                    Transform::default(),
                    Visibility::Hidden,
                ))
                .id();
            world.entity_mut(root).add_child(far).insert(ContainerLod {
                near,
                far,
                detailed: true,
            });
        }
        world.entity_mut(root).remove::<StaticContainer>();
    }
    info!(
        "STATIC CONTAINERS mesh entities: {before} -> {after}; cached variants={}",
        cache.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_containers_share_assembled_meshes_and_keep_bounds() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>();
        let mesh = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Cuboid::default());
        let mat = app
            .world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial::default());
        let mut roots = Vec::new();
        for x in [0., 10.] {
            let root = app
                .world_mut()
                .spawn((Transform::from_xyz(x, 0., 0.), StaticContainer(1)))
                .id();
            for dx in [-1., 1.] {
                let child = app
                    .world_mut()
                    .spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(mat.clone()),
                        Transform::from_xyz(dx, 0., 0.),
                    ))
                    .id();
                app.world_mut().entity_mut(root).add_child(child);
            }
            roots.push(root);
        }
        assemble(app.world_mut());
        let near: Vec<_> = roots
            .iter()
            .map(|e| app.world().get::<ContainerLod>(*e).unwrap().near[0])
            .collect();
        let a = app.world().get::<Mesh3d>(near[0]).unwrap();
        let b = app.world().get::<Mesh3d>(near[1]).unwrap();
        assert_eq!(a.0.id(), b.0.id());
        assert_eq!(
            app.world()
                .resource::<Assets<Mesh>>()
                .get(&a.0)
                .unwrap()
                .count_vertices(),
            48
        );
        assert_eq!(
            app.world()
                .get::<Transform>(roots[1])
                .unwrap()
                .translation
                .x,
            10.
        );
    }
}

#[cfg(test)]
mod quality_tests {
    use super::*;
    #[test]
    fn forcing_coarse_and_restoring_distance_lod_is_reversible() {
        let mut app = App::new();
        app.world_mut()
            .spawn((Camera3d::default(), GlobalTransform::default()));
        let near = app.world_mut().spawn(Visibility::Inherited).id();
        let far = app.world_mut().spawn(Visibility::Hidden).id();
        app.world_mut().spawn((
            GlobalTransform::default(),
            ContainerLod {
                near: vec![near],
                far,
                detailed: true,
            },
        ));
        app.insert_resource(CoarseContainers(true))
            .add_systems(Update, update_lod);
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(near).unwrap(),
            Visibility::Hidden
        );
        assert_eq!(
            *app.world().get::<Visibility>(far).unwrap(),
            Visibility::Inherited
        );
        app.world_mut().resource_mut::<CoarseContainers>().0 = false;
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(near).unwrap(),
            Visibility::Inherited
        );
        assert_eq!(
            *app.world().get::<Visibility>(far).unwrap(),
            Visibility::Hidden
        );
    }
}
