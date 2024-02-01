use bevy::prelude::*;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::{DebugRenderMode, RapierDebugRenderPlugin}};
use entities::ball;

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
        .add_systems(Update, (ball::jump, ball::move_by_keybord, events::collision_events))
        .run();
}