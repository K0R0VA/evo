// use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;

// fn main() {
//     App::new()
//              .add_plugins(DefaultPlugins)
//             .add_plugins(RapierPhysicsPlugin::<()>::default())
//             .add_plugins(RapierDebugRenderPlugin {
//                 mode: DebugRenderMode::all(),
//                 ..Default::default()
//             })
//             .add_systems(Startup, setup)
//         .run();
// }

// fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
//     // Камера
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });

//     // Платформа
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, ..Default::default() })),
//             material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
//             transform: Transform::from_xyz(0.0, 3.0, 0.0),
//             ..default()
//         },
//         RigidBody::Fixed,
//         Collider::cuboid(2.5, 0.1, 2.5),
//     ));

//     // Шарик
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Mesh::try_from(shape::Icosphere { radius: 0.5, ..Default::default() }).unwrap()),
//             material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//             transform: Transform::from_xyz(0.0, 5.0, 0.0),
//             ..default()
//         },
//         RigidBody::Dynamic,
//         Collider::ball(0.5),
//         Velocity::linear(Vec3::new(0.0, -2.0, 0.0)),
//     ));
// }

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_rapier3d::{dynamics::{Ccd, GravityScale, Velocity}, geometry::{Collider, ColliderMassProperties, Friction, Restitution}, plugin::{NoUserData, RapierPhysicsPlugin}, render::{DebugRenderMode, RapierDebugRenderPlugin}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..Default::default()
        })
        .add_systems(
            Startup,
            (setup_camera_fog, setup_pyramid_scene),
        )
        .add_systems(Update, update_system)
        .run();
}

fn setup_camera_fog(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        FogSettings {
            color: Color::rgba(0.25, 0.25, 0.25, 1.0),
            falloff: FogFalloff::Linear {
                start: 5.0,
                end: 20.0,
            },
            ..default()
        },
    ));
}

fn setup_pyramid_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let step = materials.add(StandardMaterial {
        base_color: Color::RED,
        perceptual_roughness: 1.0,
        ..default()
    });

    let pillar = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        perceptual_roughness: 1.0,
        ..default()
    });

    // orb
    

        // pillars
    for (x, z) in &[(-1.5, -1.5), (1.5, -1.5), (1.5, 1.5), (-1.5, 1.5)] {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -0.5,
                    max_x: 0.5,
                    min_z: -0.5,
                    max_z: 0.5,
                    min_y: 0.0,
                    max_y: 3.0,
                })),
                material: pillar.clone(),
                transform: Transform::from_xyz(*x, 0.0, *z),
                ..default()
            }, 
            bevy_rapier3d::dynamics::RigidBody::Fixed,
            Collider::cuboid(0.5, 3.0, 0.5)
        ));
    }


    // steps
    for i in 0..25 {
        let size = i as f32 / 2.0 + 3.0;
        let y = -i as f32 / 2.0;
        commands.spawn((
                PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -size,
                    max_x: size,
                    min_z: -size,
                    max_z: size,
                    min_y: 0.0,
                    max_y: 0.5,
                })),
                material: step.clone(),
                transform: Transform::from_xyz(0.0, y, 0.0),
                ..default()
            }, 
            bevy_rapier3d::dynamics::RigidBody::Fixed, 
            Collider::cuboid(size, 0.5, size)
        ));
    }

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                    radius: 1.5,
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
            Collider::ball(1.5),
            Ccd::enabled(),
            Velocity::linear(Vec3::new(0.0, 1.0, 0.0))
        ));
    

    // sky
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::default())),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("888888").unwrap(),
            unlit: true,
            cull_mode: None,
            ..default()
        }),
        transform: Transform::from_scale(Vec3::splat(1_000_000.0)),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        point_light: PointLight {
            intensity: 1500.,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn update_system(
    mut camera: Query<(&mut FogSettings, &mut Transform)>,
    time: Res<Time>,
) {
    let now = time.elapsed_seconds();

    let ( .., mut transform) = camera.single_mut();

    // Orbit camera around pyramid
    let orbit_scale = 8.0 + (now / 10.0).sin() * 7.0;
    *transform = Transform::from_xyz(
        (now / 5.0).cos() * orbit_scale,
        12.0 - orbit_scale / 2.0,
        (now / 5.0).sin() * orbit_scale,
    )
    .looking_at(Vec3::ZERO, Vec3::Y);
}