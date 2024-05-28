use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Startup, spawn_first_garden);
    }
}

#[derive(Component)]
pub struct Tree;

#[derive(Bundle)]
pub struct TreeBundle {
    pub root_collider: Collider,
    pub material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}
