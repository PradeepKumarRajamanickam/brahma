use bevy::{winit::WinitWindows, input::mouse::*, math::*, prelude::*};

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

    pub(crate) fn mouse_move_event_system(
        resources: &mut Resources,
        mut editor: ResMut<BrahmaEditor>,
        mut send_drag_node_event: ResMut<Events<OnNodeDragEvent>>,

        events: Res<Events<MouseMotion>>,
        mut reader: Local<EventReader<MouseMotion>>
    ) {
        let windows = resources.get_mut::<WinitWindows>().unwrap();

        if let Some(value) = reader.latest(&events) {
            
            for (_, window) in windows.windows.iter() {
                window
            }
            // drag node
            let node = editor.currently_held_node;
            if node != 0
            {
                send_drag_node_event.send(OnNodeDragEvent(node, value.delta));
            }
        }
    }

//     fn remove_cursor(
//     _world: &mut World,
//     resources: &mut Resources,
// ) {
    

//     for (_, window) in windows.windows.iter() {
//         println!("Window");
//         window.set_cursor_visible(false);
//         window.set_cursor_position(Position::Physical(PhysicalPosition::new((window.inner_size().width / 2) as i32, (window.inner_size().height / 2) as i32)));

//     }
// }

    // ui
    pub(crate) fn ui_node_selection_interaction(
        
        mut editor: ResMut<BrahmaEditor>,
        mut send_node_selected_event: ResMut<Events<OnNodeSelectedEvent>>,

        mut interaction_query: Query<(
            &BrahmaSelectableUiComponent,
            &BrahmaOwnerElementId,
            Mutated<Interaction>,
        )>,
        mut title_bar_held_query: Query<(
            &BrahmaDragHandleUiComponent,
            &BrahmaOwnerElementId,
            Mutated<Interaction>,
        )>,
    ) {
        // select node
        for (sel, owner_id, interaction) in &mut interaction_query.iter() {
            match *interaction {
                Interaction::Clicked => {
                    editor.currently_selected_node = owner_id.0;
                    send_node_selected_event.send(OnNodeSelectedEvent(owner_id.0));
                }
                _ => {}
            }
        }

        // title bar held
        for (sel, owner_id, interaction) in &mut title_bar_held_query.iter() {
            match *interaction {
                Interaction::Clicked => {
                    editor.currently_held_node = owner_id.0;
                    println!("Titlebar held: {}", editor.currently_held_node);
                }
                Interaction::None => {}
                
                Interaction::Hovered => { editor.currently_held_node = 0; println!("Titlebar released");}
            }
        }
    }
}
