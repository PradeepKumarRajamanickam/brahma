
use bevy::app::{prelude::*};
use brahma_yantra;

use super::systems;

#[derive(Default)]
pub struct BrahmaLogic;

impl BrahmaLogic {
    pub const LOGIC_ID: u64 = 0001; // assigned during code gen
    pub const LOGIC_NAME: &'static str  = "sample_machine";
}

impl Plugin for BrahmaLogic
{
    fn build(&self, app: &mut AppBuilder) {

        app
        // .add_startup_system_to_stage(startup_stage::POST_STARTUP, systems::on_machine_added.system())
        // .add_startup_system_to_stage(startup_stage::POST_STARTUP, systems::on_machine_removed.system())
        .add_system(systems::on_machine_added.system())
        .add_system(systems::on_machine_removed.system())
        ;

        println!("BrahmaLogic: Initialised Systems for Logic: {}, \tId: {} ", 
        BrahmaLogic::LOGIC_NAME, 
        BrahmaLogic::LOGIC_ID);
    }
}