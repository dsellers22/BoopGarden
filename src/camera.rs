use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 40.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera2d);
    }
}

fn spawn_camera2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection { scale: 0.25, ..default()},
        ..default()
    });
}
