#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::Yantra;
use brahma_yantra::Event as YantraEvent;
use super::super::super::super::events::Event as LogicEvent;

// customs
use text_io::read;

#[derive(Default)]
pub(crate) struct Lane;

pub(crate) fn system(
    events: Res<Events<YantraEvent::OnUpdate>>,
    mut reader: Local<EventReader<YantraEvent::OnUpdate>>,

    mut logic_event_OnSubmit: ResMut<Events<LogicEvent::OnSubmit>>,
    query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        if let Ok(e) = query.get(ev.target) {
            println!("Now Updating Choice State Lane {}", e.id());

            println!("Enter Your Choice A or B ");
            let input: String = read!();
            println!("User input {} ", input);
            logic_event_OnSubmit.send(LogicEvent::OnSubmit {
                input: input.to_uppercase(),
            });
        }
    }
}
