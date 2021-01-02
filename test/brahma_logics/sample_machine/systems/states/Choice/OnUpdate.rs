#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::Yantra;
use brahma_yantra::Event as YantraEvent;
use super::super::super::super::events::Event as LogicEvent;

// customs
use text_io::read;

pub(crate) struct Lane {
    pub owner: Entity,
}

pub(crate) fn system(
    events: Res<Events<YantraEvent::OnUpdate>>,
    mut reader: Local<EventReader<YantraEvent::OnUpdate>>,

    mut logic_event_OnSubmit: ResMut<Events<LogicEvent::OnSubmit>>,
    query: Query<(Entity, &Lane), With<Lane>>,
) {
    for ev in reader.iter(&events) {
        if let Ok((e, lane)) = query.get(ev.target) {
            println!("Now Updating Choice State Lane {}", e.id());

            println!("Enter Your Choice A or B ");
            let input: String = read!();
            println!("User input {} ", input);
            logic_event_OnSubmit.send(LogicEvent::OnSubmit {
                owner: lane.owner,
                input: input.to_uppercase(),
            });
        }
    }
}
