use bevy::{prelude::*, time::TimeUpdateStrategy};
use saltline_game::*;
use saltline_world::{EXIT, Port, RELAYS};
use std::time::Duration;
fn app() -> App {
    let mut a = App::new();
    a.add_plugins(MinimalPlugins)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1. / 60.,
        )))
        .init_resource::<Mission>()
        .init_resource::<Actions>()
        .init_resource::<Port>()
        .add_systems(Update, simulate);
    a.world_mut().resource_mut::<Mission>().mode = Mode::Playing;
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
    assert!(blocked(Vec3::new(25., 0., 0.), &Port::default()));
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
    a.world_mut()
        .resource_mut::<Port>()
        .walls
        .push((RELAYS[0] + Vec3::Z, Vec3::new(1., 2., 0.2)));
    a.world_mut().resource_mut::<Actions>().interact = true;
    a.update();
    assert!(!a.world().resource::<Mission>().relays[0]);
}
