use bevy::{prelude::*, time::TimeUpdateStrategy};
use saltline_game::*;
use saltline_world::{CollisionFlags, EXIT, Port, RELAYS};
use std::time::Duration;
fn app() -> App {
    let mut a = App::new();
    a.add_plugins(MinimalPlugins)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1. / 60.,
        )))
        .add_message::<PresentationEvent>()
        .init_resource::<Mission>()
        .init_resource::<Actions>()
        .init_resource::<Port>()
        .add_systems(Update, simulate);
    {
        let mut m = a.world_mut().resource_mut::<Mission>();
        m.mode = Mode::Playing;
        m.position = Vec3::new(0., 1.68, 8.);
        m.last_safe_ground = Vec3::new(0., 0., 8.);
        m.yaw = 0.;
    }
    a.update();
    a
}
#[test]
fn movement_collision_and_pause() {
    let mut a = app();
    a.world_mut().resource_mut::<Actions>().movement = Vec2::Y;
    for _ in 0..60 {
        a.update();
    }
    assert!(a.world().resource::<Mission>().position.z < 5.);
    a.world_mut().resource_mut::<Mission>().mode = Mode::Paused;
    let p = a.world().resource::<Mission>().position;
    for _ in 0..20 {
        a.update();
    }
    assert_eq!(p, a.world().resource::<Mission>().position);
    assert!(blocked(Vec3::new(41., 0., 0.), &Port::default()));
}
#[test]
fn firing_damage_reload() {
    let mut a = app();
    a.world_mut().spawn((
        Enemy {
            drone: false,
            health: 100.,
            home: Vec3::ZERO,
            cooldown: 10.,
        },
        Transform::from_xyz(0., 1., 0.),
    ));
    a.world_mut().resource_mut::<Actions>().fire = true;
    for _ in 0..25 {
        a.update();
    }
    assert!(a.world().resource::<Mission>().kills >= 1);
    a.world_mut().resource_mut::<Actions>().fire = false;
    a.world_mut().resource_mut::<Actions>().reload = true;
    for _ in 0..120 {
        a.update();
    }
    assert_eq!(a.world().resource::<Mission>().ammo, 30);
}
#[test]
fn mission_driver() {
    let mut a = app();
    for target in RELAYS.into_iter().chain([EXIT]) {
        for _ in 0..3000 {
            let m = a.world().resource::<Mission>();
            let delta = target - Vec3::new(m.position.x, 0., m.position.z);
            if delta.length() < 1.5 {
                break;
            }
            a.world_mut().resource_mut::<Actions>().movement =
                Vec2::new(delta.x, -delta.z).normalize_or_zero();
            a.update();
        }
        a.world_mut().resource_mut::<Actions>().movement = Vec2::ZERO;
        a.world_mut().resource_mut::<Actions>().interact = true;
        a.update();
    }
    assert_eq!(a.world().resource::<Mission>().mode, Mode::Victory);
}
#[test]
fn detection_attack_and_death() {
    let mut a = app();
    a.world_mut().spawn((
        Enemy {
            drone: false,
            health: 100.,
            home: Vec3::new(0., 0., 7.),
            cooldown: 0.,
        },
        Transform::from_xyz(0., 0., 7.),
    ));
    for _ in 0..1500 {
        a.update();
    }
    assert_eq!(a.world().resource::<Mission>().mode, Mode::Dead);
}
#[test]
fn obstructed_relay() {
    let mut a = app();
    a.world_mut().resource_mut::<Mission>().position = RELAYS[0] + Vec3::new(0., 1.7, 2.);
    a.world_mut().resource_mut::<Port>().add_box(
        RELAYS[0] + Vec3::new(0., 1., 1.),
        Vec3::new(2., 2., 0.4),
        0.,
        CollisionFlags::SOLID,
        "wall",
    );
    a.world_mut().resource_mut::<Actions>().interact = true;
    a.update();
    assert!(!a.world().resource::<Mission>().relays[0]);
}

#[test]
fn map_traversal_water_rescue_and_crouch_clearance() {
    let mut a = app();
    {
        let mut m = a.world_mut().resource_mut::<Mission>();
        m.position = Vec3::new(32., -4., 20.);
        m.last_safe_ground = Vec3::new(27., 0., 20.);
        m.yaw = 0.;
    }
    a.update();
    let m = a.world().resource::<Mission>();
    assert_eq!(m.feet(), Vec3::new(27., 0., 20.));
    assert_eq!(m.health, 88.);
    assert_eq!(m.vertical, 0.);
    assert!((m.yaw - std::f32::consts::PI).abs() < 0.001);
    a.world_mut().resource_mut::<Port>().add_box(
        Vec3::new(27., 1.5, 20.),
        Vec3::new(2., 0.3, 2.),
        0.,
        CollisionFlags::SOLID,
        "ceiling",
    );
    {
        let mut m = a.world_mut().resource_mut::<Mission>();
        m.crouched = true;
        m.position.y = 1.05;
    }
    a.world_mut().resource_mut::<Actions>().crouch = false;
    a.update();
    assert!(a.world().resource::<Mission>().crouched);
    assert!((a.world().resource::<Mission>().feet().y).abs() < 0.001);
}
