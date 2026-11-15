//! Backdrop assets at the original berth anchors; collision remains world-owned.
use crate::{CollisionFlags, Port};
use bevy::prelude::*;
use saltline_models::{
    Kit,
    static_props::{SceneryKind, StaticModelPath},
};
pub fn build(c: &mut Commands, k: &Kit, root: Entity, port: &mut Port) {
    let crane = k.root(c, Vec3::ZERO);
    c.entity(crane).insert((
        Name::new("quay gantry"),
        ChildOf(root),
        StaticModelPath(SceneryKind::Crane.asset_path()),
    ));
    // Keep the established wheel and upright colliders independent of GLB geometry.
    for x in [18.5, 28.5] {
        for z in [-5.4, 5.4] {
            port.add_box(
                Vec3::new(x, 0.6, z),
                Vec3::new(0.8, 1.2, 3.),
                0.,
                CollisionFlags::SOLID,
                "metal",
            );
            port.add_box(
                Vec3::new(x, 11., z),
                Vec3::new(0.44, 20., 0.44),
                0.,
                CollisionFlags::SOLID,
                "metal",
            );
        }
    }
    // Exported ship geometry is localized to this existing world anchor.
    let ship = k.root(c, Vec3::new(41., 0., -9.));
    c.entity(ship).insert((
        Name::new("KITTIWAKE"),
        ChildOf(root),
        StaticModelPath(SceneryKind::Ship.asset_path()),
    ));
    port.add_box(
        Vec3::new(41., 5.3, -40.5),
        Vec3::new(16., 19.4, 63.),
        0.,
        CollisionFlags::SOLID,
        "metal",
    );
    // The reference boat points along local +X, then turns -90 degrees at this berth.
    let boat = k.root(c, Vec3::new(31.5, -1.6, 21.));
    c.entity(boat).insert((
        Name::new("extraction boat"),
        ChildOf(root),
        StaticModelPath(SceneryKind::Boat.asset_path()),
        Transform::from_xyz(31.5, -1.6, 21.)
            .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
    ));
    let skyline = k.root(c, Vec3::ZERO);
    c.entity(skyline).insert((
        Name::new("harbor skyline"),
        ChildOf(root),
        StaticModelPath(SceneryKind::Skyline.asset_path()),
    ));
}
