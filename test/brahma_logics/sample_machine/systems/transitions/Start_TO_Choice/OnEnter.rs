#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::*;

use super::*;

#[derive(Default)]
pub struct Lane;

pub fn system(
    events: Res<Events<brahma_yantra::EventOnEnter>>,
    mut reader: Local<EventReader<brahma_yantra::EventOnEnter>>,

    mut yantra: ResMut<Yantra>,
    mut query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        let target = ev.target;
        for entity in query.iter() {
            if entity == target {
                println!(
                    "Now Entered Transition Start TO Choice State {}",
                    target.id()
                );
                println!("Transitioning To Choice");
                yantra.transition(entity);
            }
        }
    }
}
