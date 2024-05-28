use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
use crate::player::Player;
use crate::garden::{Garden, GardenBundle};
use crate::tree::{Tree, TreeBundle};

const BOOP_FREQUENCY: f32 = 3.0;
const BOOP_POWER: f32 = 3500.0;
const BOOP_LIFT: f32 = 20000.0;

#[derive(Component)]
pub struct Booper;

#[derive(Resource)]
pub struct BoopTimer {
    timer: Timer,
}

pub struct BooperPlugin;

impl Plugin for BooperPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoopTimer {
            timer: Timer::from_seconds(BOOP_FREQUENCY, TimerMode::Repeating),
        })
            .add_systems(Startup, spawn_booper)
            .add_systems(Update, (boop, detect_player_contacts, spawn_tree));
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
    .insert(Restitution::coefficient(1.0))
    .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
    .insert(Damping { linear_damping: 0.0, angular_damping: 0.8})
        .insert(ActiveEvents::COLLISION_EVENTS);
}

 fn boop(
     mut query: Query<(&mut ExternalImpulse, &Transform),  With<Booper>>,
     mut player_query: Query<&Transform, With<Player>>,
     mut boop_timer: ResMut<BoopTimer>,
     time: Res<Time>,
 ) {
     boop_timer.timer.tick(time.delta());
     if !boop_timer.timer.just_finished() {
         return;
     }
     
     let Ok(player_transform) = player_query.get_single_mut() else {
         return;
     };
     
     for (mut external_impulse, booper_transform) in query.iter_mut() {
         let separation_vector: Vec3 = player_transform.translation - booper_transform.translation;
         let separation_vector_normal: Vec3 = separation_vector.normalize();
         external_impulse.impulse = Vec2::new(separation_vector_normal.x * BOOP_POWER, separation_vector_normal.y * BOOP_POWER);
         external_impulse.impulse += Vec2::new(0.0, BOOP_LIFT);
     }
 }


fn detect_player_contacts(
    rapier_context: Res<RapierContext>,
    mut query: Query<Entity, With<Booper>>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let player_entity: Entity = player_query.get_single_mut().unwrap();

    for booper_entity in query.iter_mut() {
        if let Some(contact_pair) = rapier_context.contact_pair(booper_entity, player_entity) {
            if contact_pair.has_any_active_contacts() {
                println!("Player Contact Detected!");
            }
        }
    }
}

// NOT FINAL

fn spawn_tree(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut boop_query: Query<(Entity, &Transform), With<Booper>>,
    mut garden_query: Query<Entity, With<Garden>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (booper_entity, transform) in boop_query.iter_mut() {
        
        for garden_entity in garden_query.iter_mut() {
        
            if let Some(contact_pair) = rapier_context.contact_pair(booper_entity, garden_entity) {
        
                if contact_pair.has_any_active_contacts() {
        
                    println!("Garden Contact Detected!");
        
                    let spawn_translation: Vec3 = Vec3::new(transform.translation.x, transform.translation.y + 20.0, transform.translation.z - 1.0);
        
                    commands.entity(booper_entity).despawn_recursive();
        
                    commands.spawn( (
                        TreeBundle {
                            root_collider: Collider::cuboid(2.5, 25.0),
                            material_mesh: MaterialMesh2dBundle {
                                mesh: Mesh2dHandle(meshes.add(Rectangle::new(5.0, 50.0))),
                                material: materials.add(Color::rgb(0.0, 0.0, 0.0)),
                                transform: Transform::from_translation(spawn_translation),//FIX THIS
                                ..default()
                            },
                        },
                        Tree,
                    ));
                }
            }
        }

    }
}





