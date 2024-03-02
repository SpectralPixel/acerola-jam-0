use bevy::{
    prelude::*,
    window::close_on_esc
};

pub struct SetupGamePlugin;

impl Plugin for SetupGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins((
            // ))
            .add_systems(Update, close_on_esc);
    }
}