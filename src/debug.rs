use bevy::prelude::*;
use bevy_rapier2d::pipeline::{CollisionEvent, ContactForceEvent};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_collision_events);
    }
}

fn display_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Collision: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Contact Force: {:?}", contact_force_event);
    }
}