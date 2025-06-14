mod plugins;
mod components;

use crate::components::*;
use bevy::{prelude::*, window::*};
use bevy_repier2d::prelude::*;

const ARENA_WIDTH: u8 = 255;
const ARENA_HEIGHT: u8 = 255;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set( WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Sansel's Cute Dino!"),
                resolution: WindowResolution::new(1200., 750.),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(plugins::DinoPlugin)
        .add_systems(Startup, camera_setup)
        .add_systems(Update, (execute_animations, size_scaling, positioning))
        .run();
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                }

                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>
) {
    let window = windows.single().unwrap(); // This will panic if there's not exactly one primary window

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
            1.,
        );
    }
}

fn positioning(windows: Query<&Window, With<PrimaryWindow>>, mut entity: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.single().unwrap();

    for (pos, mut transform) in entity.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
