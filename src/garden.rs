use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

pub struct GardenPlugin;

impl Plugin for GardenPlugin {
    fn build(&self, app: &mut App) { 
        app.add_systems(Startup, spawn_first_garden);
    }
}

#[derive(Component)]
pub struct Garden;

#[derive(Bundle)]
pub struct GardenBundle {
    pub collider: Collider,
    pub materialmesh: MaterialMesh2dBundle<ColorMaterial>,    
}

fn spawn_first_garden(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
            Collider::cuboid(50.0, 5.0),
            Garden,
        ))
        .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 10.0))),
            material: materials.add(Color::rgb(0.0, 0.5, 0.0)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(150.0, -20.0, 0.0)));
}