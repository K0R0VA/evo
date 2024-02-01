use bevy::ecs::{component::Component, system::Query};
use bevy::prelude::*;
use bevy_rapier3d::geometry::ActiveEvents;
use bevy_rapier3d::{dynamics::Velocity, geometry::{Collider, Restitution}};

#[derive(Component)]
pub struct Ball;

#[derive(Component, Default)]
pub struct BallCollisionState {
    pub on_platform: bool
}


pub fn setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                    radius: 0.5,
                    ..Default::default()
                }).unwrap()),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..default()
                }),
                transform: Transform::from_scale(Vec3::splat(1.))
                    .with_translation(Vec3::new(0.0, 7.0, 0.0)),
                ..default()
            },
            bevy_rapier3d::dynamics::RigidBody::Dynamic,
            Collider::ball(0.5),
            Velocity::linear(Vec3::new(0.0, -2.0, 0.0)),
            Restitution::coefficient(0.9),
            ActiveEvents::COLLISION_EVENTS,
            BallCollisionState::default(),
            Ball
        ));
}

pub fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &BallCollisionState, With<Ball>)>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (mut velocity, _, ..) in query.iter_mut().filter(|(_, BallCollisionState {on_platform}, _)| *on_platform) {
            let x = velocity.linvel.x;
            let z = velocity.linvel.z;
            velocity.linvel = Vec3::new(x, 3.0, z); 
        }
    }
}

pub fn move_by_keybord(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, With<Ball>)> 
) {
    let w = keyboard_input.pressed(KeyCode::W);
    let s = keyboard_input.pressed(KeyCode::S);
    let a = keyboard_input.pressed(KeyCode::A);
    let d = keyboard_input.pressed(KeyCode::D);
    if a || s || w || d {
        for (mut velocity, _, ..) in query.iter_mut() {
            let x = s.then_some(0.6).unwrap_or_default() + w.then_some(-0.6).unwrap_or_default();
            let z = a.then_some(0.6).unwrap_or_default() + d.then_some(-0.6).unwrap_or_default();
            let y = velocity.linvel.y;
            velocity.linvel = Vec3::new(x, y, z);
        }
    }
}
