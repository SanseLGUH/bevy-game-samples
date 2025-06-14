use bevy::prelude::*;

use crate::{*, components::*};

mod dino;
mod obstacles;

pub struct DinoPlugin;
impl Plugin for DinoPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, spawn_dino)
			.add_systems(Update, (jump_animation, dash_animation));
	}
}

pub fn spawn_dino(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let texture_atlas_handle = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 1, None, None));
    let animation_config = AnimationConfig::new(0, 5, 10);

	commands.spawn((
		Dino::new(1., 0.),
	    Sprite {
	        image: asset_server.load("dino/move.png"),
	        texture_atlas: Some(TextureAtlas {
	            layout: texture_atlas_handle.clone(),
	            index: animation_config.first_sprite_index,
	        }),
	        ..default()
	    }, animation_config,
		Size::new(2., 3.), Position { x: 35, y: 75 }
	));
}

pub fn jump_animation(keyboard_input: Res<ButtonInput<KeyCode>>, entity: Query<(&mut Position, &mut Sprite, &mut AnimationConfig), With<Dino>>) {
	if keyboard_input.pressed(KeyCode::Space) {
		for (mut pos, mut sprite, mut config) in entity {
			pos.y = 100;
		}
	}
}

pub fn dash_animation() {

}

// pub fn 