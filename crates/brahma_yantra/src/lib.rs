pub use self::{components::*, stages::*, yantra::*, events::*, utils::*, types::*};

use bevy::prelude::*;

mod components;
mod stages;
mod yantra;
mod events;
mod utils;
mod types;
mod systems;

#[derive(Default)]
pub struct YantraPlugin;

impl Plugin for YantraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // stages
            .add_startup_stage_before(
                stage::STARTUP,
                YANTRA_SETUP,
                SystemStage::parallel(),
            )
            .add_stage_before(
                stage::UPDATE,
                YANTRA_MACHINE_ADDED,
                SystemStage::parallel(),
            )
            .add_stage_after(
                stage::UPDATE,
                YANTRA_MACHINE_UPDATE,
                SystemStage::parallel(),
            )
            .add_stage_after(
                YANTRA_MACHINE_UPDATE,
                YANTRA_MACHINE_REMOVED,
                SystemStage::parallel(),
            )
            // resources
            .add_resource(Yantra::default())
            // events
            .add_event::<EventOnEnter>()
            .add_event::<EventOnUpdate>()
            .add_event::<EventOnExit>()
            // systems
            .add_startup_system_to_stage(
                YANTRA_SETUP,
                YantraPlugin::setup.system(),
            )
            .add_system_to_stage(
                YANTRA_MACHINE_UPDATE,
                Yantra::on_initialise_lanes.system(),
            )
            .add_system_to_stage(
                YANTRA_MACHINE_UPDATE,
                Yantra::on_deinitialise_lanes.system(),
            );
    }
}
impl YantraPlugin {
    fn setup(mut commands: &mut Commands, mut yantra: ResMut<Yantra>) {
        let ent_yantra_holder = commands
            .spawn((Yantra::default(),))
            .current_entity()
            .unwrap();
        yantra.yantra_entity = Some(ent_yantra_holder);
        println!("YantraPlugin: Initialised");
    }
}
