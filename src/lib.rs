#[path = "ui/lib.rs"]
mod editor;

mod state_events;
mod traits;

use bevy::{app::prelude::*, window};
use bevy::ecs::IntoQuerySystem;

#[derive(Default)]
pub struct BrahmaPlugin;

impl Plugin for BrahmaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(editor::BrahmaEditorPlugin::default());
    }
}