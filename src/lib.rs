use bevy::{
    prelude::*,
    window::close_on_esc
};
use bevy_rapier2d::prelude::*;

mod camera;
mod player;

pub struct SetupGamePlugin;

impl Plugin for SetupGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(75.),
                camera::CameraPlugin,
                player::PlayerPlugin,
            ))
            .add_systems(Update, close_on_esc);
    }
}