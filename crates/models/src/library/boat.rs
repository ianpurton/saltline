use crate::*;
pub fn build(c: &mut Commands, k: &Kit, _root: Entity, exit: Vec3) {
    let boat = k.root(c, exit + Vec3::new(4., -0.15, 0.));
    k.part(c, boat, Vec3::ZERO, Vec3::new(4., 0.9, 7.), 0);
    k.part(c, boat, Vec3::new(0., 0.7, 0.), Vec3::new(2.5, 0.5, 4.), 3);
    k.part(c, boat, Vec3::new(0., 1.4, 0.8), Vec3::new(1.8, 1., 1.3), 4);
}
