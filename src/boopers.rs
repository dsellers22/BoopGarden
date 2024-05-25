use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use bevy_rapier2d::prelude::*;

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

fn spawn_booper(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    /* Create the booper. */
    commands.spawn((
        RigidBody::Dynamic,
        Booper,
    ))
    .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {radius: 5.0})),
            material: materials.add(Color::rgb(0.0, 0.0, 0.7)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
            })
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
    .insert(Collider::ball(5.0))
    .insert(Restitution::coefficient(1.0));
}


