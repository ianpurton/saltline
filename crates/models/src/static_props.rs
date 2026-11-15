//! Shared asynchronous scenes for the static prop catalogue.
use crate::library::map_props::PropKind;
use bevy::{prelude::*, world_serialization::WorldInstanceReady};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

#[derive(Component)]
pub struct StaticProp(pub PropKind);
#[derive(Component)]
struct Instantiated;
#[derive(Resource, Default)]
struct Scenes(HashMap<PropKind, Handle<WorldAsset>>);

/// Shared with the render world so capture warmup starts after scene instantiation.
#[derive(Resource, Clone, Default)]
pub struct StaticPropsReady(Arc<AtomicBool>);
impl StaticPropsReady {
    pub fn ready(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

pub struct StaticPropsPlugin;
impl Plugin for StaticPropsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scenes>()
            .init_resource::<StaticPropsReady>()
            .add_observer(instantiated)
            .add_systems(Update, (attach, readiness).chain());
    }
}
fn instantiated(
    event: On<WorldInstanceReady>,
    props: Query<(), With<StaticProp>>,
    mut c: Commands,
) {
    if props.contains(event.entity) {
        c.entity(event.entity).insert(Instantiated);
    }
}
fn attach(
    mut c: Commands,
    server: Res<AssetServer>,
    mut scenes: ResMut<Scenes>,
    props: Query<(Entity, &StaticProp), Without<WorldAssetRoot>>,
) {
    for (entity, prop) in &props {
        let handle = scenes
            .0
            .entry(prop.0)
            .or_insert_with(|| server.load(prop.0.asset_path()));
        c.entity(entity).insert(WorldAssetRoot(handle.clone()));
    }
}
fn readiness(
    server: Res<AssetServer>,
    props: Query<(&StaticProp, Option<&WorldAssetRoot>, Has<Instantiated>)>,
    ready: Res<StaticPropsReady>,
) {
    let mut complete = true;
    for (prop, scene, instantiated) in &props {
        let Some(scene) = scene else {
            complete = false;
            continue;
        };
        if let Some(error) =
            server
                .get_load_states(scene.0.id())
                .and_then(|(root, direct, recursive)| {
                    use bevy::asset::{
                        DependencyLoadState, LoadState, RecursiveDependencyLoadState,
                    };
                    match (root, direct, recursive) {
                        (LoadState::Failed(e), _, _)
                        | (_, DependencyLoadState::Failed(e), _)
                        | (_, _, RecursiveDependencyLoadState::Failed(e)) => Some(e),
                        _ => None,
                    }
                })
        {
            panic!(
                "Static prop {} failed to load: {error}",
                prop.0.asset_path()
            );
        }
        complete &= instantiated && server.is_loaded_with_dependencies(&scene.0);
    }
    ready.0.store(complete, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_loads_instances_and_reuses_assets() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            AssetPlugin {
                file_path: format!("{}/../../assets", env!("CARGO_MANIFEST_DIR")),
                ..default()
            },
            bevy::transform::TransformPlugin,
            bevy::world_serialization::WorldSerializationPlugin,
            bevy::image::ImagePlugin::default(),
            bevy::mesh::MeshPlugin,
            bevy::gltf::GltfPlugin::default(),
            StaticPropsPlugin,
        ))
        .init_asset::<StandardMaterial>();
        app.finish();
        app.cleanup();
        let parent = app
            .world_mut()
            .spawn((Transform::from_xyz(10., 0., 2.), Visibility::default()))
            .id();
        let roots: Vec<_> = PropKind::ALL
            .into_iter()
            .map(|kind| {
                app.world_mut()
                    .spawn((
                        StaticProp(kind),
                        Transform::IDENTITY,
                        Visibility::default(),
                        ChildOf(parent),
                    ))
                    .id()
            })
            .collect();
        let repeated = app
            .world_mut()
            .spawn((
                StaticProp(PropKind::Barrel),
                Transform::IDENTITY,
                Visibility::default(),
            ))
            .id();
        assert!(!app.world().resource::<StaticPropsReady>().ready());
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        loop {
            app.update();
            if app.world().resource::<StaticPropsReady>().ready() {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "catalogue scene loading timed out"
            );
            std::thread::yield_now();
        }
        for root in roots {
            assert_eq!(app.world().get::<ChildOf>(root).unwrap().parent(), parent);
            assert_eq!(
                *app.world().get::<Transform>(root).unwrap(),
                Transform::IDENTITY
            );
            let descendants: Vec<_> = app.world().get::<Children>(root).unwrap().iter().collect();
            assert!(!descendants.is_empty());
        }
        let original = app
            .world_mut()
            .query::<(&StaticProp, &WorldAssetRoot)>()
            .iter(app.world())
            .find(|(p, _)| p.0 == PropKind::Barrel)
            .unwrap()
            .1
            .0
            .clone();
        assert_eq!(
            original,
            app.world().get::<WorldAssetRoot>(repeated).unwrap().0
        );
        assert_eq!(
            app.world().resource::<Scenes>().0.len(),
            PropKind::ALL.len()
        );
        assert!(app.world_mut().query::<&Mesh3d>().iter(app.world()).count() > PropKind::ALL.len());
    }
}
