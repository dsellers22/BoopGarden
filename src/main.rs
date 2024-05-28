mod camera;
mod boopers;
mod environment;
mod player;
mod schedule;
mod state;
mod debug;
mod garden;
mod tree;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use boopers::BooperPlugin;
use environment::EnvironmentPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;
use debug::DebugPlugin;
use garden::GardenPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(BooperPlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(GardenPlugin)
        .run();
}

