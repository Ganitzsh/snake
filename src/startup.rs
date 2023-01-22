use bevy::{
    prelude::{
        shape, App, AssetServer, Assets, Camera2dBundle, Commands, Component, EventWriter, Mesh,
        Res, ResMut, SystemSet, Transform, Vec3,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, SpriteBundle},
    utils::default,
    window::Windows,
};

use crate::{
    components::{SnakeHead, SnakeMovement, SnakeSegment},
    constants::{
        BACKGROUND_DEPTH, SNAKE_DEFAULT_DIRECTION, SNAKE_DEFAULT_SPEED, SNAKE_DEPTH,
        SNAKE_HEAD_SCALE_FACTOR, SNAKE_SEGMENT_GAP, SNAKE_SEGMENT_SCALE_FACTOR,
        STARTUP_FOOD_AMOUNT,
    },
    events::SpawnFood,
    helpers::spawn_snake_segment,
    resources::{GameState, SnakeSegments},
};

#[derive(Component)]
struct Background;

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let background_texture = asset_server.load("background.png");

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default()
                .with_scale(Vec3::new(window.width(), window.height(), 0.0))
                .with_translation(Vec3::new(
                    window.width() / 2.0,
                    window.height() / 2.0,
                    BACKGROUND_DEPTH,
                )),
            material: materials.add(ColorMaterial::from(background_texture)),
            ..default()
        })
        .insert(Background);
}

fn setup_camera(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let mut camera = Camera2dBundle::default();

    camera.transform.translation.x = window.width() / 2.0;
    camera.transform.translation.y = window.height() / 2.0;

    commands.spawn(camera);
}

fn initialize_snake(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut game_state: ResMut<GameState>,
    mut segments: ResMut<SnakeSegments>,
) {
    let window = windows.get_primary().unwrap();
    let center_vec = Vec3::new(window.width() / 2.0, window.height() / 2.0, SNAKE_DEPTH);

    let head_transform = Transform {
        scale: Vec3::splat(SNAKE_HEAD_SCALE_FACTOR),
        translation: center_vec,
        ..default()
    };

    let mut tail_transform = Transform {
        scale: Vec3::splat(SNAKE_SEGMENT_SCALE_FACTOR),
        translation: center_vec,
        ..default()
    };

    tail_transform.translation.x -= 2.0 * SNAKE_SEGMENT_GAP;

    *game_state = GameState {
        speed: SNAKE_DEFAULT_SPEED,
    };

    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("head.png"),
                transform: head_transform,
                ..default()
            })
            .insert(SnakeHead)
            .insert(SnakeSegment)
            .insert(SnakeMovement {
                direction: SNAKE_DEFAULT_DIRECTION,
            })
            .id(),
        spawn_snake_segment(commands, asset_server, tail_transform),
    ])
}

fn initialize_food(mut spawn_food_events: EventWriter<SpawnFood>) {
    for _ in 0..STARTUP_FOOD_AMOUNT {
        spawn_food_events.send_default();
    }
}

pub fn register(app: &mut App) -> &mut App {
    app.add_startup_system_set(
        SystemSet::new()
            .label("startup")
            .with_system(setup_background)
            .with_system(setup_camera)
            .with_system(initialize_food)
            .with_system(initialize_snake),
    )
}
