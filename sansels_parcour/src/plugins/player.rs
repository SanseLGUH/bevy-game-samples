use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: mut &App) {
	}
}

fn keyboard(keyboard: Res<ButtonInput<KeyCode>>) {
}