use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::player;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_input);
    }
}

pub fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    player_transforms: Query<&mut Transform, With<player::Player>>,
    mut ext_forces: Query<&mut ExternalImpulse, With<player::Player>>,
    time: Res<Time>,
) {
    for player_transform in player_transforms.iter() {
        let mut forward: f32 = 0.;
        let mut direction: f32 = 0.;
        if keys.pressed(KeyCode::KeyW) {
            forward = 1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction -= 1.;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += 1.;
        }
        let (_, _, rotation) = player_transform.rotation.to_euler(EulerRot::XYZ);
        super::player::move_player(forward, direction, rotation, &time, &mut ext_forces);
    }
}
