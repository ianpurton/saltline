//! Backdrop anchors from the original berth assembly; no corridor-era dressing.
use crate::{CollisionFlags, Port};
use bevy::prelude::*;
use saltline_models::{
    Kit,
    library::map_props::{PropKind, PropOptions, build as prop, paint_index},
};
pub fn build(c: &mut Commands, k: &Kit, root: Entity, port: &mut Port) {
    // One rail-mounted gantry straddles the two quay rails.
    let crane = k.root(c, Vec3::ZERO);
    c.entity(crane)
        .insert((Name::new("quay gantry"), ChildOf(root)));
    for x in [18.5, 28.5] {
        for z in [-5.4, 5.4] {
            k.part(c, crane, Vec3::new(x, 0.6, z), Vec3::new(0.8, 1.2, 3.), 3);
            k.rod(
                c,
                crane,
                Vec3::new(x, 1., z),
                Vec3::new(x, 21.2, z),
                0.22,
                3,
            );
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
    for z in [-5.4, 5.4] {
        k.part(
            c,
            crane,
            Vec3::new(35., 22.8, z),
            Vec3::new(54., 1.8, 0.7),
            3,
        );
        for x in [18.5, 28.5] {
            k.rod(
                c,
                crane,
                Vec3::new(x, 21.2, -5.4),
                Vec3::new(x, 21.2, 5.4),
                0.22,
                3,
            );
        }
        k.rod(
            c,
            crane,
            Vec3::new(23.5, 31., z),
            Vec3::new(12., 23., z),
            0.035,
            5,
        );
        k.rod(
            c,
            crane,
            Vec3::new(23.5, 31., z),
            Vec3::new(62., 23., z),
            0.035,
            5,
        );
        k.rod(
            c,
            crane,
            Vec3::new(23.5, 23., z),
            Vec3::new(23.5, 31., z),
            0.13,
            3,
        );
        k.rod(
            c,
            crane,
            Vec3::new(41., 22.8, z),
            Vec3::new(41., 18.11, z),
            0.018,
            5,
        );
    }
    k.part(
        c,
        crane,
        Vec3::new(41., 20.7, 0.),
        Vec3::new(2.44, 0.3, 12.19),
        3,
    );
    k.part(
        c,
        crane,
        Vec3::new(41., 19.255, 0.),
        Vec3::new(2.438, 2.59, 12.19),
        paint_index("red"),
    );
    k.part(
        c,
        crane,
        Vec3::new(27.8, 21.2, 6.4),
        Vec3::new(2., 2.4, 2.),
        11,
    );
    k.part(
        c,
        crane,
        Vec3::new(27.8, 21.5, 7.42),
        Vec3::new(1.7, 1.1, 0.02),
        12,
    );
    // Freighter: original hull envelope X 33..49, Z -72..-9, deck about Y 13.
    let ship = k.root(c, Vec3::new(41., 0., -9.));
    c.entity(ship)
        .insert((Name::new("KITTIWAKE"), ChildOf(root)));
    for (mesh, mat) in k
        .ship_hull
        .iter()
        .zip([paint_index("red"), paint_index("blue")])
    {
        saltline_models::geometry::assembly::component(
            c,
            ship,
            "freighter hull",
            mesh.clone(),
            k.mats[mat].clone(),
            Transform::default(),
        );
    }
    k.part(
        c,
        ship,
        Vec3::new(0., 13.05, -34.),
        Vec3::new(14.5, 0.15, 49.),
        4,
    );
    for x in [-7.2, 7.2] {
        k.rod(
            c,
            ship,
            Vec3::new(x, 14., -18.),
            Vec3::new(x, 14., -57.),
            0.035,
            4,
        );
        for z in (-56..-18).step_by(3) {
            k.rod(
                c,
                ship,
                Vec3::new(x, 13., z as f32),
                Vec3::new(x, 14., z as f32),
                0.025,
                4,
            );
        }
    }
    for (y, w, d) in [(15., 14.4, 10.), (18., 12., 8.), (21.5, 10., 7.)] {
        k.part(c, ship, Vec3::new(0., y, -54.), Vec3::new(w, 3., d), 11);
        for x in [-4., -2., 0., 2., 4.] {
            k.part(
                c,
                ship,
                Vec3::new(x, y + 0.5, -54. + d * 0.5 + 0.01),
                Vec3::new(1.1, 0.65, 0.025),
                12,
            );
        }
    }
    k.part(
        c,
        ship,
        Vec3::new(1.5, 24., -56.),
        Vec3::new(2.2, 4., 2.8),
        7,
    );
    k.rod(
        c,
        ship,
        Vec3::new(0., 23., -51.),
        Vec3::new(0., 29., -51.),
        0.06,
        4,
    );
    port.add_box(
        Vec3::new(41., 5.3, -40.5),
        Vec3::new(16., 19.4, 63.),
        0.,
        CollisionFlags::SOLID,
        "metal",
    );
    // Rescue craft at the original sea-level anchor, bow turned toward the quay.
    let boat = k.root(c, Vec3::new(31.5, -1.6, 21.));
    c.entity(boat)
        .insert((Name::new("extraction boat"), ChildOf(root)));
    saltline_models::geometry::assembly::component(
        c,
        boat,
        "rescue hull",
        k.hard.hull.clone(),
        k.mats[7].clone(),
        Transform::from_xyz(0., 0.6, 0.)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(2.2, 7.5, 1.3)),
    );
    k.part(
        c,
        boat,
        Vec3::new(0., 1.15, 0.),
        Vec3::new(1.7, 0.15, 5.5),
        5,
    );
    k.part(
        c,
        boat,
        Vec3::new(0., 1.9, 0.8),
        Vec3::new(1.5, 1.4, 1.8),
        11,
    );
    k.part(
        c,
        boat,
        Vec3::new(0., 2.1, 1.72),
        Vec3::new(1.2, 0.6, 0.03),
        12,
    );
    for x in [-0.7, 0.7] {
        k.rod(
            c,
            boat,
            Vec3::new(x, 1.15, -2.8),
            Vec3::new(x, 1.8, -2.8),
            0.025,
            4,
        );
    }
    // Distant silhouettes sit outside the playable berth, never inside its routes.
    for i in 0..12 {
        let x = -90. + i as f32 * 11.;
        let z = -100. - (i % 3) as f32 * 12.;
        let h = 7. + (i % 4) as f32 * 3.;
        k.part(c, root, Vec3::new(x, h * 0.5, z), Vec3::new(8., h, 18.), 9);
    }
    for i in 0..5 {
        let pole = prop(c, k, root, PropKind::LightMast, PropOptions::default());
        c.entity(pole)
            .insert(Transform::from_xyz(-55. + i as f32 * 18., 0., -45.));
    }
}
