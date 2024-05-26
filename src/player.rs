use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
use crate::schedule::InGameSet;

const PLAYER_SPEED: f32 = 30.0;
const PLAYER_JUMP: f32 = 5000.0;
const PLAYER_RESTITUTION: f32 = 0.5;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement_controls, player_jump_controls).in_set(InGameSet::UserInput));
    }
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    /* Create the player. */
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
        .insert(Restitution::coefficient(PLAYER_RESTITUTION))
        .insert(ColliderMassProperties::Mass(10.0))
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        });
}

fn player_movement_controls(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };
    
    if keyboard_input.pressed(KeyCode::KeyD) {
        transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }
}

fn player_jump_controls(
    mut query: Query<&mut ExternalImpulse, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut external_impulse) = query.get_single_mut() else {
        println!("Player body not found!");
        return;
    };
    
    if keyboard_input.just_pressed(KeyCode::Space) {
        external_impulse.impulse = Vec2::new(0.0, PLAYER_JUMP);
    }
}

 