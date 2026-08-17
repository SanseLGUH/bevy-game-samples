use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, setup_camera)
			.add_systems(Update, camera_follows);
	}
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

use crate::Player;

fn camera_follows(
    mut cameras_pos: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    players_pos: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let mut camera_pos = cameras_pos.single_mut().unwrap();
    let player_pos = players_pos.single().unwrap();
    camera_pos.translation = player_pos.translation;
}