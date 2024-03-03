use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::geometry::Collider;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_level);
    }
}

pub struct LevelRectangle {
    translation: Vec2,
    scale: Vec2,
}

impl LevelRectangle {
    pub fn new(translation: Vec2, scale: Vec2) -> Self {
        LevelRectangle { translation, scale }
    }
}

pub fn initialize_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shapes = [
        LevelRectangle::new(
            Vec2::new(0., -370.),
            Vec2::new(1300., 30.),
        ),
        LevelRectangle::new(
            Vec2::new(0., 370.),
            Vec2::new(1300., 30.),
        ),
        LevelRectangle::new(
            Vec2::new(-650., 0.),
            Vec2::new(30., 750.),
        ),
        LevelRectangle::new(
            Vec2::new(650., 0.),
            Vec2::new(30., 750.),
        ),
    ];

    for rect in shapes.into_iter() {
        let color = Color::hsl(220., 0.95, 0.7);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(rect.scale.x, rect.scale.y))),
                material: materials.add(color),
                transform: Transform::from_xyz(rect.translation.x, rect.translation.y, 0.0),
                ..default()
            },
            Collider::cuboid(rect.scale.x / 2., rect.scale.y / 2.),
        ));
    }
}
