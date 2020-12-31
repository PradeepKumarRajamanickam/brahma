use bevy::{ecs::Entity, utils::HashMap};

use crate::*;

// fsm events
pub struct EventOnEnter {
    pub target: Entity,
}
pub struct EventOnUpdate {
    pub target: Entity,
}
pub struct EventOnExit {
    pub target: Entity,
}

// pub struct EventMachineReady {
//     pub machine: Entity,
// }
