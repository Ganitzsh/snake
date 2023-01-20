use bevy::prelude::{App, Deref, DerefMut, Entity, Resource};

#[derive(Default, Resource, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct GameState {
    pub speed: f32,
}

pub fn register(app: &mut App) -> &mut App {
    app.insert_resource(SnakeSegments::default())
        .insert_resource(GameState::default())
}
