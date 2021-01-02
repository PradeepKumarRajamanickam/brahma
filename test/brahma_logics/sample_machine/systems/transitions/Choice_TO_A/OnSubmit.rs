#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::Yantra;
use brahma_yantra::Trait::Owner;
use brahma_yantra::Event as YantraEvent;
use super::super::super::super::events::Event as LogicEvent;

pub(crate) struct Lane {
    pub owner: Entity,
}

// local event
pub(crate) fn system(
    events: Res<Events<LogicEvent::OnSubmit>>,
    mut reader: Local<EventReader<LogicEvent::OnSubmit>>,

    mut yantra: ResMut<Yantra>,
    query: Query<(Entity, &Lane), With<Lane>>,
) {
    for ev in reader.iter(&events) {
        let target_machine: Entity = ev.get_owner();
        for (entity, lane) in query.iter() {
            if target_machine == lane.owner {
                if ev.input == "A" {
                    println!(
                        "Transitioning To A, on Machine {} ",
                        yantra.get_owner_for_lane(entity).unwrap().id()
                    );
                    yantra.transition(entity);
                }
            }
        }
    }
}
