use bevy::prelude::*;

pub const STARTUP_FOOD_AMOUNT: i32 = 50;
pub const SNAKE_EAT_SELF_DISTANCE: f32 = 10.0;
pub const FOOD_OFFSET_X: f32 = 20.0;
pub const FOOD_OFFSET_Y: f32 = 20.0;
pub const FOOD_SCALE_FACTOR: f32 = 0.15;
pub const SNAKE_SPEED_INCREMENT: f32 = 0.05;
pub const SNAKE_ROTATE_ANGLE: f32 = 0.15;
pub const SNAKE_HEAD_SCALE_FACTOR: f32 = 0.35;
pub const SNAKE_SEGMENT_SCALE_FACTOR: f32 = 0.25;
pub const SNAKE_SEGMENT_GAP: f32 = 32.5;
pub const SNAKE_DEFAULT_DIRECTION: Vec2 = Vec2::new(1.0, 0.0);
pub const SNAKE_DEFAULT_SPEED: f32 = 2.0;
pub const SNAKE_DEPTH: f32 = 1.0;
pub const FOOD_DEPTH: f32 = 0.5;
pub const BACKGROUND_DEPTH: f32 = 0.0;
