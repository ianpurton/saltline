use bevy::prelude::*;
pub fn pivot(c: &mut Commands, parent: Entity, name: &str, position: Vec3) -> Entity {
    let e = c
        .spawn((
            Name::new(name.to_owned()),
            Transform::from_translation(position),
            Visibility::default(),
        ))
        .id();
    c.entity(parent).add_child(e);
    e
}
pub fn component(
    c: &mut Commands,
    parent: Entity,
    name: &str,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
) -> Entity {
    let e = c
        .spawn((
            Name::new(name.to_owned()),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            transform,
        ))
        .id();
    c.entity(parent).add_child(e);
    e
}
