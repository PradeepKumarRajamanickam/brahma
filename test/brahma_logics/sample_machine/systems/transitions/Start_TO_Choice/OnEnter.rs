#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::*;

use super::*;

#[derive(Default)]
pub(crate) struct Lane;

pub(crate) fn system(
    events: Res<Events<Event::OnEnter>>,
    mut reader: Local<EventReader<Event::OnEnter>>,

    mut yantra: ResMut<Yantra>,
    query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        let target = ev.target;
        for entity in query.iter() {
            if entity == target {
                println!(
                    "Now Entered Transition Start TO Choice State {}",
                    target.id()
                );
                println!("Transitioning To Choice");
                yantra.transition(entity);
            }
        }
    }
}
