mod camera;
mod boopers;
mod environment;
mod player;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use boopers::BooperPlugin;
use environment::EnvironmentPlugin;
use player::PlayerPlugin;

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
        .run();
}
