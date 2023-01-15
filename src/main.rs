use bevy::{app::AppExit, prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Background;

#[derive(Default, Resource, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

#[derive(Component)]
struct SnakeMovement {
    direction: Vec2,
    speed: f32,
}

#[derive(Default)]
struct SpawnFood;

#[derive(Default)]
struct FoodEaten;

const STARTUP_FOOD_AMOUNT: i32 = 20;
const FOOD_OFFSET_X: f32 = 20.0;
const FOOD_OFFSET_Y: f32 = 20.0;
const FOOD_SCALE_FACTOR: f32 = 0.15;
const SNAKE_SPEED_INCREMENT: f32 = 0.05;
const SNAKE_ROTATE_ANGLE: f32 = 0.15;
const SNAKE_HEAD_SCALE_FACTOR: f32 = 0.35;
const SNAKE_SEGMENT_SCALE_FACTOR: f32 = 0.25;
const SNAKE_SEGMENT_GAP: f32 = 32.5;
const SNAKE_DEFAULT_DIRECTION: Vec2 = Vec2::new(1.0, 0.0);
const SNAKE_DEFAULT_SPEED: f32 = 2.0;

fn spawn_snake_segment(
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
            direction: Vec2::ZERO,
            speed: SNAKE_DEFAULT_SPEED,
        })
        .id()
}

fn spawn_snake(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut segments: ResMut<SnakeSegments>,
) {
    let window = windows.get_primary().unwrap();
    let center_vec = Vec3::new(window.width() / 2.0, window.height() / 2.0, 1.0);

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

    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("head.png"),
                transform: head_transform,
                ..default()
            })
            .insert(SnakeHead)
            .insert(SnakeMovement {
                direction: SNAKE_DEFAULT_DIRECTION,
                speed: SNAKE_DEFAULT_SPEED,
            })
            .id(),
        spawn_snake_segment(commands, asset_server, tail_transform),
    ])
}

fn snake_head_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_query: Query<(&mut SnakeMovement, &mut Transform), With<SnakeHead>>,
) {
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

fn snake_head_movement(
    mut head_positions: Query<(&SnakeHead, &SnakeMovement, &mut Transform)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    for (_head, movement, mut transform) in head_positions.iter_mut() {
        transform.translation.x += movement.direction.x * movement.speed;
        transform.translation.y += movement.direction.y * movement.speed;

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

fn food_eaten_event_listener(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut food_eaten_events: EventReader<FoodEaten>,
    mut spawn_food_events: EventWriter<SpawnFood>,
    mut segments: ResMut<SnakeSegments>,
    mut transform_query: Query<(&Transform, &mut SnakeMovement)>,
) {
    if food_eaten_events.iter().len() > 0 {
        food_eaten_events.clear();
        spawn_food_events.send_default();

        let (_, mut head_movement) = transform_query.get_mut(*segments.first().unwrap()).unwrap();
        head_movement.speed += SNAKE_SPEED_INCREMENT;

        info!("Snake speed: {}", head_movement.speed);

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

fn animate_food(mut foods_query: Query<&mut Transform, With<Food>>) {
    for mut food_transform in foods_query.iter_mut() {
        food_transform.rotate_z(0.05);
    }
}

fn spawn_food_event_listener(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut spawn_food_events: EventReader<SpawnFood>,
) {
    for _ in spawn_food_events.iter() {
        let mut rng = rand::thread_rng();
        let window = windows.get_primary().unwrap();
        let food_x = rng.gen_range(FOOD_OFFSET_X, window.width() - FOOD_OFFSET_X);
        let food_y = rng.gen_range(FOOD_OFFSET_Y, window.height() - FOOD_OFFSET_Y);

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("food.png"),
                transform: Transform {
                    translation: Vec3::new(food_x, food_y, 0.5),
                    scale: Vec3::splat(FOOD_SCALE_FACTOR),
                    ..default()
                },
                ..default()
            })
            .insert(Food);
    }
}

fn snake_grow(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut spawn_food_events: EventWriter<SpawnFood>,
    mut segments: ResMut<SnakeSegments>,
    transform_query: Query<(&Transform, &SnakeMovement)>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        spawn_food_events.send_default();
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        let last_segment = segments.last().unwrap();
        let (last_transform, last_movement) = transform_query.get(*last_segment).unwrap();

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
        ))
    }
}

