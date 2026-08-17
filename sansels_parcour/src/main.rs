pub mod plugins;
use crate::plugins::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

fn setup_map(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(500., 50.))
        .insert( Transform::from_xyz(0., -100., 0.) );

    commands
        .spawn(Player)
        .insert( RigidBody::Dynamic )
        .insert( Collider::cuboid(50., 50.) )
        .insert( LockedAxes::ROTATION_LOCKED )
        .insert( Restitution::coefficient(0.7) )
        .insert( Transform::from_xyz(0.0, 400.0, 0.0) );
} 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_map)
        .run();
}