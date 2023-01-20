use bevy::prelude::{App, DefaultPlugins, PluginGroup, WindowDescriptor, WindowPlugin};
use bevy::utils::default;

mod animation;
mod components;
mod constants;
mod events;
mod helpers;
mod input;
mod resources;
mod snake;
mod startup;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Snake".to_string(),
            ..default()
        },
        ..default()
    }));

    resources::register(&mut app);
    startup::register(&mut app);
    events::register(&mut app);
    input::register(&mut app);
    animation::register(&mut app);
    snake::register(&mut app);

    app.run();
}
