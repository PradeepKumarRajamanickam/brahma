use crate::*;

impl Yantra {
    // crates
    pub(crate) fn on_initialise_lanes(
        commands: &mut Commands,
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
                    let ent_lane = _build(commands, owner_entity)
                        .with(Parent(machine_entity))
                        .current_entity()
                        .unwrap();

                    ent_lane_vec.push(ent_lane);
                    yantra.lane_to_owner_machine.insert(ent_lane, owner_entity);

                    if yantra.log_enabled && yantra.log_verbose {
                        println!(
                            "Added State Lane {} for Machine {}",
                            ent_lane.id(),
                            owner_entity.id()
                        );
                    }
                }
                machine_data
                    .state_owned_lanes
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
                    let ent_lane = _build(commands, owner_entity)
                        .with(Parent(machine_entity))
                        .current_entity()
                        .unwrap();

                    ent_lane_vec.push(ent_lane);
                    yantra.lane_to_owner_machine.insert(ent_lane, owner_entity);
                    yantra
                        .lane_to_owner_transition
                        .insert(ent_lane, yantra_transition);

                    if yantra.log_enabled && yantra.log_verbose {
                        println!(
                            "Added Transition Lane {} for Machine {}",
                            ent_lane.id(),
                            owner_entity.id()
                        );
                    }
                }

                let ent_vec = machine_data
                    .state_owned_lanes
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
        commands: &mut Commands,
        mut yantra: ResMut<Yantra>,
    ) {
        while yantra.deinit_buffer.len() > 0 {
            let owner_entity = yantra.deinit_buffer.pop().unwrap();
            let entity =
                yantra.machine_owner_to_holder.get(&owner_entity).unwrap();
            commands.despawn_recursive(*entity);
        }
    }

    pub(crate) fn on_yantra_lane_events(
        mut yantra: ResMut<Yantra>,
        mut event_on_enter: ResMut<Events<Event::OnEnter>>,
        mut event_on_update: ResMut<Events<Event::OnUpdate>>,
        mut event_on_exit: ResMut<Events<Event::OnExit>>,
    ) {
        while yantra.lane_on_enter_buffer.len() > 0 {
            let entity = yantra.lane_on_enter_buffer.pop().unwrap();
            event_on_enter.send(Event::OnEnter { target: entity });
        }

        for entity in yantra.running_lanes.iter() {
            event_on_update.send(Event::OnUpdate { target: *entity });
        }

        while yantra.lane_on_exit_buffer.len() > 0 {
            let entity = yantra.lane_on_exit_buffer.pop().unwrap();
            event_on_exit.send(Event::OnExit { target: entity });
        }
    }
}
