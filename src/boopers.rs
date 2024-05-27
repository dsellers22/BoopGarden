use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use bevy_rapier2d::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct Booper;

#[derive(Resource)]
pub struct BoopTimer {
    timer: Timer,
}

pub struct BooperPlugin;

impl Plugin for BooperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_booper);
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
            ..default()
            })
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
    .insert(Collider::ball(5.0))
    .insert(Restitution::coefficient(1.0));
}

 fn boop(
     mut query: Query<&mut ExternalImpulse, With<Booper>>,
     mut player_query: Query<&Transform, With<Player>>,
     mut boop_timer: ResMut<BoopTimer>,
     time: Res<Time>,
 ) {
     boop_timer.timer.tick(time.delta());
     if !boop_timer.timer.just_finished() {
         return;
     }
     
     let Ok(mut player_transform) = player_query.get_single_mut() else {
         return;
     };
     
     
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
        external_impulse.impulse = Vec2::new(0.0, crate::player::PLAYER_JUMP);
    }
}



