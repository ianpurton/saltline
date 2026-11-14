//! Native procedural map catalogue. Dimensions and silhouettes follow the construction manual.
use crate::*;
pub const PAINTS: &[(&str, [f32; 3])] = &[
    ("red", [0.48, 0.10, 0.07]),
    ("blue", [0.08, 0.22, 0.36]),
    ("green", [0.18, 0.30, 0.21]),
    ("yellow", [0.76, 0.59, 0.16]),
    ("orange", [0.69, 0.27, 0.08]),
    ("white", [0.78, 0.79, 0.73]),
    ("grey", [0.36, 0.41, 0.42]),
    ("teal", [0.09, 0.38, 0.38]),
    ("rust", [0.40, 0.19, 0.09]),
    ("black", [0.045, 0.055, 0.06]),
    ("olive", [0.27, 0.30, 0.16]),
];
pub fn paint_index(name: &str) -> usize {
    13 + PAINTS.iter().position(|(n, _)| *n == name).unwrap_or(6)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropKind {
    JerseyBarrier,
    LightMast,
    FencePanel,
    GuardBooth,
    GateArm,
    BollardMooring,
    BollardPost,
    RelayNode,
    ShelfRack,
    Catwalk,
    ReeferUnit,
    Barrel,
    Pallet,
    PalletStack,
    CrateWood,
    CrateStack,
    IbcTote,
    Forklift,
    CableReel,
    TireStack,
    Dumpster,
    Generator,
    JerryCan,
    TrafficCone,
    GasCylinders,
    TarpCargo,
    Sandbags,
    ToolboxCart,
    FloodlightStand,
    CardboardBoxes,
    PipeStack,
    SignPost,
    Stairs,
}
impl PropKind {
    pub fn size(self) -> Vec3 {
        match self {
            Self::JerseyBarrier => Vec3::new(3.0000, 0.8100, 0.6000),
            Self::LightMast => Vec3::new(0.5000, 14.0000, 0.5000),
            Self::FencePanel => Vec3::new(3.0000, 2.4000, 0.0800),
            Self::GuardBooth => Vec3::new(2.6000, 2.7000, 2.2000),
            Self::GateArm => Vec3::new(5.0000, 1.1000, 0.3000),
            Self::BollardMooring => Vec3::new(0.7000, 0.6000, 0.7000),
            Self::BollardPost => Vec3::new(0.1700, 1.0000, 0.1700),
            Self::RelayNode => Vec3::new(1.2000, 2.4000, 0.8000),
            Self::ShelfRack => Vec3::new(2.7000, 4.2000, 1.1000),
            Self::Catwalk => Vec3::new(3.0000, 1.1000, 1.2000),
            Self::ReeferUnit => Vec3::new(0.5000, 2.3000, 2.3000),
            Self::Barrel => Vec3::new(0.6000, 0.9000, 0.6000),
            Self::Pallet => Vec3::new(1.2000, 0.1440, 1.0000),
            Self::PalletStack => Vec3::new(1.2000, 1.1500, 1.0000),
            Self::CrateWood => Vec3::new(1.1000, 1.0000, 0.9000),
            Self::CrateStack => Vec3::new(1.2000, 1.9000, 1.0000),
            Self::IbcTote => Vec3::new(1.2000, 1.1600, 1.0000),
            Self::Forklift => Vec3::new(3.4000, 2.2000, 1.2500),
            Self::CableReel => Vec3::new(1.3000, 1.3000, 0.8000),
            Self::TireStack => Vec3::new(0.8000, 1.0000, 0.8000),
            Self::Dumpster => Vec3::new(1.9000, 1.3500, 1.1000),
            Self::Generator => Vec3::new(2.2000, 1.4000, 1.0000),
            Self::JerryCan => Vec3::new(0.3500, 0.4700, 0.1700),
            Self::TrafficCone => Vec3::new(0.3600, 0.7000, 0.3600),
            Self::GasCylinders => Vec3::new(0.9000, 1.6000, 0.5000),
            Self::TarpCargo => Vec3::new(2.4000, 1.5000, 1.4000),
            Self::Sandbags => Vec3::new(2.0000, 0.9000, 0.6000),
            Self::ToolboxCart => Vec3::new(0.9000, 0.9500, 0.5000),
            Self::FloodlightStand => Vec3::new(1.0000, 2.2000, 1.0000),
            Self::CardboardBoxes => Vec3::new(1.0000, 0.8000, 0.8000),
            Self::PipeStack => Vec3::new(6.0000, 0.9000, 1.2000),
            Self::SignPost => Vec3::new(0.6000, 2.4000, 0.0800),
            Self::Stairs => Vec3::new(3.0000, 2.6000, 1.0000),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct PropOptions {
    pub seed: i32,
    pub color: &'static str,
    pub forks_up: f32,
    pub posts: &'static str,
    pub sign: &'static str,
}
impl Default for PropOptions {
    fn default() -> Self {
        Self {
            seed: 7,
            color: "",
            forks_up: 0.,
            posts: "both",
            sign: "",
        }
    }
}
struct Parts<'a, 'w, 's> {
    k: &'a Kit,
    c: &'a mut Commands<'w, 's>,
    root: Entity,
}
impl Parts<'_, '_, '_> {
    fn b(&mut self, p: [f32; 3], s: [f32; 3], m: usize) {
        self.k
            .part(self.c, self.root, Vec3::from(p), Vec3::from(s), m);
    }
    fn rod(&mut self, a: [f32; 3], b: [f32; 3], r: f32, m: usize) {
        self.k
            .rod(self.c, self.root, Vec3::from(a), Vec3::from(b), r, m);
    }
    fn wheel(&mut self, p: [f32; 3], s: [f32; 3], rotation: Quat, m: usize) {
        geometry::assembly::component(
            self.c,
            self.root,
            "cylinder",
            self.k.hard.hub.clone(),
            self.k.mats[m].clone(),
            Transform::from_translation(Vec3::from(p))
                .with_scale(Vec3::from(s))
                .with_rotation(rotation),
        );
    }
    fn pallet(&mut self, y: f32) {
        for x in [-0.48, 0., 0.48] {
            self.b([x, y + 0.055, 0.], [0.14, 0.11, 0.96], 10);
        }
        for i in 0..5 {
            self.b(
                [0., y + 0.126, -0.4 + i as f32 * 0.2],
                [1.2, 0.036, 0.15],
                10,
            );
        }
    }
    fn crate_box(&mut self, p: [f32; 3], s: [f32; 3]) {
        self.b(p, s, 10);
        for side in [-1., 1.] {
            for x in [-0.4, 0.4] {
                self.b(
                    [p[0] + x * s[0], p[1], p[2] + side * (s[2] * 0.5 + 0.012)],
                    [0.075, s[1], 0.035],
                    3,
                );
            }
        }
    }
}
/// Returns a local-root model; placement and collision are owned by the world crate.
pub fn build(
    c: &mut Commands,
    k: &Kit,
    parent: Entity,
    kind: PropKind,
    options: PropOptions,
) -> Entity {
    let root = k.root(c, Vec3::ZERO);
    c.entity(parent).add_child(root);
    c.entity(root).insert(Name::new(format!("{kind:?}")));
    let mut p = Parts { k, c, root };
    let size = kind.size();
    let yellow = paint_index("yellow");
    let steel = 4;
    let black = 5;
    let color = paint_index(if options.color.is_empty() {
        "yellow"
    } else {
        options.color
    });
    use PropKind::*;
    match kind {
        JerseyBarrier => {
            p.b([0., 0.055, 0.], [3., 0.11, 0.6], 9);
            p.b([0., 0.22, 0.], [3., 0.22, 0.4], 9);
            p.b([0., 0.55, 0.], [3., 0.52, 0.18], 9);
            for x in [-1., 1.] {
                p.b([x, 0.72, 0.], [0.12, 0.04, 0.2], black);
            }
        }
        Barrel => {
            p.wheel(
                [0., 0.45, 0.],
                [0.6, 0.9, 0.6],
                Quat::IDENTITY,
                paint_index(if options.color.is_empty() {
                    "blue"
                } else {
                    options.color
                }),
            );
            for y in [0.1, 0.33, 0.67, 0.83] {
                p.wheel([0., y, 0.], [0.62, 0.025, 0.62], Quat::IDENTITY, steel);
            }
        }
        Pallet => p.pallet(0.),
        PalletStack => {
            for i in 0..8 {
                p.pallet(i as f32 * 0.144);
            }
        }
        CrateWood => p.crate_box([0., 0.5, 0.], [1.1, 1., 0.9]),
        CrateStack => {
            p.crate_box([0., 0.5, 0.], [1.2, 1., 1.]);
            p.crate_box([0.04, 1.45, 0.], [1.02, 0.9, 0.88]);
        }
        CardboardBoxes => {
            for (x, z, h) in [(-0.25, -0.18, 0.52), (0.24, 0.18, 0.8)] {
                p.b([x, h * 0.5, z], [0.48, h, 0.44], 10);
                p.b([x, h + 0.001, z], [0.06, 0.005, 0.44], 3);
            }
        }
        IbcTote => {
            p.pallet(0.);
            p.b([0., 0.64, 0.], [1.04, 0.98, 0.88], 11);
            for x in [-0.57, -0.3, 0., 0.3, 0.57] {
                for z in [-0.48, 0.48] {
                    p.rod([x, 0.16, z], [x, 1.13, z], 0.012, steel);
                }
            }
            for y in [0.2, 0.5, 0.8, 1.13] {
                for z in [-0.48, 0.48] {
                    p.rod([-0.57, y, z], [0.57, y, z], 0.012, steel);
                }
                for x in [-0.57, 0.57] {
                    p.rod([x, y, -0.48], [x, y, 0.48], 0.012, steel);
                }
            }
            p.b([0., 0.27, 0.51], [0.16, 0.12, 0.08], black);
        }
        Forklift => {
            p.b([-0.4, 0.5, 0.], [1.9, 0.5, 0.9], color);
            p.b([-1.08, 0.8, 0.], [0.55, 0.55, 0.95], color);
            for x in [-1.1, 0.33] {
                for z in [-0.49, 0.49] {
                    p.wheel(
                        [x, 0.3, z],
                        [0.58, 0.2, 0.58],
                        Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                        black,
                    );
                }
            }
            for z in [-0.42, 0.42] {
                for x in [-0.85, 0.55] {
                    p.b([x, 1.48, z], [0.055, 1.35, 0.055], steel);
                }
                p.b([0.85, 1.25, z * 0.6], [0.1, 1.9, 0.08], black);
            }
            p.b([-0.14, 2.15, 0.], [1.65, 0.1, 1.05], black);
            p.b([-0.35, 0.99, 0.], [0.4, 0.18, 0.52], black);
            p.b([-0.58, 1.23, 0.], [0.12, 0.4, 0.52], black);
            for z in [-0.29, 0.29] {
                p.b(
                    [1.4, 0.18 + options.forks_up * 1.5, z],
                    [1.2, 0.06, 0.1],
                    steel,
                );
            }
        }
        ShelfRack => {
            for x in [-1.28, 1.28] {
                for z in [-0.5, 0.5] {
                    p.b([x, 2.1, z], [0.065, 4.2, 0.065], paint_index("blue"));
                }
            }
            for y in [0.2, 1.45, 2.7, 4.05] {
                p.b([0., y, 0.], [2.7, 0.065, 1.1], paint_index("orange"));
                if y < 4. {
                    for x in [-0.85, 0., 0.85] {
                        p.crate_box([x, y + 0.38, 0.], [0.7, 0.7, 0.8]);
                    }
                }
            }
        }
        LightMast => {
            p.b([0., 0.2, 0.], [0.5, 0.4, 0.5], 9);
            p.rod([0., 0.4, 0.], [0., 13.8, 0.], 0.065, steel);
            p.b([0., 13.8, 0.], [2.5, 0.09, 0.09], steel);
            for x in [-0.95, -0.32, 0.32, 0.95] {
                p.b([x, 13.65, 0.], [0.5, 0.18, 0.38], black);
                p.b([x, 13.55, 0.04], [0.42, 0.035, 0.28], 7);
            }
            p.c.spawn((
                PointLight {
                    color: Color::srgb(1., 0.69, 0.38),
                    intensity: 18000.,
                    range: 20.,
                    shadow_maps_enabled: false,
                    ..default()
                },
                Transform::from_xyz(0., 13.4, 0.),
                ChildOf(root),
            ));
        }
        FencePanel => {
            // Thin cross wires retain sight lines; tall player barriers are separate colliders.
            for x in [-1.5, 1.5] {
                if (x < 0. && options.posts != "right") || (x > 0. && options.posts != "left") {
                    p.rod([x, 0., 0.], [x, 2.4, 0.], 0.035, steel);
                }
            }
            for y in [0.14, 2.3] {
                p.rod([-1.5, y, 0.], [1.5, y, 0.], 0.02, steel);
            }
            for i in -9..=9 {
                let x = i as f32 * 0.3;
                let lo = (-1.5_f32).max(x);
                let hi = 1.5_f32.min(x + 1.2);
                if lo < hi {
                    p.rod(
                        [lo, (lo - x) * 2., 0.],
                        [hi, (hi - x) * 2., 0.],
                        0.004,
                        steel,
                    );
                    p.rod(
                        [lo, 2.4 - (lo - x) * 2., 0.],
                        [hi, 2.4 - (hi - x) * 2., 0.],
                        0.004,
                        steel,
                    );
                }
            }
        }
        GuardBooth => {
            p.b([0., 0.18, 0.], [2.6, 0.36, 2.2], 9);
            p.b([0., 0.7, 0.], [2.5, 0.8, 2.1], 11);
            for x in [-1.2, 0., 1.2] {
                for z in [-1., 1.] {
                    p.b([x, 1.8, z], [0.07, 1.4, 0.07], steel);
                }
            }
            p.b([0., 2.55, 0.], [2.6, 0.3, 2.2], paint_index("blue"));
            for z in [-1.03, 1.03] {
                p.b([0., 1.8, z], [2.3, 1.1, 0.025], 12);
            }
        }
        GateArm => {
            p.b([-2., 0.55, 0.], [0.3, 1.1, 0.3], yellow);
            p.rod([-2., 0.95, 0.], [-1.6, 5.4, 0.], 0.045, 11);
            for i in 0..9 {
                p.b(
                    [-1.98 + i as f32 * 0.045, 1.1 + i as f32 * 0.49, 0.],
                    [0.1, 0.18, 0.1],
                    7,
                );
            }
        }
        BollardPost => {
            p.wheel([0., 0.5, 0.], [0.17, 1., 0.17], Quat::IDENTITY, yellow);
            p.b([0., 0.02, 0.], [0.26, 0.04, 0.26], steel);
        }
        BollardMooring => {
            p.b([0., 0.06, 0.], [0.7, 0.12, 0.7], black);
            for x in [-0.2, 0.2] {
                p.wheel([x, 0.3, 0.], [0.2, 0.45, 0.2], Quat::IDENTITY, black);
            }
            p.rod([-0.33, 0.52, 0.], [0.33, 0.52, 0.], 0.09, black);
        }
        CableReel => {
            for z in [-0.34, 0.34] {
                p.wheel(
                    [0., 0.65, z],
                    [1.3, 0.08, 1.3],
                    Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                    10,
                );
            }
            p.wheel(
                [0., 0.65, 0.],
                [0.85, 0.6, 0.85],
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                black,
            );
        }
        TireStack => {
            for i in 0..5 {
                let mesh = p.k.ring.clone();
                geometry::assembly::component(
                    p.c,
                    root,
                    "tire",
                    mesh,
                    p.k.mats[black].clone(),
                    Transform::from_xyz(0., 0.1 + i as f32 * 0.2, 0.)
                        .with_scale(Vec3::new(1.1, 1.25, 1.1)),
                );
            }
        }
        Dumpster => {
            p.b([0., 0.65, 0.], [1.9, 1.2, 1.1], paint_index("green"));
            for x in [-0.47, 0.47] {
                p.b([x, 1.28, 0.], [0.92, 0.14, 1.13], black);
            }
            for x in [-0.7, 0.7] {
                for z in [-0.4, 0.4] {
                    p.wheel(
                        [x, 0.1, z],
                        [0.18, 0.1, 0.18],
                        Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                        black,
                    );
                }
            }
        }
        Generator => {
            p.b([0., 0.12, 0.], [2.2, 0.24, 1.], black);
            p.b([0., 0.8, 0.], [2.1, 1.2, 0.94], color);
            for i in 0..10 {
                p.b(
                    [-0.8 + i as f32 * 0.16, 0.85, 0.48],
                    [0.06, 0.6, 0.02],
                    black,
                );
            }
            p.b([0.6, 1.5, 0.], [0.08, 0.4, 0.08], black);
        }
        JerryCan => {
            p.b([0., 0.21, 0.], [0.35, 0.42, 0.17], paint_index("green"));
            p.b([0., 0.45, 0.], [0.18, 0.04, 0.07], black);
            for x in [-0.12, 0.12] {
                p.rod([x, 0.08, 0.09], [-x, 0.34, 0.09], 0.008, steel);
            }
        }
        TrafficCone => {
            p.b([0., 0.03, 0.], [0.36, 0.06, 0.36], black);
            geometry::assembly::component(
                p.c,
                root,
                "cone",
                p.k.hard.taper.clone(),
                p.k.mats[7].clone(),
                Transform::from_xyz(0., 0.37, 0.).with_scale(Vec3::new(0.27, 0.66, 0.27)),
            );
            p.wheel([0., 0.42, 0.], [0.15, 0.09, 0.15], Quat::IDENTITY, 11);
        }
        GasCylinders => {
            for x in [-0.3, 0., 0.3] {
                p.wheel(
                    [x, 0.72, 0.],
                    [0.28, 1.42, 0.28],
                    Quat::IDENTITY,
                    paint_index("blue"),
                );
                p.b([x, 1.51, 0.], [0.06, 0.18, 0.06], steel);
            }
            p.b([0., 0.04, 0.], [0.9, 0.08, 0.5], 10);
        }
        TarpCargo => {
            p.b([0., 0.1, 0.], [2.4, 0.2, 1.4], 10);
            p.k.shaped(
                p.c,
                root,
                Vec3::new(0., 0.85, 0.),
                Vec3::new(2.35, 1.3, 1.35),
                paint_index("olive"),
            );
            for x in [-0.7, 0.7] {
                p.b([x, 1.51, 0.], [0.045, 0.018, 1.36], black);
            }
        }
        Sandbags => {
            for y in 0..3 {
                for x in 0..4 {
                    p.k.shaped(
                        p.c,
                        root,
                        Vec3::new(
                            -0.75 + x as f32 * 0.5 + (y % 2) as f32 * 0.05,
                            0.15 + y as f32 * 0.3,
                            0.,
                        ),
                        Vec3::new(0.5, 0.3, 0.6),
                        paint_index("olive"),
                    );
                }
            }
        }
        ToolboxCart => {
            p.b([0., 0.52, 0.], [0.9, 0.8, 0.5], paint_index("red"));
            for y in [0.25, 0.4, 0.55, 0.7, 0.85] {
                p.b([0., y, 0.26], [0.7, 0.02, 0.02], steel);
            }
            for x in [-0.35, 0.35] {
                for z in [-0.18, 0.18] {
                    p.wheel(
                        [x, 0.09, z],
                        [0.16, 0.06, 0.16],
                        Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                        black,
                    );
                }
            }
        }
        FloodlightStand => {
            for z in [-0.4, 0.4] {
                p.rod([-0.4, 0., z], [0., 1.2, 0.], 0.025, steel);
                p.rod([0.4, 0., z], [0., 1.2, 0.], 0.025, steel);
            }
            p.rod([0., 0.5, 0.], [0., 2., 0.], 0.025, steel);
            for x in [-0.27, 0.27] {
                p.b([x, 2., 0.], [0.48, 0.4, 0.12], yellow);
                p.b([x, 2., 0.07], [0.4, 0.3, 0.02], 7);
            }
        }
        PipeStack => {
            for y in 0..3 {
                for z in 0..(3 - y) {
                    p.wheel(
                        [
                            0.,
                            0.15 + y as f32 * 0.3,
                            -0.4 + z as f32 * 0.4 + y as f32 * 0.2,
                        ],
                        [0.3, 6., 0.3],
                        Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                        paint_index("rust"),
                    );
                }
            }
        }
        SignPost => {
            p.rod([0., 0., 0.], [0., 2.35, 0.], 0.025, steel);
            p.b(
                [0., 2.05, 0.],
                [0.6, 0.6, 0.04],
                if options.sign == "no_entry" {
                    paint_index("red")
                } else {
                    yellow
                },
            );
            p.b([0., 2.05, 0.025], [0.4, 0.08, 0.015], black);
        }
        ReeferUnit => {
            p.b([0., 1.15, 0.], [0.5, 2.3, 2.3], 11);
            for y in 0..12 {
                p.b([0.26, 0.2 + y as f32 * 0.16, 0.], [0.025, 0.055, 2.], black);
            }
        }
        RelayNode => {
            p.b([0., 1.2, 0.], [1.2, 2.4, 0.8], steel);
            p.b([0., 1.7, 0.41], [0.8, 0.5, 0.025], 6);
            p.rod([0., 2.4, 0.], [0., 3., 0.], 0.02, black);
        }
        Stairs => {
            for i in 0..12 {
                let y = (i + 1) as f32 * 0.2;
                let x = -1.5 + i as f32 * 0.2;
                p.b([x + 0.125, y - 0.015, 0.], [0.25, 0.03, 0.9], steel);
                p.b([x + 0.016, y - 0.015, 0.], [0.035, 0.034, 0.9], yellow);
            }
            p.b([1.2, 2.5825, 0.], [0.64, 0.035, 0.96], steel);
            for z in [-0.465, 0.465] {
                p.rod([-1.43, 0.13, z], [0.94, 2.5, z], 0.06, black);
                p.rod([-1.4, 1.1, z], [1.45, 3.6, z], 0.025, yellow);
                for i in 0..5 {
                    let x = -1.4 + i as f32 * 0.7;
                    let y = 0.2 + (x + 1.4) * 0.84;
                    p.rod([x, y, z], [x, y + 1., z], 0.025, steel);
                }
            }
        }
        Catwalk => {
            p.b([0., 0.025, 0.], [3., 0.05, 1.05], steel);
            for z in [-0.565, 0.565] {
                p.b([0., -0.04, z], [3., 0.18, 0.06], black);
                for x in [-1.45, -0.5, 0.5, 1.45] {
                    p.rod([x, 0.05, z], [x, 1.05, z], 0.025, steel);
                }
                for y in [0.55, 1.05] {
                    p.rod([-1.48, y, z], [1.48, y, z], 0.025, yellow);
                }
            }
        }
    }
    let _ = size;
    root
}
