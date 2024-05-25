mod camera;
mod asset_loader;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use asset_loader::AssetLoaderPlugin;
use asset_loader::SceneAssets;

#[derive(Bundle)]
pub struct RigidBodyBundle {
    pub rigid_body: RigidBody,
    pub model: SceneBundle,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_systems(PostStartup, setup_physics)
        .run();
}

fn setup_physics(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -75.0, 0.0)));

    /* Create the bouncing ball. */
    commands.spawn(
        RigidBodyBundle {
            rigid_body: RigidBody::Dynamic,
            model: SceneBundle {
                scene: scene_assets.booper.clone(),
                ..default()}
        })
        .insert(Collider::ball(2.50))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}