use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

const GROW_FREQUENCY: f32 = 5.0;
pub(crate) const TREE_THICKNESS: f32 = 5.0;
pub(crate) const TREE_HEIGHT: f32 = 50.0;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GrowTimer {
                timer: Timer::from_seconds(GROW_FREQUENCY, TimerMode::Repeating)
            })
            .add_systems(Update, grow_tree);
    }
}

#[derive(Resource)]
pub struct GrowTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Branch; // a component to tag an entity as a branch

#[derive(Component)]
pub struct Branches { // a component to go on a tree to track how many branches it has
    pub value: i32,   
}

impl Branches {
    pub fn new(value: i32) -> Self { Self { value } }
}

#[derive(Bundle)]
pub struct TreeBundle {
    pub root_collider: Collider,
    pub material_mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub branches: Branches,
}

impl Default for TreeBundle {
    fn default() -> Self {
        Self {
            root_collider: Collider::cuboid(2.5, 25.0),
            material_mesh: MaterialMesh2dBundle { ..default() },
            branches: Branches::new(0),
        }
    }
}


fn grow_tree(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Transform, &mut Branches), With<Tree>>,
    mut grow_timer: ResMut<GrowTimer>,
    time: Res<Time>,
){
    grow_timer.timer.tick(time.delta());
    if !grow_timer.timer.just_finished() {
        return;
    }
    
    for (tree_transform, mut tree_branches) in query.iter_mut() {
        if tree_branches.value == 0 {
            
            let spawn_translation: Vec3 = Vec3::new(tree_transform.translation.x - TREE_HEIGHT / 4.0, tree_transform.translation.y + TREE_HEIGHT / 2.0, tree_transform.translation.z);
            
            commands.spawn((
                Collider::cuboid(TREE_HEIGHT / 4.0, TREE_THICKNESS / 2.0),
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(TREE_HEIGHT / 2.0, TREE_THICKNESS))),
                    material: materials.add(Color::rgb(0.0, 0.0, 0.0)),
                    transform: Transform::from_translation(spawn_translation).with_rotation(Quat::from_rotation_z(-45.0)),
                    ..default()
                },
            ));
            
            tree_branches.value += 1;
        }    
    }
}
