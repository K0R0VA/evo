use bevy::ecs::{entity::Entity, event::EventReader, query::{With, WorldQuery}, system::Query};
use bevy_rapier3d::pipeline::CollisionEvent;

use crate::entities::{ball::{Ball, BallCollisionState}, platform::Platform};


pub fn collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut ball_query: Query<(&mut BallCollisionState, With<Ball>)>,
    platform_query: Query<(Entity, With<Platform>)>
) {
    let events_stream = collision_events
        .read();
    for collision_event in events_stream {
        if !is_this_entities(collision_event, &ball_query,&platform_query) { continue; }
        let (a, b, is_started) = match collision_event {
            CollisionEvent::Started(a, b, _) => (*a, *b, true),
            CollisionEvent::Stopped(a, b, _) => (*a, *b, false)
        };
        update_query_item_state(&mut ball_query, a, b, |(mut state, _)| {
            state.on_platform = is_started;
        });
    }
}

fn is_this_entities<A: WorldQuery, B: WorldQuery>(
    collision_event: &CollisionEvent,
    a_query: &Query<A>,
    b_query: &Query<B>
) -> bool {
    let (a, b) = match collision_event {
        CollisionEvent::Started(a, b, _) => (a, b),
        CollisionEvent::Stopped(a, b, _) => (a, b)
    };
    let f = a_query.get(*a).is_ok() && b_query.get(*b).is_ok();
    let s = a_query.get(*b).is_ok() && b_query.get(*a).is_ok();
    f || s
}
        
fn update_query_item_state<T: WorldQuery>(
    query: &mut Query<T>,
    a: Entity,
    b: Entity,
    f: impl Fn(T::Item<'_>),
)  {
    if let Ok(item) = query.get_mut(a) {
        f(item);
        return;
    }
    if let Ok(item) = query.get_mut(b) {
        f(item);
    }
}