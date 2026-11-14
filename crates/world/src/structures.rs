use crate::{CollisionFlags as Flags, Port, layout};
use bevy::prelude::*;
use saltline_models::{
    Kit,
    library::map_props::{self, PropKind, PropOptions},
};
fn block(
    c: &mut Commands,
    k: &Kit,
    root: Entity,
    port: &mut Port,
    p: Vec3,
    size: Vec3,
    mat: usize,
) {
    k.part(c, root, p, size, mat);
    port.add_box(
        p,
        size,
        0.,
        Flags::SOLID,
        if mat == 9 { "concrete" } else { "metal" },
    );
}
pub fn shell(c: &mut Commands, k: &Kit, root: Entity, port: &mut Port) {
    let ground = k.part(
        c,
        root,
        Vec3::new(-5., -0.25, 0.),
        Vec3::new(70., 0.5, 60.),
        0,
    );
    c.entity(ground)
        .insert(MeshMaterial3d(k.surfaces.asphalt.clone()));
    #[cfg(not(target_arch = "wasm32"))]
    c.entity(ground)
        .insert(saltline_models::materials::wet::WetSurface(false));
    let sea = k.part(
        c,
        root,
        Vec3::new(500., layout::SEA_LEVEL - 0.05, 0.),
        Vec3::new(1200., 0.1, 1200.),
        8,
    );
    c.entity(sea)
        .insert(MeshMaterial3d(k.surfaces.water.clone()));
    #[cfg(not(target_arch = "wasm32"))]
    c.entity(sea)
        .insert(saltline_models::materials::wet::WetSurface(true));
    // Warehouse shell; dimensions and openings follow sheets 02 and 21.
    block(
        c,
        k,
        root,
        port,
        Vec3::new(-2., 0.005, -19.5),
        Vec3::new(28., 0.05, 17.),
        9,
    );
    // West / north walls, split south and east openings, and lintels.
    let walls = [
        (Vec3::new(-16., 3.5, -19.5), Vec3::new(0.25, 7., 17.)),
        (Vec3::new(-2., 3.5, -28.), Vec3::new(28., 7., 0.25)),
        (Vec3::new(-11., 3.5, -11.), Vec3::new(10., 7., 0.25)),
        (Vec3::new(5.5, 3.5, -11.), Vec3::new(13., 7., 0.25)),
        (Vec3::new(-3.5, 6., -11.), Vec3::new(5., 2., 0.25)),
        (Vec3::new(12., 3.5, -25.), Vec3::new(0.25, 7., 6.)),
        (Vec3::new(12., 3.5, -14.), Vec3::new(0.25, 7., 6.)),
        (Vec3::new(12., 5.75, -19.5), Vec3::new(0.25, 2.5, 5.)),
    ];
    for (p, size) in walls {
        block(c, k, root, port, p, size, 4);
        if size.y == 7. {
            let base = Vec3::new(p.x, 0.6, p.z);
            k.part(c, root, base, Vec3::new(size.x, 1.2, size.z), 9);
        }
        let along_x = size.x > size.z;
        let length = if along_x { size.x } else { size.z };
        for i in 0..(length / 0.24) as usize {
            let offset = -length * 0.5 + 0.12 + i as f32 * 0.24;
            let rib = p + if along_x {
                Vec3::new(offset, 0., 0.)
            } else {
                Vec3::new(0., 0., offset)
            };
            k.part(
                c,
                root,
                rib,
                if along_x {
                    Vec3::new(0.045, size.y, 0.28)
                } else {
                    Vec3::new(0.28, size.y, 0.045)
                },
                4,
            );
        }
    }
    for z in [-23.75, -15.25] {
        let roof = k.part(
            c,
            root,
            Vec3::new(-2., 8.25, z),
            Vec3::new(28.5, 0.12, 8.86),
            4,
        );
        c.entity(roof).insert(
            Transform::from_xyz(-2., 8.25, z)
                .with_rotation(Quat::from_rotation_x(if z < -19.5 {
                    -0.2861
                } else {
                    0.2861
                }))
                .with_scale(Vec3::new(28.5, 0.12, 8.86)),
        );
        for x in [-14., -7., 0., 7., 11.] {
            k.rod(
                c,
                root,
                Vec3::new(x, 7., if z < -19.5 { -28. } else { -11. }),
                Vec3::new(x, 9.5, -19.5),
                0.08,
                4,
            );
        }
    }
    for x in [-16., 12.] {
        saltline_models::geometry::assembly::component(
            c,
            root,
            "warehouse gable",
            k.hard.gable.clone(),
            k.mats[4].clone(),
            Transform::from_xyz(x, 7., -19.5)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::new(17., 2.5, 0.25)),
        );
    }
    port.add_box(
        Vec3::new(-2., 8.25, -19.5),
        Vec3::new(28., 2.5, 17.),
        0.,
        Flags::SOLID,
        "metal",
    );
    // Physical fence height and anti-climb volumes have different purposes.
    for z in [-30., 28.] {
        port.add_box(
            Vec3::new(-4., 1.2, z),
            Vec3::new(68., 2.4, 0.08),
            0.,
            Flags::SOLID,
            "metal",
        );
        port.add_box(
            Vec3::new(-4., 3.5, z),
            Vec3::new(68., 7., 0.08),
            0.,
            Flags::PLAYER_ONLY,
            "barrier",
        );
    }
    for (z, len) in [(-14., 32.), (18., 20.)] {
        port.add_box(
            Vec3::new(-38., 1.2, z),
            Vec3::new(0.08, 2.4, len),
            0.,
            Flags::SOLID,
            "metal",
        );
        port.add_box(
            Vec3::new(-38., 3.5, z),
            Vec3::new(0.08, 7., len),
            0.,
            Flags::PLAYER_ONLY,
            "barrier",
        );
    }
    port.add_box(
        Vec3::new(-38., 3.5, 5.),
        Vec3::new(0.08, 7., 6.),
        0.,
        Flags::PLAYER_ONLY,
        "barrier",
    );
    let gate = map_props::build(c, k, root, PropKind::FencePanel, PropOptions::default());
    c.entity(gate).insert(
        Transform::from_xyz(-37.72, 0., 11.3)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(2., 1., 1.)),
    );
    // Quay lip and deep sea wall; the upper deck remains at ground level.
    block(
        c,
        k,
        root,
        port,
        Vec3::new(30., -0.336, -1.),
        Vec3::new(0.65, 0.728, 60.),
        9,
    );
    block(
        c,
        k,
        root,
        port,
        Vec3::new(30.22, -2.45, -1.),
        Vec3::new(0.3, 3.5, 60.),
        9,
    );
    k.part(
        c,
        root,
        Vec3::new(29.7, 0.032, -1.),
        Vec3::new(0.15, 0.008, 60.),
        3,
    );
    for x in [18.5, 28.5] {
        block(
            c,
            k,
            root,
            port,
            Vec3::new(x, 0.017, -1.),
            Vec3::new(0.07, 0.034, 57.2),
            4,
        );
    }
    for z in [-22., -12., -2., 9., 16.4, 25.6] {
        let bollard =
            map_props::build(c, k, root, PropKind::BollardMooring, PropOptions::default());
        c.entity(bollard).insert(Transform::from_xyz(29.2, 0., z));
        port.add_box(
            Vec3::new(29.2, 0.3, z),
            Vec3::new(0.7, 0.6, 0.7),
            0.,
            Flags::SOLID,
            "metal",
        );
    }
    // Ladder is a visual water-access detail, extraction remains ground-side.
    for z in [19.74, 20.26] {
        k.rod(
            c,
            root,
            Vec3::new(29.88, -3.9, z),
            Vec3::new(29.88, 0.3, z),
            0.025,
            4,
        );
    }
    for i in 0..14 {
        let y = -3.9 + i as f32 * 0.3;
        k.rod(
            c,
            root,
            Vec3::new(29.88, y, 19.74),
            Vec3::new(29.88, y, 20.26),
            0.022,
            4,
        );
    }
    for z in [17.4, 19.1, 20.9, 22.6] {
        let post = map_props::build(c, k, root, PropKind::BollardPost, PropOptions::default());
        c.entity(post).insert(Transform::from_xyz(29.18, 0., z));
        port.add_box(
            Vec3::new(29.18, 0.5, z),
            Vec3::new(0.17, 1., 0.17),
            0.,
            Flags::SOLID,
            "metal",
        );
    }
    for (a, b) in [(17.4, 19.1), (20.9, 22.6)] {
        k.rod(
            c,
            root,
            Vec3::new(29.18, 0.7, a),
            Vec3::new(29.18, 0.7, b),
            0.012,
            3,
        );
    }
    for z in (-26..28).step_by(5) {
        saltline_models::geometry::assembly::component(
            c,
            root,
            "quay tire fender",
            k.ring.clone(),
            k.mats[5].clone(),
            Transform::from_xyz(30.43, -0.9, z as f32)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::splat(1.6)),
        );
    }
    for x in [-9., -3., 3., 9.] {
        k.part(
            c,
            root,
            Vec3::new(x, 0.035, -15.),
            Vec3::new(0.08, 0.008, 6.),
            3,
        );
    }
    for x in [-30., -24., -18., -12., -6., 0., 6., 12.] {
        k.part(
            c,
            root,
            Vec3::new(x, 0.009, 6.),
            Vec3::new(2., 0.012, 0.08),
            3,
        );
    }
}
