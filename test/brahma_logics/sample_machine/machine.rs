#![allow(non_snake_case)]

use std::collections::HashMap;

use bevy::{ecs::*, prelude::*};
use brahma_yantra::*;

use systems::*;
use crate::brahma_logics::sample_machine::*;

use super::*;

pub struct Machine {
    pub enabled: bool,
}

impl Default for Machine {
    fn default() -> Self {
        Machine { enabled: true }
    }
}

// machine
pub(crate) fn on_machine_added(
    mut commands: &mut Commands,
    mut yantra: ResMut<Yantra>,
    query: Query<(Entity, &Machine), Added<Machine>>,
) {
    for (ent_machine, machine) in query.iter() {
        // **********
        // * STATES *
        // **********

        let STATE_Start = YantraState(1);
        let STATE_Choice = YantraState(2);
        let STATE_A = YantraState(3);
        let STATE_B = YantraState(4);

        let states = vec![STATE_Start, STATE_Choice, STATE_A, STATE_B];

        // ***************
        // * TRANSITIONS *
        // ***************

        // * Start > TO > Choice
        let TRANSITION_Start_TO_Choice = YantraTransition(3);
        let TRANSITION_Choice_TO_A = YantraTransition(4);
        let TRANSITION_Choice_TO_B = YantraTransition(5);

        let transitions = vec![
            TRANSITION_Start_TO_Choice,
            TRANSITION_Choice_TO_A,
            TRANSITION_Choice_TO_B,
        ];

        // *********
        // * LANES *
        // *********
        let state_lane_tags: Vec<Vec<CommandClosure>> = vec![
            // commands
            vec![|c| c.spawn((states::Start::OnEnter::Lane::default(),))],
            vec![|c| c.spawn((states::Choice::OnEnter::Lane::default(),))],
            vec![|c| c.spawn((states::A::OnEnter::Lane::default(),))],
            vec![|c| c.spawn((states::B::OnEnter::Lane::default(),))],
        ];

        let transition_lane_tags: Vec<Vec<CommandClosure>> = vec![
            // commands
            vec![|c| {
                c.spawn((
                    transitions::Start_TO_Choice::OnEnter::Lane::default(),
                    transitions::Choice_TO_A::OnSubmit::Lane::default(),
                    transitions::Choice_TO_B::OnSubmit::Lane::default(),
                ))
            }],
        ];

        // *****************
        // * RELATIONSHIPS *
        // *****************

        let YANTRA_Start = STATE_Start;

        // * Transition target state to switch to
        let mut hmap_trans_targets = HashMap::new();

        // * Start > To > Choice
        hmap_trans_targets.insert(TRANSITION_Start_TO_Choice, STATE_Choice);
        hmap_trans_targets.insert(TRANSITION_Choice_TO_A, STATE_A);
        hmap_trans_targets.insert(TRANSITION_Choice_TO_B, STATE_B);

        // * Transitions associated to state
        let mut hmap_trans_owner = HashMap::new();
        hmap_trans_owner.insert(TRANSITION_Start_TO_Choice, STATE_Start);
        hmap_trans_owner.insert(TRANSITION_Choice_TO_A, STATE_Choice);
        hmap_trans_owner.insert(TRANSITION_Choice_TO_B, STATE_Choice);

        // **************
        // * INITIALISE *
        // **************

        yantra.init_machine(brahma_yantra::YantraMachineBuilder {
            logic_id: BrahmaLogic::LOGIC_ID,
            logic_name: BrahmaLogic::LOGIC_NAME.to_string(),
            start: YANTRA_Start,
            enabled: machine.enabled,
            owner_entity: ent_machine,
            state_lane_tags,
            transition_lane_tags,
            states,
            transitions,
            transition_target: hmap_trans_targets,
            transition_state_owner: hmap_trans_owner,
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
