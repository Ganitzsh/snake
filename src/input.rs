use bevy::prelude::{App, Input, KeyCode, Quat, Query, Res, SystemSet, Transform, Vec2, With};

use crate::{
    components::{SnakeHead, SnakeMovement},
    constants::{SNAKE_DEFAULT_DIRECTION, SNAKE_ROTATE_ANGLE},
};

fn snake_head_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_query: Query<(&mut SnakeMovement, &mut Transform), With<SnakeHead>>,
) {
    if head_query.is_empty() {
        return;
    }

    let (mut head_movement, mut head_transform) = head_query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        head_movement.direction = head_movement
            .direction
            .rotate(Vec2::from_angle(SNAKE_ROTATE_ANGLE));
        head_transform.rotation = Quat::from_rotation_z(
            -head_movement
                .direction
                .angle_between(SNAKE_DEFAULT_DIRECTION),
        );
    }

    if keyboard_input.pressed(KeyCode::Right) {
        head_movement.direction = head_movement
            .direction
            .rotate(Vec2::from_angle(-SNAKE_ROTATE_ANGLE));
        head_transform.rotation = Quat::from_rotation_z(
            -head_movement
                .direction
                .angle_between(SNAKE_DEFAULT_DIRECTION),
        );
    }
}

pub fn register(app: &mut App) -> &mut App {
    app.add_system_set(
        SystemSet::new()
            .label("input")
            .with_system(snake_head_input),
    )
}
