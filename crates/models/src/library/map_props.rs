//! Static GLB catalogue. Dimensions remain authoritative for world collision.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub const ALL: [Self; 33] = [
        Self::JerseyBarrier,
        Self::LightMast,
        Self::FencePanel,
        Self::GuardBooth,
        Self::GateArm,
        Self::BollardMooring,
        Self::BollardPost,
        Self::RelayNode,
        Self::ShelfRack,
        Self::Catwalk,
        Self::ReeferUnit,
        Self::Barrel,
        Self::Pallet,
        Self::PalletStack,
        Self::CrateWood,
        Self::CrateStack,
        Self::IbcTote,
        Self::Forklift,
        Self::CableReel,
        Self::TireStack,
        Self::Dumpster,
        Self::Generator,
        Self::JerryCan,
        Self::TrafficCone,
        Self::GasCylinders,
        Self::TarpCargo,
        Self::Sandbags,
        Self::ToolboxCart,
        Self::FloodlightStand,
        Self::CardboardBoxes,
        Self::PipeStack,
        Self::SignPost,
        Self::Stairs,
    ];
    pub fn asset_path(self) -> &'static str {
        match self {
            Self::JerseyBarrier => "models/jersey_barrier.glb#Scene0",
            Self::LightMast => "models/light_mast.glb#Scene0",
            Self::FencePanel => "models/fence_panel.glb#Scene0",
            Self::GuardBooth => "models/guard_booth.glb#Scene0",
            Self::GateArm => "models/gate_arm.glb#Scene0",
            Self::BollardMooring => "models/bollard_mooring.glb#Scene0",
            Self::BollardPost => "models/bollard_post.glb#Scene0",
            Self::RelayNode => "models/relay_node.glb#Scene0",
            Self::ShelfRack => "models/shelf_rack.glb#Scene0",
            Self::Catwalk => "models/catwalk.glb#Scene0",
            Self::ReeferUnit => "models/reefer_unit.glb#Scene0",
            Self::Barrel => "models/barrel.glb#Scene0",
            Self::Pallet => "models/pallet.glb#Scene0",
            Self::PalletStack => "models/pallet_stack.glb#Scene0",
            Self::CrateWood => "models/crate_wood.glb#Scene0",
            Self::CrateStack => "models/crate_stack.glb#Scene0",
            Self::IbcTote => "models/ibc_tote.glb#Scene0",
            Self::Forklift => "models/forklift.glb#Scene0",
            Self::CableReel => "models/cable_reel.glb#Scene0",
            Self::TireStack => "models/tire_stack.glb#Scene0",
            Self::Dumpster => "models/dumpster.glb#Scene0",
            Self::Generator => "models/generator.glb#Scene0",
            Self::JerryCan => "models/jerry_can.glb#Scene0",
            Self::TrafficCone => "models/traffic_cone.glb#Scene0",
            Self::GasCylinders => "models/gas_cylinders.glb#Scene0",
            Self::TarpCargo => "models/tarp_cargo.glb#Scene0",
            Self::Sandbags => "models/sandbags.glb#Scene0",
            Self::ToolboxCart => "models/toolbox_cart.glb#Scene0",
            Self::FloodlightStand => "models/floodlight_stand.glb#Scene0",
            Self::CardboardBoxes => "models/cardboard_boxes.glb#Scene0",
            Self::PipeStack => "models/pipe_stack.glb#Scene0",
            Self::SignPost => "models/sign_post.glb#Scene0",
            Self::Stairs => "models/stairs_metal.glb#Scene0",
        }
    }
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
/// Legacy authoring options; GLB visuals use fixed exported defaults.
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
/// Returns a local-root GLB model; placement and collision are owned by the world crate.
/// Requires `StaticPropsPlugin`. Options are retained for source compatibility;
/// visuals use the fixed exported catalogue appearance.
pub fn build(
    c: &mut Commands,
    k: &Kit,
    parent: Entity,
    kind: PropKind,
    _options: PropOptions,
) -> Entity {
    let root = k.root(c, Vec3::ZERO);
    c.entity(parent).add_child(root);
    c.entity(root).insert((
        Name::new(format!("{kind:?}")),
        crate::static_props::StaticProp(kind),
    ));
    root
}
