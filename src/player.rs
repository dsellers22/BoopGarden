use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    /* Create the booper. */
    commands.spawn((
        RigidBody::Dynamic,
        Player,
    ))
        .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(8.0, 8.0))),
            material: materials.add(Color::rgb(0.7, 0.0, 0.0)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(10.0, 100.0, 0.0)))
        .insert(Collider::cuboid(4.0, 4.0))
        .insert(Restitution::coefficient(1.0));
}