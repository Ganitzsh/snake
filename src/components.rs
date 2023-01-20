use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Component)]
pub struct SnakeMovement {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Background;
