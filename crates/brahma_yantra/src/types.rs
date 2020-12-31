use std::{sync::Arc, collections::HashMap};

use bevy::{
    ecs::{Bundle, Commands, Component, DynamicBundle, Entity},
};

// #[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
// pub struct YantraMachine(pub u64);

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct YantraState(pub u64);

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct YantraTransition(pub u64);

pub struct YantraMachineBuilder {
    pub owner_entity: Entity,

    pub states: Vec<YantraState>,
    pub state_lane_tags: Vec<Vec<CommandClosure>>,
    pub state_transitions: HashMap<YantraState, Vec<YantraTransition>>,

    pub transitions: Vec<YantraTransition>,
    pub transition_lane_tags: Vec<Vec<CommandClosure>>,
    pub transition_target: HashMap<YantraTransition, YantraState>,
}

pub(crate) struct YantraMachineData {
    pub current_state: Option<YantraState>,
    pub state_owned_lane: HashMap<YantraState, Vec<Entity>>,
    pub state_transitions: HashMap<YantraState, Vec<YantraTransition>>,
    pub transition_target: HashMap<YantraTransition, YantraState>,
}

pub type CommandClosure = fn(commands: &mut Commands) -> &mut Commands;
