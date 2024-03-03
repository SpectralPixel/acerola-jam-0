use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 5.5;
const VERTICAL_THRUST_FACTOR: f32 = 1.7;
const TURN_THRUST_FACTOR: f32 = 0.001;
const NEW_MOVEMENT: bool = true;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, initialize_player)
            .add_systems(
                Update,
                //(handle_input, apply_forces)
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
    let size_factor = 0.02;
    let collider_size = 750.;
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("Player-alt.png"),
            //transform: Transform::from_scale(Vec3::splat(size_factor)),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                rotation: Quat::from_rotation_z(PI / 2.),
                scale: Vec3::splat(size_factor),
            },
            ..default()
        },
        RigidBody::Dynamic,
        GravityScale(5.0),
        ColliderMassProperties::Density(4.0),
        Collider::ball(collider_size),
        ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.,
        },
        Damping {
            linear_damping: 0.3,
            angular_damping: 1.0,
        },
    ));
}

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    player_transforms: Query<&mut Transform, With<Player>>,
    mut ext_forces: Query<&mut ExternalImpulse, With<Player>>,
) {
    for player_transform in player_transforms.iter() {
        if NEW_MOVEMENT {
            if keys.pressed(KeyCode::KeyW) {
                println!("W Pressed!");
                let (_, _, rotation) = player_transform.rotation.to_euler(EulerRot::XYZ);
                apply_force(
                    Vec2::new(
                        f32::cos(rotation) * PLAYER_SPEED,
                        f32::sin(rotation) * PLAYER_SPEED,
                    ),
                    0.,
                    &mut ext_forces
                );
            }
            if keys.pressed(KeyCode::KeyD) {
                println!("D Pressed!");
                apply_force(
                    Vec2::new(0., 0.),
                    -TURN_THRUST_FACTOR,
                    &mut ext_forces
                );
            }
            if keys.pressed(KeyCode::KeyA) {
                println!("A Pressed!");
                apply_force(
                    Vec2::new(0., 0.),
                    TURN_THRUST_FACTOR,
                    &mut ext_forces
                );
            }
        } else {
            if keys.pressed(KeyCode::KeyW) {
                println!("W Pressed!");
                apply_force(
                    Vec2::new(0., PLAYER_SPEED * VERTICAL_THRUST_FACTOR),
                    0.,
                    &mut ext_forces
                );
            }
            if keys.pressed(KeyCode::KeyS) {
                println!("S Pressed!");
                apply_force(
                    Vec2::new(0., -PLAYER_SPEED / VERTICAL_THRUST_FACTOR),
                    0.,
                    &mut ext_forces
                );
            }
            if keys.pressed(KeyCode::KeyD) {
                println!("D Pressed!");
                apply_force(
                    Vec2::new(PLAYER_SPEED, 0.),
                    0.,
                    &mut ext_forces
                );
            }
            if keys.pressed(KeyCode::KeyA) {
                println!("A Pressed!");
                apply_force(
                    Vec2::new(-PLAYER_SPEED, 0.),
                    0.,
                    &mut ext_forces
                );
            }
        }
    }
}

fn apply_force(
    pos_offset: Vec2,
    torque_offset: f32,
    ext_forces: &mut Query<&mut ExternalImpulse, With<Player>>
) {
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.impulse += pos_offset;
        ext_force.torque_impulse += torque_offset;
    }
}
