use bevy::prelude::*;

use crate::entities::{ball, camera, platform};

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    ball::setup(&mut commands, &mut meshes, &mut materials);
    platform::setup(&mut commands, &mut meshes, &mut materials);
    camera::setup(&mut commands);
    
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
        transform: Transform::from_xyz(7.0, 8.0, 7.0),
        point_light: PointLight {
            intensity: 1500.,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

