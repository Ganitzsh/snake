use bevy::{
    app::AppExit,
    prelude::{
        App, Commands, Entity, EventWriter, IntoSystemDescriptor, Query, Res, ResMut, SystemSet,
        Transform, Vec2, With,
    },
    window::Windows,
};

use crate::{
    components::{Food, SnakeHead, SnakeMovement, SnakeSegment},
    constants::{SNAKE_EAT_SELF_DISTANCE, SNAKE_SEGMENT_GAP, SNAKE_SPEED_INCREMENT},
    events::FoodEaten,
    resources::{GameState, SnakeSegments},
};

fn snake_head_movement(
    game_state: Res<GameState>,
    mut head_positions: Query<(&SnakeHead, &SnakeMovement, &mut Transform)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    for (_head, movement, mut transform) in head_positions.iter_mut() {
        transform.translation.x += movement.direction.x * game_state.speed;
        transform.translation.y += movement.direction.y * game_state.speed;

        if transform.translation.x > window.width() {
            transform.translation.x = 0.0;
        } else if transform.translation.x < 0.0 {
            transform.translation.x = window.width();
        }

        if transform.translation.y > window.height() {
            transform.translation.y = 0.0;
        } else if transform.translation.y < 0.0 {
            transform.translation.y = window.height();
        }
    }
}

fn snake_segments_movement(
    windows: Res<Windows>,
    game_state: Res<GameState>,
    segments: Res<SnakeSegments>,
    mut segments_query: Query<(&mut SnakeMovement, &mut Transform), With<SnakeSegment>>,
) {
    println!("segments: {:?}", segments.0);
    let window = windows.get_primary().unwrap();
    let transforms: Vec<Transform> = segments
        .iter()
        .map(|e| *segments_query.get(*e).unwrap().1)
        .collect();

    for ((mut movement, mut transform), prev_transform) in
        segments_query.iter_mut().skip(1).zip(transforms.iter())
    {
        let mut diff_x;
        let mut diff_y;

        diff_x = prev_transform.translation.x - transform.translation.x;
        if diff_x.abs() >= window.width() - 50.0 {
            diff_x = match diff_x < 0.0 {
                true => diff_x + window.width(),
                false => diff_x - window.width(),
            }
        }

        diff_y = prev_transform.translation.y - transform.translation.y;
        if diff_y.abs() >= window.height() - 50.0 {
            diff_y = match diff_y < 0.0 {
                true => diff_y + window.height(),
                false => diff_y - window.height(),
            };
        }

        movement.direction.x = diff_x;
        movement.direction.y = diff_y;
        movement.direction = movement.direction.normalize();

        let diff_len = Vec2::new(diff_x, diff_y).length();

        transform.translation.x +=
            movement.direction.x * (diff_len / SNAKE_SEGMENT_GAP) * game_state.speed;
        transform.translation.y +=
            movement.direction.y * (diff_len / SNAKE_SEGMENT_GAP) * game_state.speed;

        if transform.translation.x < 0.0 {
            transform.translation.x = window.width();
        } else if transform.translation.x > window.width() {
            transform.translation.x = 0.0;
        }

        if transform.translation.y < 0.0 {
            transform.translation.y = window.height();
        } else if transform.translation.y > window.height() {
            transform.translation.y = 0.0;
        }
    }
}

fn check_eat_self(
    mut game_state: ResMut<GameState>,
    mut segments: ResMut<SnakeSegments>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    segments_query: Query<&Transform, With<SnakeSegment>>,
) {
    let head_transform = segments_query.iter().next().unwrap();

    for (i, segment_transform) in segments_query.iter().skip(2).enumerate() {
        let distance = segment_transform
            .translation
            .distance(head_transform.translation);

        if distance <= SNAKE_EAT_SELF_DISTANCE {
            let corresponding_segments = segments.split_off(i + 2);
            let segments_eaten = corresponding_segments.len();

            for entity in corresponding_segments.into_iter() {
                commands.entity(entity).despawn();
            }

            if segments.len() <= 1 {
                exit.send(AppExit);
            }

            game_state.speed -= segments_eaten as f32 * SNAKE_SPEED_INCREMENT;
        }
    }
}

fn check_food_eaten(
    mut commands: Commands,
    foods_query: Query<(Entity, &Transform), With<Food>>,
    head_query: Query<&Transform, With<SnakeHead>>,
    mut food_eaten_events: EventWriter<FoodEaten>,
) {
    if head_query.is_empty() {
        return;
    }

    let head_transform = head_query.single();

    for (entity, food_transform) in foods_query.iter() {
        if food_transform
            .translation
            .distance(head_transform.translation)
            <= 20.0
        {
            commands.entity(entity).despawn();
            food_eaten_events.send_default();
        }
    }
}

pub fn register(app: &mut App) -> &mut App {
    app.add_system_set(
        SystemSet::new()
            .label("snake_logic")
            .with_system(snake_head_movement)
            .with_system(snake_segments_movement.after(snake_head_movement))
            .with_system(check_food_eaten.after(snake_head_movement))
            .with_system(check_eat_self.after(snake_segments_movement)),
    )
}
