use std::collections::{HashMap, HashSet};

use bevy::{ecs::Entity};

use crate::*;

#[derive(Default)]
pub struct Yantra {
    // lane_holder
    pub(crate) yantra_entity: Option<Entity>,

    // machine
    machine_is_ready: HashMap<Entity, bool>,
    machine_to_holder: HashMap<Entity, Entity>,
    machine_to_data: HashMap<Entity, YantraMachineData>,

    // buffers
    init_buffer: Vec<YantraMachineBuilder>,
    deinit_buffer: Vec<Entity>,
    // owner
    lane_to_owner_machine: HashMap<Entity, Entity>,
}

impl Yantra {
    //publics
    pub fn is_machine_ready(&self, machine: Entity) -> bool {
        return *self.machine_is_ready.get(&machine).unwrap_or(&false);
    }
    pub fn init_machine(&mut self, machine_info: YantraMachineBuilder) {
        self.init_buffer.push(machine_info);
    }

    pub fn deinit_machine(&mut self, machine: Entity) {
        self.deinit_buffer.push(machine);
    }

    pub fn switch_to_state(&mut self, machine: Entity, state: YantraState) {
        if let Some(data) = self.machine_to_data.get(&machine) {}
    }

    fn start_state(&mut self, machine: Entity, state: YantraState) {}
    fn stop_state(&mut self, machine: Entity, state: YantraState) {}
    pub fn get_yantra_entity(&self) -> Entity {
        self.yantra_entity.unwrap()
    }
    pub fn get_owner_for_lane(&self, lane: Entity) -> Option<&Entity> {
        self.lane_to_owner_machine.get(&lane)
    }
    // pub fn get_machine_data(
    //     &self,
    //     machine: Entity,
    // ) -> Option<&YantraMachineData> {
    //     self.machine_to_data.get(&machine)
    // }

    // private
    fn get_current_buffer(frame_number: u32) -> u32 {
        return frame_number % 2;
    }

    // callback
    pub(crate) fn on_initialise_lanes(
        mut commands: &mut Commands,
        mut yantra: ResMut<Yantra>,
    ) {
        while yantra.init_buffer.len() > 0 {
            let mach_builder = yantra.init_buffer.pop().unwrap();

            // machine
            let machine_entity = commands
                .spawn((YantraMachineEntitiesHolder,))
                .with(Parent(yantra.get_yantra_entity()))
                .current_entity()
                .unwrap();

            yantra
                .machine_to_holder
                .insert(mach_builder.owner_entity, machine_entity);

            // lanes
            for _lane_state in mach_builder.state_lane_tags {
                for _build in _lane_state {
                    let ent_lane = _build(commands)
                        .with(Parent(machine_entity))
                        .current_entity()
                        .unwrap();
                    println!(
                        "Added State Lane {} to parent {}",
                        ent_lane.id(),
                        machine_entity.id()
                    );
                }
            }
            for _lane_transition in mach_builder.transition_lane_tags {
                for _build in _lane_transition {
                    let ent_lane = _build(commands)
                        .with(Parent(machine_entity))
                        .current_entity()
                        .unwrap();
                    println!(
                        "Added Transition Lane {} to parent {}",
                        ent_lane.id(),
                        machine_entity.id()
                    );
                }
            }

            yantra
                .machine_is_ready
                .insert(mach_builder.owner_entity, true);
        }
    }
}
