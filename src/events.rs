use bevy::{
    prelude::{
        App, AssetServer, Commands, EventReader, EventWriter, Query, Res, ResMut, SystemSet,
        Transform, Vec3,
    },
    sprite::SpriteBundle,
    utils::default,
    window::Windows,
};

use crate::{
    components::{Food, SnakeMovement},
    constants::{
        FOOD_DEPTH, FOOD_OFFSET_X, FOOD_OFFSET_Y, FOOD_SCALE_FACTOR, SNAKE_SEGMENT_GAP,
        SNAKE_SEGMENT_SCALE_FACTOR, SNAKE_SPEED_INCREMENT,
    },
    helpers::spawn_snake_segment,
    resources::{GameState, SnakeSegments},
};
use rand::Rng;

#[derive(Default)]
pub struct SpawnFood;

#[derive(Default)]
pub struct FoodEaten;

fn spawn_food_event_listener(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut spawn_food_events: EventReader<SpawnFood>,
) {
    for _ in spawn_food_events.iter() {
        let mut rng = rand::thread_rng();
        let window = windows.get_primary().unwrap();
        let food_x = rng.gen_range(FOOD_OFFSET_X..window.width() - FOOD_OFFSET_X);
        let food_y = rng.gen_range(FOOD_OFFSET_Y..window.height() - FOOD_OFFSET_Y);

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("food.png"),
                transform: Transform {
                    translation: Vec3::new(food_x, food_y, FOOD_DEPTH),
                    scale: Vec3::splat(FOOD_SCALE_FACTOR),
                    ..default()
                },
                ..default()
            })
            .insert(Food);
    }
}

fn food_eaten_event_listener(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut food_eaten_events: EventReader<FoodEaten>,
    mut spawn_food_events: EventWriter<SpawnFood>,
    mut segments: ResMut<SnakeSegments>,
    mut transform_query: Query<(&Transform, &mut SnakeMovement)>,
) {
    if food_eaten_events.iter().len() > 0 {
        food_eaten_events.clear();
        spawn_food_events.send_default();

        game_state.speed += SNAKE_SPEED_INCREMENT;

        let last_segment = segments.last().unwrap();
        let (last_transform, last_movement) = transform_query.get_mut(*last_segment).unwrap();

        segments.push(spawn_snake_segment(
            commands,
            asset_server,
            Transform {
                scale: Vec3::splat(SNAKE_SEGMENT_SCALE_FACTOR),
                translation: Vec3::new(
                    last_transform.translation.x + (-last_movement.direction.x * SNAKE_SEGMENT_GAP),
                    last_transform.translation.y + (-last_movement.direction.y * SNAKE_SEGMENT_GAP),
                    1.0,
                ),
                ..default()
            },
        ));
    }
}

pub fn register(app: &mut App) -> &mut App {
    app.add_event::<SpawnFood>()
        .add_event::<FoodEaten>()
        .add_system_set(
            SystemSet::new()
                .label("event_listeners")
                .with_system(spawn_food_event_listener)
                .with_system(food_eaten_event_listener),
        )
}
