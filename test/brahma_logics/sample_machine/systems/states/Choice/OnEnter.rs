#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::Yantra;
use brahma_yantra::Event as YantraEvent;
use super::super::super::super::events::Event as LogicEvent;

pub(crate) struct Lane {
    pub owner: Entity,
}

pub(crate) fn system(
    events: Res<Events<YantraEvent::OnEnter>>,
    mut reader: Local<EventReader<YantraEvent::OnEnter>>,

    mut yantra: ResMut<Yantra>,

    query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        if let Ok(e) = query.get(ev.target) {
            println!("Now Entered Choice State Lane {}", ev.target.id());
        }
    }
}
