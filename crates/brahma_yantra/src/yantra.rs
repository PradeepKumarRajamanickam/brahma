use std::collections::{HashMap, HashSet};

use bevy::{ecs::Entity};

use crate::*;

#[derive(Default)]
pub struct Yantra {
    // lane_holder
    pub(crate) yantra_entity: Option<Entity>,
    // buffers
    pub(crate) init_buffer: Vec<YantraMachineBuilder>,
    pub(crate) deinit_buffer: Vec<Entity>,
    pub(crate) lane_on_exit_buffer: Vec<Entity>,
    pub(crate) lane_on_enter_buffer: Vec<Entity>,
    // pub(crate) lane_event_buffer: Vec<[Local,>,

    // privates
    // machine
    pub(crate) machine_is_ready: HashSet<Entity>,
    pub(crate) machine_is_running: HashSet<Entity>,
    pub(crate) machine_to_data: HashMap<Entity, YantraMachineData>,
    pub(crate) machine_owner_to_holder: HashMap<Entity, Entity>,

    // lanes
    pub(crate) running_lanes: HashSet<Entity>,

    // owner
    pub(crate) lane_to_owner_machine: HashMap<Entity, Entity>,
    pub(crate) lane_to_owner_transition: HashMap<Entity, YantraTransition>,
}

impl Yantra {
    //publics
    pub fn is_same_machine(&self, lane_a: Entity, lane_b: Entity) -> bool {
        let lane_a_machine = *self.get_owner_for_lane(lane_a).unwrap();
        let lane_b_machine = *self.get_owner_for_lane(lane_b).unwrap();
        return lane_a_machine == lane_b_machine;
    }
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
            if let Some(lanes) = data.state_owned_lanes.get(&state) {
                data.current_state = Some(state);
                self.running_lanes.extend(lanes);
                self.lane_on_enter_buffer.extend(lanes);
                println!(
                    "Machine {} Lanes started running {:?} ",
                    machine.id(),
                    lanes.iter().map(|x| x.id()).collect::<Vec<u32>>(),
                );
            }

            println!(
                "Machine {} State started running {} ",
                machine.id(),
                state.0,
            )
        }
    }

    pub fn stop_running_state(&mut self, machine: Entity) {
        if let Some(data) = self.machine_to_data.get_mut(&machine) {
            match data.current_state {
                Some(state) => {
                    if let Some(lanes) = data.state_owned_lanes.get(&state) {
                        self.lane_on_exit_buffer.extend(lanes);
                        for l in lanes {
                            self.running_lanes.remove(&l);
                        }

                        println!(
                            "Machine {} Lanes stopped running {:?} ",
                            machine.id(),
                            lanes.iter().map(|x| x.id()).collect::<Vec<u32>>(),
                        );
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

    // privates
    #[allow(dead_code)]
    fn get_current_buffer(frame_number: u32) -> u32 {
        return frame_number % 2;
    }
}
