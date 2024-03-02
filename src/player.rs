use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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

#[derive(Component)]
pub struct Player;

pub fn initialize_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sprite_size = 1.;
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("Player.png"),
            transform: Transform::from_scale(Vec3::splat(sprite_size / 10.)),
            ..default()
        },
        RigidBody::Dynamic,
        GravityScale(2.0),
        Collider::ball(sprite_size * 200.),
    ));
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
