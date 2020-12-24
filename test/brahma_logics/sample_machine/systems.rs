use bevy::ecs::*;
use brahma_yantra::Yantra;

use super::*;

// machine
pub fn on_machine_added(
    // mut commands: Commands,
    yantra: Res<Yantra>,
    mut query: Query<(Entity, Added<Machine>)>,
) {
    for (entity, machine) in query.iter() {
        println!("Machine Added {}", entity.id());
    }
}
pub fn on_machine_removed(
    mut commands: Commands,
    yantra: Res<Yantra>,
    mut query: Query<Changed<Machine>>,
) {
    for entity in query.removed::<Machine>() {
        println!("Machine Removed");
    }
}
// States
pub fn STATE_Start_LANE_OnEnter() {
    println!("Now Entered Start State");
}

// Transits
pub fn Transit_Start_TO_Choice_LANE_OnEnter() {
    println!("Now Entered: Transit Start to Choice");
}
