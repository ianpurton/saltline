use crate::*;
pub fn build(c: &mut Commands, k: &Kit, p: Vec3) -> Entity {
    let e = k.root(c, p);
    k.part(c, e, Vec3::new(0., 0.8, 0.), Vec3::new(0.9, 1.6, 0.6), 4);
    k.part(
        c,
        e,
        Vec3::new(0., 1.25, 0.32),
        Vec3::new(0.65, 0.4, 0.04),
        6,
    );
    k.part(c, e, Vec3::new(0.3, 2., 0.), Vec3::new(0.035, 2., 0.035), 0);
    let indicator = k.part(c, e, Vec3::new(0., 1.8, 0.), Vec3::new(0.4, 0.1, 0.4), 7);
    indicator
}