fn check_eat_self(
    mut segments: ResMut<SnakeSegments>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    mut head_query: Query<(&Transform, &mut SnakeMovement), With<SnakeHead>>,
    segments_query: Query<&Transform, With<SnakeSegment>>,
) {
    let (head_transform, mut head_movement) = head_query.single_mut();

    for (i, segment_transform) in segments_query.iter().enumerate() {
        if segment_transform
            .translation
            .distance(head_transform.translation)
            <= 10.0
        {
            let corresponding_segments = segments.split_off(i);
            let segments_eaten = corresponding_segments.len();

            info!("You ate yourself! {:?}", corresponding_segments);

            for entity in corresponding_segments.into_iter() {
                commands.entity(entity).despawn();
            }

            if segments.len() <= 1 {
                exit.send(AppExit);
            }

            head_movement.speed -= segments_eaten as f32 * SNAKE_SPEED_INCREMENT;
        }
    }
}

fn check_food_eaten(
    mut commands: Commands,
    foods_query: Query<(Entity, &Transform), With<Food>>,
    head_query: Query<&Transform, With<SnakeHead>>,
    mut food_eaten_events: EventWriter<FoodEaten>,
) {
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

fn snake_segments_movement(
    segments: Res<SnakeSegments>,
    mut segments_query: Query<(&mut SnakeMovement, &mut Transform), Without<SnakeHead>>,
    head_query: Query<(&SnakeMovement, &Transform), With<SnakeHead>>,
) {
    let (head_movement, head_transform) = head_query.single();

    let segments_transforms: Vec<Transform> = segments
        .iter()
        .skip(1)
        .map(|e| -> Transform { *segments_query.get(*e).unwrap().1 })
        .collect();

    for (index, (mut movement, mut transform)) in segments_query.iter_mut().enumerate() {
        let diff_x;
        let diff_y;

        if index == 0 {
            diff_x = head_transform.translation.x - transform.translation.x;
            diff_y = head_transform.translation.y - transform.translation.y;
        } else if segments_transforms.len() > 1 {
            let prev_transform = segments_transforms.get(index - 1).unwrap();

            diff_x = prev_transform.translation.x - transform.translation.x;
            diff_y = prev_transform.translation.y - transform.translation.y;
        } else {
            break;
        }

        movement.direction.x = diff_x;
        movement.direction.y = diff_y;
        movement.direction = movement.direction.normalize();

        let diff_len = Vec2::new(diff_x, diff_y).length();

        transform.translation.x +=
            movement.direction.x * (diff_len / SNAKE_SEGMENT_GAP) * head_movement.speed;
        transform.translation.y +=
            movement.direction.y * (diff_len / SNAKE_SEGMENT_GAP) * head_movement.speed;

        transform.rotate_z(0.05)
    }
}

fn setup_camera(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let mut camera = Camera2dBundle::default();

    camera.transform.translation.x = window.width() / 2.0;
    camera.transform.translation.y = window.height() / 2.0;

    commands.spawn(camera);
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.3,
    });
}

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
                .with_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)),
            material: materials.add(ColorMaterial::from(background_texture)),
            ..default()
        })
        .insert(Background);
}

fn initialize_food(mut spawn_food_events: EventWriter<SpawnFood>) {
    for _ in 0..STARTUP_FOOD_AMOUNT {
        spawn_food_events.send_default();
    }
}

fn main() {
    App::new()
        .insert_resource(SnakeSegments::default())
        .add_event::<SpawnFood>()
        .add_event::<FoodEaten>()
        .add_startup_system(setup_background)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_startup_system(initialize_food)
        .add_system(snake_head_input)
        .add_system(snake_head_movement)
        .add_system(snake_segments_movement)
        .add_system(animate_food)
        .add_system(
            snake_grow
                .before(check_eat_self)
                .after(snake_segments_movement),
        )
        .add_system(check_food_eaten)
        .add_system(check_eat_self.after(snake_segments_movement))
        .add_system(spawn_food_event_listener)
        .add_system(food_eaten_event_listener)
        .add_plugins(DefaultPlugins)
        .run();
}
