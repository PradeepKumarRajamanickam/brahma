mod node;
mod editor;
mod systems;
mod constants;
mod components;
mod events;
mod materials;

use editor::*;
use events::*;
use materials::*;
use components::*;

use bevy::{app::prelude::*};
use bevy::ecs::IntoQuerySystem;



#[derive(Default)]
pub struct BrahmaEditorPlugin;

impl Plugin for BrahmaEditorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        
        .add_startup_system(BrahmaEditor::setup.system())
        // .add_plugin(plugin)

        // resources
        .init_resource::<BrahmaEditor>()
        .init_resource::<BrahmaViewport>()
        .init_resource::<BrahmaMaterials>()

        // events
        .add_event::<OnNodeSelectedEvent>()
        .add_event::<OnNodeDeleteEvent>()
        .add_event::<OnCreateNewNodeEvent>()
        .add_event::<OnCreateNewViewportEvent>()

        // user event systems
        .add_system(BrahmaEditor::on_create_new_viewport.system())
        .add_system(BrahmaEditor::on_create_new_node.system())
        .add_system(BrahmaEditor::on_node_selected.system())
        .add_system(BrahmaEditor::on_node_delete.system())

        // systems
        .add_system(BrahmaEditor::ui_node_selection_interaction.system())
        .add_system(BrahmaEditor::kb_key_press_input_system.system());

        println!("BrahmaEditorPlugin: Initialised");
    }
}