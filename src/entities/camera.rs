use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;

use super::snake::SnakeHead;

#[derive(Component)]
pub struct Camera;


pub fn setup(
    commands: &mut Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                0.,
                5.,
                -5.
            ).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        Camera
    ));
}

pub fn look_at_snake(
    mut camera: Query<(&mut Transform, With<Camera>), Without<SnakeHead>>,
    snake_position_query: Query<(&Transform, &Velocity, With<SnakeHead>), Without<Camera>>
) {

    let (mut camera, _) = camera.single_mut();
    let (snake_position, snake_velocity, _) = snake_position_query.single();
    if snake_velocity.linvel.length() > 0.5 {
        let mut velocity = snake_velocity.linvel;
        velocity.y = 0.;
        let look_direction = velocity.normalize() * 5.;
        let translation = snake_position.translation;
        camera.translation = translation - look_direction + Vec3::Y * 5.;
        camera.look_at(translation, Vec3::Y);
    }
}