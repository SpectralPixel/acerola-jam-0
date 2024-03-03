use acerola_jam_0::*;
use bevy::prelude::*;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        }),
        ..default()
    };

    App::new()
        .add_plugins((DefaultPlugins.set(window_plugin), SetupGamePlugin))
        .run();
}
