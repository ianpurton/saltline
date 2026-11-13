//! Deterministic exterior dressing; all substantial additions are outside the playable bounds.
use bevy::prelude::*;
use saltline_models::{Kit, geometry::assembly::component};
pub fn build(c: &mut Commands, k: &Kit, root: Entity) {
    // A terminal closes the far end of the road, with a readable crane silhouette.
    k.part(
        c,
        root,
        Vec3::new(-8., -0.5, -120.),
        Vec3::new(150., 1., 64.),
        9,
    );
    for i in 0..13 {
        let x = -70. + i as f32 * 10.;
        let h = 8. + ((i * 7) % 5) as f32 * 2.5;
        k.part(
            c,
            root,
            Vec3::new(x, h * 0.5, -146.),
            Vec3::new(8., h, 24.),
            if i % 3 == 0 { 1 } else { 4 },
        );
        for y in [3., 6.] {
            for dx in [-2.5, 0., 2.5] {
                k.part(
                    c,
                    root,
                    Vec3::new(x + dx, y, -133.9),
                    Vec3::new(1.1, 0.45, 0.03),
                    7,
                );
            }
        }
    }
    for x in [-23., 23.] {
        for dz in [-2., 2.] {
            k.part(
                c,
                root,
                Vec3::new(x, 14., -108. + dz),
                Vec3::new(1.5, 28., 1.5),
                3,
            );
        }
        for y in [7., 14., 21.] {
            k.rod(
                c,
                root,
                Vec3::new(x, y, -110.),
                Vec3::new(x, y + 6., -106.),
                0.2,
                0,
            );
            k.part(c, root, Vec3::new(x, y, -108.), Vec3::new(2.2, 0.4, 6.), 4);
        }
    }
    k.part(
        c,
        root,
        Vec3::new(0., 28., -108.),
        Vec3::new(51., 2.5, 5.),
        3,
    );
    k.part(
        c,
        root,
        Vec3::new(8., 25.7, -105.),
        Vec3::new(4., 2., 3.),
        4,
    );
    for x in [6., 10.] {
        k.rod(
            c,
            root,
            Vec3::new(x, 26., -108.),
            Vec3::new(x, 16., -108.),
            0.045,
            5,
        );
    }
    k.part(
        c,
        root,
        Vec3::new(8., 16., -108.),
        Vec3::new(5., 0.5, 3.),
        3,
    );
    // Varied cargo at the landward edge and beyond the extraction route.
    for i in 0..25 {
        let x = if i < 12 {
            -48. - (i % 3) as f32 * 3.3
        } else {
            -22. + (i % 8) as f32 * 6.
        };
        let z = if i < 12 {
            3. - (i / 3) as f32 * 23.
        } else {
            -97. - (i / 8) as f32 * 10.
        };
        for layer in 0..2 + i % 3 {
            k.container(c, Vec3::new(x, layer as f32 * 2.85, z), 1 + (i + layer) % 3);
        }
    }
    // Tank farm and stacks produce depth beyond the freighter.
    for i in 0..7 {
        let x = 90. + (i % 4) as f32 * 12.;
        let z = -135. - (i / 4) as f32 * 24.;
        component(
            c,
            root,
            "storage tank",
            k.hard.hub.clone(),
            k.mats[4].clone(),
            Transform::from_xyz(x, 6., z).with_scale(Vec3::new(10., 12., 10.)),
        );
        k.rod(c, root, Vec3::new(x, 12., z), Vec3::new(x, 13., z), 0.9, 0);
    }
    for x in [83., 99.] {
        k.rod(
            c,
            root,
            Vec3::new(x, 0., -175.),
            Vec3::new(x, 40., -175.),
            2.1,
            4,
        );
    }
    k.part(
        c,
        root,
        Vec3::new(110., -0.5, -160.),
        Vec3::new(100., 1., 110.),
        9,
    );
}
