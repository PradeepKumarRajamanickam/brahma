#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::*;

use super::*;

#[derive(Default)]
pub(crate) struct Lane;

pub(crate) fn system(
    events: Res<Events<brahma_yantra::EventOnEnter>>,
    mut reader: Local<EventReader<brahma_yantra::EventOnEnter>>,
    mut query: Query<Entity, (With<Lane>, With<YantraLaneActive>)>,
) {
    for ev in &mut reader.iter(&events) {
        if let Ok(e) = query.get(ev.target) {
            println!("Now Entered Start State {}", e.id());
        }
    }
}
