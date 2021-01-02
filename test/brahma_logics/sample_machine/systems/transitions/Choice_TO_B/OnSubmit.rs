#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::Yantra;
use brahma_yantra::Event as YantraEvent;
use super::super::super::super::events::Event as LogicEvent;

#[derive(Default)]
pub(crate) struct Lane;

pub(crate) fn system(
    events: Res<Events<LogicEvent::OnSubmit>>,
    mut reader: Local<EventReader<LogicEvent::OnSubmit>>,

    mut yantra: ResMut<Yantra>,
    query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        // let target = ev.target;
        for entity in query.iter() {
            if ev.input == "B" {
                println!("Transitioning To B");
                yantra.transition(entity);
            }
            // if entity == target {
            //     println!("Transitioning To Choice");
            //     yantra.transition(entity);
            // }
        }
    }
}
