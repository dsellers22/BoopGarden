use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::asset_loader::SceneAssets;

#[derive(Component)]
pub struct Booper;

#[derive(Bundle)]
pub struct RigidBodyBundle {
    pub rigid_body: RigidBody,
    pub model: SceneBundle,
}

pub struct BooperPlugin;

impl Plugin for BooperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_booper);
    }
}

fn spawn_booper(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -75.0, 0.0)));

    /* Create the bouncing ball. */
    commands.spawn((
        RigidBodyBundle {
            rigid_body: RigidBody::Dynamic,
            model: SceneBundle {
                scene: scene_assets.booper.clone(),
                transform: Transform::from_xyz(0.0, 400.0, 0.0).with_scale(Vec3::splat(0.5)),
                ..default()}
        },
        Booper,
    ))
        .insert(Collider::ball(2.50))
        .insert(Restitution::coefficient(1.0));
}