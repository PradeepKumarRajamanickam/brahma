use bevy::app::prelude::*;
use bevy::ecs::IntoQuerySystem;

#[derive(Default)]
pub struct BrahmaLogic;

impl BrahmaLogic {
    pub const LOGIC_ID: u64 = 0002; // assigned during code gen
    pub const LOGIC_NAME: &'static str = "sample_machine_2";
}

impl Plugin for BrahmaLogic {
    fn build(&self, app: &mut AppBuilder) {
        println!(
            "BrahmaLogic: Initialised Systems for Logic: {}, \tId: {} ",
            BrahmaLogic::LOGIC_NAME,
            BrahmaLogic::LOGIC_ID
        );
    }
}
