use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                RapierDebugRenderPlugin {
                    //enabled: false,
                    ..default()
            })
            .add_systems(
                Startup,
                setup_camera
            );
    }
}

#[derive(Component)]
struct MainCamera;

pub fn setup_camera(
    mut commands: Commands
) {
    commands.spawn((
        MainCamera,
        Camera2dBundle { ..default() },
    ));
}
