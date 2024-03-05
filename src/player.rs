use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

const PLAYER_SPEED: f32 = 660.;
const TURN_THRUST_FACTOR: f32 = 0.12;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, initialize_player);
    }
}

#[derive(Component)]
pub struct Player;

pub fn initialize_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let size_factor = 0.02;
    let collider_size = 750.;
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("Player-alt.png"),
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

pub fn move_player(
    is_moving: f32,
    rot_dir: f32,
    angle: f32,
    time: &Res<Time>,
    mut ext_forces: &mut Query<&mut ExternalImpulse, With<Player>>,
) {
    let delta_time = time.delta_seconds();
    apply_force(
        Vec2::new(
            f32::cos(angle) * PLAYER_SPEED * is_moving * delta_time,
            f32::sin(angle) * PLAYER_SPEED * is_moving * delta_time,
        ),
        TURN_THRUST_FACTOR * rot_dir * delta_time,
        &mut ext_forces,
    );
}

fn apply_force(
    pos_offset: Vec2,
    torque_offset: f32,
    ext_forces: &mut Query<&mut ExternalImpulse, With<Player>>,
) {
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.impulse += pos_offset;
        ext_force.torque_impulse += torque_offset;
    }
}
