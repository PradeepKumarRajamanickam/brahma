use std::collections::{HashMap, HashSet};

use bevy::{ecs::Entity};

use crate::*;

#[derive(Default)]
pub struct Yantra {
    // lane_holder
    pub(crate) yantra_entity: Option<Entity>,

    // machine
    machine_is_ready: HashSet<Entity>,
    machine_is_running: HashSet<Entity>,
    machine_to_data: HashMap<Entity, YantraMachineData>,
    machine_owner_to_holder: HashMap<Entity, Entity>,

    // lanes
    running_lanes: HashSet<Entity>,

    // buffers
    init_buffer: Vec<YantraMachineBuilder>,
    deinit_buffer: Vec<Entity>,

    // owner
    lane_to_owner_machine: HashMap<Entity, Entity>,
    lane_to_owner_transition: HashMap<Entity, YantraTransition>,
}

impl Yantra {
    //publics
    pub fn is_machine_running(&self, machine: Entity) -> bool {
        return self.machine_is_running.contains(&machine);
    }
    pub fn is_machine_ready(&self, machine: Entity) -> bool {
        return self.machine_is_ready.contains(&machine);
    }
    pub fn init_machine(&mut self, machine_info: YantraMachineBuilder) {
        self.init_buffer.push(machine_info);
    }

    pub fn deinit_machine(&mut self, machine: Entity) {
        self.machine_is_ready.remove(&machine);
        self.machine_is_running.remove(&machine);
        self.machine_to_data.remove(&machine);
        self.deinit_buffer.push(machine);
    }

    pub fn start(&mut self, machine: Entity) {
        if !self.is_machine_ready(machine) {
            println!("Machine is not ready to start yet {}", machine.id());
            return;
        }
        if self.is_machine_running(machine) {
            println!(
                "Machine is already running. Start ignored {}",
                machine.id()
            );
            return;
        }
        if let Some(data) = self.machine_to_data.get(&machine) {
            println!("Machine has started {} ", machine.id());
            self.run_state(machine, data.start_state);
            self.machine_is_running.insert(machine);
        }
    }

    pub fn stop(&mut self, machine: Entity) {
        self.stop_running_state(machine);
        if self.machine_is_running.remove(&machine) {
            println!("Machine has stopped {} ", machine.id());
        } else {
            println!("Machine not running. Stop ignored {} ", machine.id());
        }
    }

    pub fn transition(&mut self, entity: Entity) {
        if let Some(machine) = self.get_owner_for_lane(entity) {
            let transition = self.get_transition_from_lane(entity).unwrap();
            if let Some(data) = self.machine_to_data.get(&machine) {
                if let Some(state) = data.transition_target.get(&transition) {
                    self.run_state(*machine, *state);
                }
            }
        }
    }

    pub fn run_state(&mut self, machine: Entity, state: YantraState) {
        self.stop_running_state(machine);

        if let Some(data) = self.machine_to_data.get_mut(&machine) {
            if let Some(lanes) = data.state_owned_lane.get(&state) {
                self.running_lanes.extend(lanes);
            }

            println!(
                "State started running {} for Machine {}",
                state.0,
                machine.id()
            )
        }
    }

    pub fn stop_running_state(&mut self, machine: Entity) {
        if let Some(data) = self.machine_to_data.get_mut(&machine) {
            match data.current_state {
                Some(state) => {
                    if let Some(lanes) = data.state_owned_lane.get(&state) {
                        for l in lanes {
                            self.running_lanes.remove(&l);
                        }
                    }

                    println!(
                        "State stopped {} for Machine {}",
                        state.0,
                        machine.id()
                    );
                }
                None => {
                    println!(
                        "No state running to stop for Machine {}",
                        machine.id()
                    );
                }
            }

            data.current_state = None;
        }
    }

