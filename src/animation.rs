use bevy::prelude::{IntoSystemDescriptor, App, Query, Transform, With};

use crate::components::Food;

fn animate_food(mut foods_query: Query<&mut Transform, With<Food>>) {
    for mut food_transform in foods_query.iter_mut() {
        food_transform.rotate_z(0.05);
    }
}

pub fn register(app: &mut App) -> &mut App {
    app.add_system(animate_food.after("snake_logic"))
}
