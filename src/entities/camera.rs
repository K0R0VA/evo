use bevy::ecs::component::Component;
use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;


pub fn setup(
    commands: &mut Commands,
) {
    let now = 0.4_f32;
    let orbit_scale = 8.0 + now.sin() * 7.0;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                (now / 5.0).cos() * orbit_scale,
                12.0 - orbit_scale / 2.0,
                (now / 5.0).sin() * orbit_scale).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
        },
        FogSettings {
            color: Color::rgba(0.25, 0.25, 0.25, 1.0),
            falloff: FogFalloff::Linear {
                start: 5.0,
                end: 20.0,
            },
            ..default()
        },
        Camera
    ));
}