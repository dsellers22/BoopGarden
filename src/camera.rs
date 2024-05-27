use bevy::prelude::*;
use crate::player::Player;

const CAMERA_DISTANCE: f32 = 80.0;

#[derive(Component)]
pub struct GameCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera2d)
            .add_systems(Update, update_camera2d);
    }
}

fn spawn_camera2d(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection { scale: 0.5, ..default()},
        ..default()
    },
    GameCamera,
    ));
}

fn update_camera2d(
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<GameCamera>)>,
) {
    
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };
    
    let Ok(player_transform) = player_query.get_single_mut() else {
        return;
    };
    
    camera_transform.translation = Vec3::new(player_transform.translation.x, player_transform.translation.y, CAMERA_DISTANCE);
}
