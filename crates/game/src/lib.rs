use bevy::prelude::*;
use saltline_world::{EXIT, Port, RELAYS};
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Menu,
    Playing,
    Paused,
    Dead,
    Victory,
}
#[derive(Resource)]
pub struct Mission {
    pub mode: Mode,
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub health: f32,
    pub ammo: u32,
    pub reserve: u32,
    pub relays: [bool; 3],
    pub reload: f32,
    pub cooldown: f32,
    pub vertical: f32,
    pub elapsed: f32,
    pub hurt: f32,
    pub kills: u32,
    pub crouched: bool,
    pub last_safe_ground: Vec3,
    pub airborne: f32,
}
impl Default for Mission {
    fn default() -> Self {
        Self {
            mode: Mode::Menu,
            position: saltline_world::layout::PLAYER + Vec3::Y * 1.68,
            yaw: -std::f32::consts::FRAC_PI_2,
            pitch: 0.,
            health: 100.,
            ammo: 30,
            reserve: 210,
            relays: [false; 3],
            reload: 0.,
            cooldown: 0.,
            vertical: 0.,
            elapsed: 0.,
            hurt: 0.,
            kills: 0,
            crouched: false,
            last_safe_ground: saltline_world::layout::PLAYER,
            airborne: 0.,
        }
    }
}
#[derive(Resource, Default)]
pub struct Actions {
    pub movement: Vec2,
    pub sprint: bool,
    pub crouch: bool,
    pub jump: bool,
    pub fire: bool,
    pub reload: bool,
    pub interact: bool,
    pub aim: bool,
}
#[derive(Component)]
pub struct Enemy {
    pub drone: bool,
    pub health: f32,
    pub home: Vec3,
    pub cooldown: f32,
}
pub fn blocked(p: Vec3, port: &Port) -> bool {
    !port.clear(p, 1.8, 0.32, false)
}
pub fn visible(a: Vec3, b: Vec3, port: &Port) -> bool {
    port.visible(a, b)
}
impl Mission {
    pub fn eye_height(&self) -> f32 {
        if self.crouched { 1.05 } else { 1.68 }
    }
    pub fn feet(&self) -> Vec3 {
        self.position - Vec3::Y * self.eye_height()
    }
}
/// Confirmed simulation outcomes consumed by visual effects.
#[derive(Message, Clone, Copy, Debug)]
pub enum PresentationEvent {
    Shot { origin: Vec3, direction: Vec3 },
    Impact { position: Vec3, destroyed: bool },
    RelayCut { position: Vec3 },
}
pub fn simulate(
    time: Res<Time>,
    mut m: ResMut<Mission>,
    mut a: ResMut<Actions>,
    port: Res<Port>,
    mut presentation: MessageWriter<PresentationEvent>,
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
    performance: Option<Res<crate::performance::PerformanceSettings>>,
    diagnostics: Option<bevy::diagnostic::Diagnostics>,
) {
    let _trace = info_span!("lib::simulate").entered();
    let _cpu = crate::performance::CpuMeasurement::new(
        performance.as_deref(),
        diagnostics,
        "saltline/system/lib/simulate_ms",
    );
    if m.mode != Mode::Playing {
        return;
    }
    let dt = time.delta_secs();
    m.elapsed += dt;
    m.hurt = (m.hurt - dt).max(0.);
    m.cooldown = (m.cooldown - dt).max(0.);
    if m.reload > 0. {
        m.reload -= dt;
        if m.reload <= 0. {
            let n = (30 - m.ammo).min(m.reserve);
            m.ammo += n;
            m.reserve -= n;
        }
    }
    if a.reload && m.reload <= 0. && m.ammo < 30 && m.reserve > 0 {
        m.reload = 1.8;
    }
    let mut feet = m.feet();
    let crouch = a.crouch || (m.crouched && !port.clear(feet, 1.8, 0.32, true));
    m.crouched = crouch;
    a.crouch = crouch;
    let height = if crouch { 1.15 } else { 1.8 };
    let ground = port.ground_height(feet, feet.y + 0.02);
    let grounded = ground.is_some_and(|y| (feet.y - y).abs() < 0.04) && m.vertical <= 0.;
    let speed = if crouch {
        1.9
    } else if a.sprint {
        6.4
    } else {
        3.8
    };
    let displacement = Quat::from_rotation_y(m.yaw)
        * Vec3::new(a.movement.x, 0., -a.movement.y).clamp_length_max(1.)
        * speed
        * dt;
    feet = port.move_player(feet, displacement, height, grounded);
    if a.jump && grounded {
        m.vertical = (2_f32 * 21. * 1.).sqrt();
    }
    m.vertical = (m.vertical - 21. * dt).max(-40.);
    let dy = m.vertical * dt;
    let substeps = (dy.abs() / 0.1).ceil().max(1.) as usize;
    let mut landed = false;
    for _ in 0..substeps {
        let next = feet + Vec3::Y * (dy / substeps as f32);
        if dy <= 0. {
            if let Some(y) = port.ground_height(next, feet.y + 0.005) {
                if next.y <= y {
                    feet.y = y;
                    if m.airborne > 0.12 && m.vertical < -13. {
                        m.health -= 7. * (-m.vertical - 13.);
                        m.hurt = 0.3;
                    }
                    m.vertical = 0.;
                    landed = true;
                    break;
                }
            }
            feet = next;
        } else if port.clear(next, height, 0.32, true) {
            feet = next;
        } else {
            m.vertical = 0.;
            break;
        }
    }
    if landed {
        m.airborne = 0.;
        if feet.x < 28.8 {
            m.last_safe_ground = feet;
        }
    } else {
        m.airborne += dt;
    }
    if feet.y < saltline_world::layout::SEA_LEVEL - 2.2 {
        feet = m.last_safe_ground;
        m.vertical = 0.;
        m.airborne = 0.;
        m.yaw += std::f32::consts::PI;
        m.health -= 12.;
        m.hurt = 0.3;
    }
    m.position = feet + Vec3::Y * m.eye_height();
    let shot = a.fire && m.cooldown <= 0. && m.reload <= 0. && m.ammo > 0;
    if shot {
        m.ammo -= 1;
        m.cooldown = 0.11;
        let forward = Quat::from_euler(EulerRot::YXZ, m.yaw, m.pitch, 0.) * Vec3::NEG_Z;
        presentation.write(PresentationEvent::Shot {
            origin: m.position,
            direction: forward,
        });
        let target = enemies
            .iter()
            .enumerate()
            .filter(|(_, (e, t))| e.health > 0. && visible(m.position, t.translation, &port))
            .filter_map(|(i, (_, t))| {
                let delta = t.translation + Vec3::Y * 0.65 - m.position;
                let along = delta.dot(forward);
                (along > 0. && (delta - forward * along).length() < 0.8).then_some((i, along))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|x| x.0);
        if let Some(i) = target {
            if let Some((mut e, t)) = enemies.iter_mut().nth(i) {
                e.health -= 34.;
                presentation.write(PresentationEvent::Impact {
                    position: t.translation + Vec3::Y * 0.7,
                    destroyed: e.health <= 0.,
                });
                if e.health <= 0. {
                    m.kills += 1;
                }
            }
        }
    }
    for (mut e, mut t) in &mut enemies {
        if e.health <= 0. {
            t.rotation = t.rotation.slerp(Quat::from_rotation_z(1.5), dt * 4.);
            t.translation.y = (t.translation.y - dt * 2.).max(0.2);
            continue;
        }
        e.cooldown -= dt;
        let delta = m.position - t.translation;
        let dist = delta.length();
        let active = dist < 25. && visible(t.translation + Vec3::Y, m.position, &port);
        let target = if active {
            m.position
        } else {
            e.home
                + Vec3::new(
                    (m.elapsed * 0.4).sin() * 3.,
                    0.,
                    (m.elapsed * 0.4).cos() * 3.,
                )
        };
        let mut dir = target - t.translation;
        dir.y = 0.;
        dir = dir.normalize_or_zero();
        let next = t.translation + dir * dt * if e.drone { 2.4 } else { 3.1 };
        if port.clear(next, if e.drone { 0.5 } else { 1.2 }, 0.32, false)
            && (e.drone || port.walkable(next))
        {
            t.translation = next;
        } else {
            let side = Vec3::new(-dir.z, 0., dir.x) * dt * 3.;
            if port.clear(
                t.translation + side,
                if e.drone { 0.5 } else { 1.2 },
                0.32,
                false,
            ) && (e.drone || port.walkable(t.translation + side))
            {
                t.translation += side;
            }
        }
        if !e.drone {
            if let Some(y) = port.ground_height(t.translation, t.translation.y + 0.23) {
                t.translation.y = y;
            }
        }
        if dir.length() > 0. {
            t.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
        }
        if active && dist < if e.drone { 18. } else { 2.3 } && e.cooldown <= 0. {
            m.health -= if e.drone { 5. } else { 12. };
            m.hurt = 0.3;
            e.cooldown = if e.drone { 1.4 } else { 1. };
        }
    }
    if a.interact {
        for (i, p) in RELAYS.iter().enumerate() {
            if m.position.distance(*p + Vec3::Y * 1.3) < 3.
                && visible(
                    m.position,
                    *p + Vec3::Y * 1.3 + (m.position - *p).with_y(0.).normalize_or_zero() * 0.85,
                    &port,
                )
            {
                if !m.relays[i] {
                    presentation.write(PresentationEvent::RelayCut {
                        position: *p + Vec3::Y * 1.8,
                    });
                }
                m.relays[i] = true;
            }
        }
    }
    if m.relays.iter().all(|x| *x)
        && m.feet().xz().distance(EXIT.xz()) < 2.5
        && (m.feet().y - EXIT.y).abs() < 3.5
    {
        m.mode = Mode::Victory;
    }
    if m.health <= 0. {
        m.mode = Mode::Dead;
    }
    a.jump = false;
    a.interact = false;
    a.reload = false;
}

/// Command-line inspection/replay switches are native-only.
/// Browser WASM has no process argument API.
pub fn launch_args() -> impl Iterator<Item = String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::args()
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::iter::empty()
    }
}

pub mod performance;
