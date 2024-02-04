use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy_rapier3d::geometry::{ActiveEvents, Collider};

#[derive(Component)]
pub struct Platform;

pub fn setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let step = materials.add(StandardMaterial {
        base_color: Color::RED,
        perceptual_roughness: 1.0,
        ..default()
    });

    let i = 10;
    let size = i as f32 / 2.0 + 3.0;
    commands.spawn((
            PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -size,
                max_x: size,
                min_z: -size,
                max_z: size,
                min_y: -0.25,
                max_y: 0.25,
            })),
            material: step.clone(),
            ..default()
        }, 
        bevy_rapier3d::dynamics::RigidBody::Fixed, 
        Collider::cuboid(size, 0.25, size),
        ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS,
        Platform
    ));
}