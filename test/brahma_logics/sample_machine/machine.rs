#![allow(non_snake_case)]

use std::collections::HashMap;

use bevy::{ecs::*, prelude::*};
use brahma_yantra::*;

use systems::*;
use crate::brahma_logics::sample_machine::*;

use super::*;

#[derive(Default)]
pub struct Machine;

// machine
pub(crate) fn on_machine_added(
    mut commands: &mut Commands,
    mut yantra: ResMut<Yantra>,
    query: Query<Entity, Added<Machine>>,
) {
    for ent_machine in query.iter() {
        // **********
        // * STATES *
        // **********

        let STATE_Start = YantraState(1);
        let STATE_Choice = YantraState(2);

        let states = vec![STATE_Start, STATE_Choice];

        // ***************
        // * TRANSITIONS *
        // ***************

        // * Start > TO > Choice
        let TRANSITION_Start_TO_Choice = YantraTransition(3);
        let TRANSITION_Start_TO_Choice_1 = YantraTransition(4);

        let transitions =
            vec![TRANSITION_Start_TO_Choice, TRANSITION_Start_TO_Choice_1];

        // *********
        // * LANES *
        // *********
        let state_lane_tags: Vec<Vec<CommandClosure>> = vec![
            // commands
            vec![|c| c.spawn((states::Start::OnEnter::Lane::default(),))],
            vec![|c| c.spawn((states::Choice::OnEnter::Lane::default(),))],
        ];

        let transition_lane_tags: Vec<Vec<CommandClosure>> = vec![
            // commands
            vec![|c| {
                c.spawn((
                    transitions::Start_TO_Choice::OnEnter::Lane::default(),
                ))
            }],
        ];

        // *****************
        // * RELATIONSHIPS *
        // *****************

        // * Transition target state to switch to
        let mut hmap_transit_targets = HashMap::new();

        // * Start > To > Choice
        hmap_transit_targets.insert(TRANSITION_Start_TO_Choice, STATE_Choice);
        hmap_transit_targets.insert(TRANSITION_Start_TO_Choice_1, STATE_Choice);

        // * Transitions associated with states
        let mut hmap_state_trans = HashMap::new();
        hmap_state_trans
            .insert(STATE_Start.clone(), vec![TRANSITION_Start_TO_Choice]);

        // **************
        // * INITIALISE *
        // **************

        yantra.init_machine(brahma_yantra::YantraMachineBuilder {
            logic_id: BrahmaLogic::LOGIC_ID,
            logic_name: BrahmaLogic::LOGIC_NAME.to_string(),
            owner_entity: ent_machine,
            state_lane_tags,
            transition_lane_tags,
            states,
            transitions,
            transition_target: hmap_transit_targets,
            state_transitions: hmap_state_trans,
        });
    }
}
pub(crate) fn on_machine_removed(
    mut commands: &mut Commands,
    yantra: Res<Yantra>,
    mut query: Query<Entity, Changed<Machine>>,
) {
    for entity in query.removed::<Machine>() {
        println!(
            "{} Machine DeInitialised {}",
            BrahmaLogic::LOGIC_NAME,
            entity.id()
        );
    }
}
