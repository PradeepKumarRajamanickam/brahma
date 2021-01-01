use bevy::app::prelude::*;
use brahma_yantra::YantraPlugin;

// generated
use super::{sample_machine};

#[derive(Default)]
pub struct BrahmaBootStrap;
impl Plugin for BrahmaBootStrap {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(YantraPlugin::default())
            .add_plugin(sample_machine::BrahmaLogic::default());

        println!("YantraPluginBootStrap: Done");
    }
}
