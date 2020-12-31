use bevy::app::prelude::*;
use brahma_yantra::YantraPlugin;

// generated
use super::{sample_machine, sample_machine_1};

#[derive(Default)]
pub struct BrahmaBootStrap;
impl Plugin for BrahmaBootStrap {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(YantraPlugin::default())
            .add_plugin(sample_machine::BrahmaLogic::default())
            .add_plugin(sample_machine_1::BrahmaLogic::default());

        println!("YantraPluginBootStrap: Done");
    }
}
