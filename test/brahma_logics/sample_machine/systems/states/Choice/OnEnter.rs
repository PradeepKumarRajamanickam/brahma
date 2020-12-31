#![allow(non_snake_case)]
use bevy::{prelude::*, ecs::*};
use brahma_yantra::*;

use super::*;

#[derive(Default)]
pub struct Lane;

pub fn system(
    events: Res<Events<brahma_yantra::EventOnEnter>>,
    mut reader: Local<EventReader<brahma_yantra::EventOnEnter>>,

    yantra: Res<Yantra>,
    mut query: Query<Entity, With<Lane>>,
) {
    for ev in reader.iter(&events) {
        println!("Now Entered Choice State {}", ev.target.id());
    }
}
