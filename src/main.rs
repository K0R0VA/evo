use bevy::prelude::*;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::{DebugRenderMode, RapierDebugRenderPlugin}};
use entities::{snake, camera};

mod startup;
mod entities;
mod events;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..Default::default()
        })
        .add_systems(
            Startup,startup::startup)
        .add_systems(Update, (snake::jump, snake::move_by_keybord))
        .add_systems(Update, events::collision_events)
        .add_systems(Update, camera::look_at_snake)
        .run();
}