use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_environment);
    }
}

fn spawn_environment(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -75.0, 0.0)));
}