    pub fn get_yantra_entity(&self) -> Entity {
        self.yantra_entity.unwrap()
    }
    pub fn get_owner_for_lane(&self, lane: Entity) -> Option<&Entity> {
        self.lane_to_owner_machine.get(&lane)
    }
    pub fn get_transition_from_lane(
        &self,
        lane: Entity,
    ) -> Option<&YantraTransition> {
        self.lane_to_owner_transition.get(&lane)
    }

    // crates
    pub(crate) fn on_initialise_lanes(
        mut commands: &mut Commands,
        mut yantra: ResMut<Yantra>,
    ) {
        while yantra.init_buffer.len() > 0 {
            let mach_builder = yantra.init_buffer.pop().unwrap();
            let owner_entity = mach_builder.owner_entity;

            // machine
            let machine_entity = commands
                .spawn((YantraMachineEntitiesHolder,))
                .with(Parent(yantra.get_yantra_entity()))
                .current_entity()
                .unwrap();

            yantra
                .machine_owner_to_holder
                .insert(owner_entity, machine_entity);

            println!(
                "Initialising Logic {} Id {} for Machine Instance {}",
                mach_builder.logic_name,
                mach_builder.logic_id,
                owner_entity.id()
            );

            let mut machine_data = YantraMachineData::default();

            // lanes
            let mut index = 0;
            for _lane_state in mach_builder.state_lane_tags {
                let yantra_state = mach_builder.states[index];
                let mut ent_lane_vec: Vec<Entity> = vec![];
                for _build in _lane_state {
                    let ent_lane = _build(commands)
                        .with(Parent(machine_entity))
                        .current_entity()
                        .unwrap();

                    ent_lane_vec.push(ent_lane);
                    yantra.lane_to_owner_machine.insert(ent_lane, owner_entity);

                    println!(
                        "Added State Lane {} for Machine {}",
                        ent_lane.id(),
                        owner_entity.id()
                    );
                }
                machine_data
                    .state_owned_lane
                    .insert(yantra_state, ent_lane_vec);
                index = index + 1;
            }

            let mut index = 0;
            for _lane_transition in mach_builder.transition_lane_tags {
                let yantra_transition = mach_builder.transitions[index];
                let yantra_state = mach_builder
                    .transition_state_owner
                    .get(&yantra_transition)
                    .unwrap();
                let mut ent_lane_vec: Vec<Entity> = vec![];
                for _build in _lane_transition {
                    let ent_lane = _build(commands)
                        .with(Parent(machine_entity))
                        .current_entity()
                        .unwrap();

                    ent_lane_vec.push(ent_lane);
                    yantra.lane_to_owner_machine.insert(ent_lane, owner_entity);

                    println!(
                        "Added Transition Lane {} for Machine {}",
                        ent_lane.id(),
                        owner_entity.id()
                    );
                }

                let ent_vec = machine_data
                    .state_owned_lane
                    .get_mut(yantra_state)
                    .unwrap();
                ent_vec.extend(ent_lane_vec);
                index = index + 1;
            }

            machine_data.start_state = mach_builder.start;
            machine_data.transition_target = mach_builder.transition_target;

            yantra.machine_to_data.insert(owner_entity, machine_data);
            yantra.machine_is_ready.insert(owner_entity);

            println!(
                "Initialisation completed for Machine Instance {}",
                owner_entity.id()
            );

            if mach_builder.enabled {
                yantra.start(owner_entity);
            }
        }
    }

    pub(crate) fn on_deinitialise_lanes(
        mut commands: &mut Commands,
        mut yantra: ResMut<Yantra>,
    ) {
        while yantra.deinit_buffer.len() > 0 {
            let owner_entity = yantra.deinit_buffer.pop().unwrap();
            let entity =
                yantra.machine_owner_to_holder.get(&owner_entity).unwrap();
            commands.despawn_recursive(*entity);
        }
    }

    // privates
    fn get_current_buffer(frame_number: u32) -> u32 {
        return frame_number % 2;
    }
}
