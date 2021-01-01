#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::Yantra;
use brahma_yantra::Event as YantraEvent;
use super::super::super::super::events::Event as LogicEvent;

#[derive(Default)]
pub(crate) struct Lane;

pub(crate) fn system(
    events: Res<Events<YantraEvent::OnEnter>>,
    mut reader: Local<EventReader<YantraEvent::OnEnter>>,

    query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        if let Ok(e) = query.get(ev.target) {
            println!("Now Entered Start State {}", e.id());
        }
    }
}
