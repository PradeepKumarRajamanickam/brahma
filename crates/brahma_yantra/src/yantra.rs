use bevy::app::prelude::*;
use bevy::ecs::Entity;

#[derive(Default)]
pub struct Yantra {}

impl Yantra {
    pub fn register(machine: Entity) {}
    pub fn unregister(machine: Entity) {}

    pub fn start(fsm_node: Entity) {}

    pub fn stop(fsm_node: Entity) {}
}
