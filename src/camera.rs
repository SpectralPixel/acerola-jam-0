use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierDebugRenderPlugin {
            //enabled: false,
            ..default()
        })
        .add_systems(Startup, setup_camera)
        .add_systems(Update, move_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle { ..default() }));
}

pub fn move_camera(
    mut cameras: Query<(&MainCamera, &mut Transform), Without<super::player::Player>>,
    players: Query<&Transform, With<super::player::Player>>,
) {
    let players: Vec<&Transform> = players.iter().collect();
    for (i, (_, mut cam_pos)) in cameras.iter_mut().enumerate() {
        let pos = players.get(i).unwrap().translation;
        *cam_pos = Transform::from_translation(Vec3::new(pos.x, pos.y, 0.));
    }
}
