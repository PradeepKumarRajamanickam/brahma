use bevy::{math::*, prelude::*};

use super::components::*;
use super::events::*;
use super::BrahmaEditor;

impl BrahmaEditor {
    pub fn setup(
        mut commands: Commands,
        mut event_new_viewport: ResMut<Events<OnCreateNewViewportEvent>>,
        mut event_new_node: ResMut<Events<OnCreateNewNodeEvent>>,
        // mut editor: ResMut<BrahmaEditor>
    ) {
        println!("Setup...");
        commands.spawn(UiCameraComponents::default());
        event_new_viewport.send(OnCreateNewViewportEvent);

        // test
        // editor.create_node(&mut commands, materials, asset_server, title, pos)
        event_new_node.send(OnCreateNewNodeEvent {
            node_name: "node_1".to_string(),
            position: vec2(25.0, 100.0),
        });
        event_new_node.send(OnCreateNewNodeEvent {
            node_name: "node_2".to_string(),
            position: vec2(250.0, 250.),
        });
    }

    // systems
    // input
    pub(crate) fn kb_key_press_input_system(
        mut editor: ResMut<BrahmaEditor>,
        mut send_delete_node_event: ResMut<Events<OnNodeDeleteEvent>>,

        key_input: Res<Input<KeyCode>>) {

        if key_input.just_pressed(KeyCode::Back) 
            || key_input.just_pressed(KeyCode::Delete) {

            if(editor.currently_selected_node != 0)
            {
                send_delete_node_event.send(OnNodeDeleteEvent(editor.currently_selected_node));
            }
        }
    }

    // ui
    pub(crate) fn ui_node_selection_interaction(
        mut editor: ResMut<BrahmaEditor>,
        mut send_new_node_event: ResMut<Events<OnNodeSelectedEvent>>,
        mut interaction_query: Query<(
            &BrahmaSelectableUiComponent,
            &BrahmaOwnerElementId,
            Mutated<Interaction>,
        )>,
    ) {
        for (sel, owner_id, interaction) in &mut interaction_query.iter() {
            match *interaction {
                Interaction::Clicked => {
                    editor.currently_selected_node = owner_id.0;
                    send_new_node_event.send(OnNodeSelectedEvent(owner_id.0));
                }
                _ => {}
            }
        }
    }

    // pub(crate) fn ui_title_bar_selected_state
    // (
    //     title_bar: Res<BrahmaTitleBarTagComponent>,
    //     selectable:
    // )
    // {

    // }

}
