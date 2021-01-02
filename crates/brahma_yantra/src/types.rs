use std::{collections::HashMap};

use bevy::{
    ecs::{Commands, Entity},
};

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct YantraState(pub u64);

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct YantraTransition(pub u64);

#[derive(Default)]
pub(crate) struct YantraMachineData {
    pub start_state: YantraState,
    pub current_state: Option<YantraState>,
    pub state_owned_lanes: HashMap<YantraState, Vec<Entity>>,
    pub transition_target: HashMap<YantraTransition, YantraState>,
}

// builder
pub type CommandClosure =
    fn(commands: &mut Commands, machine: Entity) -> &mut Commands;
pub struct YantraMachineBuilder {
    // meta
    pub logic_id: u64,
    pub logic_name: String,

    // data
    pub enabled: bool,
    pub owner_entity: Entity,

    pub start: YantraState,
    pub states: Vec<YantraState>,
    pub state_lane_tags: Vec<Vec<CommandClosure>>,

    pub transitions: Vec<YantraTransition>,
    pub transition_lane_tags: Vec<Vec<CommandClosure>>,
    pub transition_state_owner: HashMap<YantraTransition, YantraState>,
    pub transition_target: HashMap<YantraTransition, YantraState>,
}
