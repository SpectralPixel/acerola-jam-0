use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 5.;

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
    let sprite_size = 1.;
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("Player.png"),
            transform: Transform::from_scale(Vec3::splat(sprite_size / 10.)),
            ..default()
        },
        RigidBody::Dynamic,
        GravityScale(5.0),
        ColliderMassProperties::Density(4.0),
        Collider::ball(sprite_size * 200.),
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
    mut ext_forces: Query<&mut ExternalImpulse, With<Player>>
) {
    if keys.pressed(KeyCode::KeyW) {
        println!("W Pressed!");
        apply_force(
            Vec2::new(0., PLAYER_SPEED),
            0.,
            &mut ext_forces
        );
    }
    if keys.pressed(KeyCode::KeyS) {
        println!("S Pressed!");
        apply_force(
            Vec2::new(0., -PLAYER_SPEED),
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
