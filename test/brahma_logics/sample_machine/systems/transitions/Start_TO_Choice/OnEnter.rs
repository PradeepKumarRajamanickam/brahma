#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::*;

use super::*;

#[derive(Default)]
pub struct Lane;

#[allow(non_snake_case)]
pub fn system(
    events: Res<Events<brahma_yantra::EventOnEnter>>,
    mut reader: Local<EventReader<brahma_yantra::EventOnEnter>>,

    mut yantra: ResMut<Yantra>,
    mut query: Query<Entity, (With<Lane>, With<YantraLaneActive>)>,
) {
    for ev in reader.iter(&events) {
        let target = ev.target;
        println!("Now Entered Start State {}", ev.target.id());

        for entity in query.iter() {
            if let Some(owner_machine) = yantra.get_owner_for_lane(entity) {}
        }
    }
}
