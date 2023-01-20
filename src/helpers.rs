use bevy::{
    prelude::{AssetServer, Commands, Entity, Res, Transform, Vec2},
    sprite::SpriteBundle,
    utils::default,
};

use crate::components::{SnakeMovement, SnakeSegment};

pub fn spawn_snake_segment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    transform: Transform,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("segment.png"),
            transform,
            ..default()
        })
        .insert(SnakeSegment)
        .insert(SnakeMovement {
            direction: Vec2::default(),
        })
        .id()
}
