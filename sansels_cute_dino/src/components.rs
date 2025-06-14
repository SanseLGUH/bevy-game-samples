use bevy::prelude::*;

#[derive(Component)]
pub struct Dino {
	pub speed: f64,
	pub passed: f64,
}

impl Dino {
	pub fn new(s: f64, p: f64) -> Self {
		Dino {
			speed: s,
			passed: p
		}
	} 
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: i64,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: i64) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: i64) -> Timer {
        Timer::new(std::time::Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
pub struct Size {
	pub width: f32,
	pub height: f32	
}

impl Size {
	pub fn new(w: f32, h: f32) -> Self {
		Size {
			width: w,
			height: h
		}
	}

	pub fn square(s: f32) -> Self {
		Size {
			width: s,
			height: s
		}
	} 
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}