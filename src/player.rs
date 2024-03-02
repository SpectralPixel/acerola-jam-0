use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, initialize_player)
            .add_systems(
                Update,
                handle_input
            );
    }
}

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyW) {
        println!("W Pressed!");
    }
    if keys.just_pressed(KeyCode::KeyS) {
        println!("S Pressed!");
    }
    if keys.just_pressed(KeyCode::KeyD) {
        println!("D Pressed!");
    }
    if keys.just_pressed(KeyCode::KeyA) {
        println!("A Pressed!");
    }
}
