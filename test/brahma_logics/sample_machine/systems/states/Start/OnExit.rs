#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::Yantra;
use brahma_yantra::Event as YantraEvent;
use super::super::super::super::events::Event as LogicEvent;

pub(crate) struct Lane {
    pub owner: Entity,
}
pub(crate) fn system(
    events: Res<Events<YantraEvent::OnExit>>,
    mut reader: Local<EventReader<YantraEvent::OnExit>>,

    mut yantra: ResMut<Yantra>,

    query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        if let Ok(e) = query.get(ev.target) {
            println!("Now Exited Start State {}", ev.target.id());

            let machine = yantra.get_owner_for_lane(e).unwrap().clone();
            yantra.stop(machine);
        }
    }
}
