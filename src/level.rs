use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_level);
    }
}

pub struct LevelRectangle {
    translation: Vec2,
    scale: Vec2,
    rect_type: RectangleType,
}

impl LevelRectangle {
    pub fn new(translation: Vec2, scale: Vec2, rect_type: RectangleType) -> Self {
        LevelRectangle { translation, scale, rect_type }
    }
}

pub enum RectangleType {
    Kill, // kills the player
    Solid, // solid wall, friction is applied
    Ice, // doesn't cause the player to roll
    SpeedBarrier(f32), // wall that can be traversed when a given speed is reached
}

pub fn initialize_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shapes = [
        LevelRectangle::new(
            Vec2::new(0., -570.),
            Vec2::new(2000., 30.),
            RectangleType::Ice,
        ),
        LevelRectangle::new(
            Vec2::new(0., 570.),
            Vec2::new(2000., 30.),
            RectangleType::Solid,
        ),
        LevelRectangle::new(
            Vec2::new(-1000., 0.),
            Vec2::new(30., 1155.),
            RectangleType::Solid,
        ),
        LevelRectangle::new(
            Vec2::new(1000., 0.),
            Vec2::new(30., 1155.),
            RectangleType::Solid,
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
            if let RectangleType::Ice = rect.rect_type {
                Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                }
            } else { Friction { ..Default::default() } },
        ));
    }
}
