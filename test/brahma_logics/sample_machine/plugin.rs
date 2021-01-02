use bevy::app::{prelude::*, startup_stage};
use bevy::ecs::IntoSystem;
use super::{events::*, machine, systems::*};

#[derive(Default)]
pub struct BrahmaLogic;

impl BrahmaLogic {
    pub const LOGIC_ID: u64 = 0001; // assigned during code gen
    pub const LOGIC_NAME: &'static str = "sample_machine";
}

impl Plugin for BrahmaLogic {
    fn build(&self, app: &mut AppBuilder) {
        app
            // machine
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_ADDED,
                machine::on_machine_added.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_REMOVED,
                machine::on_machine_removed.system(),
            )
            // events
            .add_event::<Event::OnSubmit>()
            // state lanes
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                states::Start::OnEnter::system.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                states::Start::OnExit::system.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                states::Choice::OnEnter::system.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                states::Choice::OnUpdate::system.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                states::A::OnEnter::system.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                states::A::OnExit::system.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                states::B::OnEnter::system.system(),
            )
            // transition lanes
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                transitions::Start_TO_Choice::OnEnter::system.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                transitions::Choice_TO_A::OnSubmit::system.system(),
            )
            .add_system_to_stage(
                brahma_yantra::YANTRA_MACHINE_UPDATE,
                transitions::Choice_TO_B::OnSubmit::system.system(),
            );

        println!(
            "BrahmaLogic: Initialised Systems for Logic: {}, \tId: {} ",
            BrahmaLogic::LOGIC_NAME,
            BrahmaLogic::LOGIC_ID
        );
    }
}
