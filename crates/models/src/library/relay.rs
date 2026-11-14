use crate::*;
pub fn build(c: &mut Commands, k: &Kit, p: Vec3) -> Entity {
    let root = k.root(c, p);
    let cabinet = super::map_props::build(
        c,
        k,
        root,
        super::map_props::PropKind::RelayNode,
        Default::default(),
    );
    c.entity(cabinet).insert(Name::new("uplink relay"));
    k.label(
        c,
        root,
        "UPLINK",
        Transform::from_xyz(0., 2.1, 0.415).with_scale(Vec3::splat(0.18)),
    );
    k.part(
        c,
        root,
        Vec3::new(0., 2.46, 0.),
        Vec3::new(0.4, 0.1, 0.4),
        7,
    )
}
