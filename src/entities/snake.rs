use bevy::ecs::{component::Component, system::Query};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::{ ImpulseJoint,  RigidBody, RopeJointBuilder};
use bevy_rapier3d::geometry::ActiveEvents;
use bevy_rapier3d::{dynamics::Velocity, geometry::{Collider, Restitution}};

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component, Default)]
pub struct SnakeCollisionState {
    pub on_platform: bool
}

pub fn setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SpatialBundle::default(), 
        Snake
    ))
    .with_children(|parent| {

        let material = materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        });
        
        let snake_head = parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                    radius: 0.5,
                    ..Default::default()
                }).unwrap()),
                transform: Transform::from_xyz(0.0, 0.75, 0.),
                material: material.clone(),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Velocity::linear(Vec3::new(0.0, 0.0, 0.0)),
            Restitution::coefficient(1.),
            ActiveEvents::COLLISION_EVENTS,
            SnakeCollisionState::default(),
            SnakeHead,
        )).id();

        let mut curr_parent = snake_head;

        let shift = 0.5;
        let num = 5;
        let radius = 0.25;

        for i in 0..num {
            let dz = (i + 1) as f32 * shift;
    
            let rope = RopeJointBuilder::new(0.5)
                .local_anchor2(Vec3::new(0.0, 0.0, -shift));
            let joint = ImpulseJoint::new(curr_parent, rope);
            let mesh = meshes.add(Mesh::try_from(shape::Icosphere {
                radius,
                ..Default::default()
            }).unwrap());

            curr_parent = parent
                .spawn((
                    PbrBundle {
                        mesh: mesh.clone(),
                        transform: Transform::from_xyz(0., 0.55, 0. + dz),
                        material: material.clone(),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(radius, radius, radius),
                    joint,
                ))
                .id();
        }

    });
    

}

pub fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &SnakeCollisionState, With<SnakeHead>)>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (mut velocity, _, ..) in query.iter_mut().filter(|(_, SnakeCollisionState {on_platform}, _)| *on_platform) {
            let x = velocity.linvel.x;
            let z = velocity.linvel.z;
            velocity.linvel = Vec3::new(x, 3.0, z); 
        }
    }
}

pub fn move_by_keybord(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_velocity: Query<(&mut Velocity, &Transform, With<SnakeHead>), Without<Camera>>, 
    camera_position: Query<(&Transform, With<Camera>), Without<SnakeHead>>
) {
    let w = keyboard_input.pressed(KeyCode::W);
    let a = keyboard_input.pressed(KeyCode::A);
    let d = keyboard_input.pressed(KeyCode::D);
    let shift = keyboard_input.pressed(KeyCode::ShiftLeft);
    if w {
        let (mut snake_velocity, snake_position, _) = snake_velocity.single_mut();
        let (camera_position, _) = camera_position.single();
        let mut direction = snake_position.translation - camera_position.translation;
        let right =  camera_position.right() * 0.1;
        if a { direction -= right; } else if d { direction += right; }
        direction.y = 0.;
        const SPEED: f32 = 1.0;
        const TURBO: f32 = 2.0;
        let mut velocity = direction.normalize() * shift.then_some(TURBO).unwrap_or(SPEED);
        velocity.y = snake_velocity.linvel.y;
        snake_velocity.linvel = velocity;
    }
}
