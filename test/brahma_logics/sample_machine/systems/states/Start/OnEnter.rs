#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::*;

use super::*;

#[derive(Default)]
pub(crate) struct Lane;

pub(crate) fn system(
    events: Res<Events<Event::OnEnter>>,
    mut reader: Local<EventReader<Event::OnEnter>>,

    query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        if let Ok(e) = query.get(ev.target) {
            println!("Now Entered Start State {}", e.id());
        }
    }
}
