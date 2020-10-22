use bevy::{prelude::*, math::*};
use super::{editor::BrahmaEditor, components::*};

pub struct OnCreateNewViewportEvent;

pub struct OnCreateNewNodeEvent
{
    pub node_name: String, 
    pub position: Vec2
}
pub struct OnNodeSelectedEvent(pub u64);
pub struct OnNodeDeleteEvent(pub u64);

impl BrahmaEditor
{
    // user events
    pub(crate) fn on_create_new_viewport(
        mut commands: Commands,

        events: Res<Events<OnCreateNewViewportEvent>>,
        mut reader: Local<EventReader<OnCreateNewViewportEvent>>,
    ) {
        for _ev in reader.iter(&events) {
            let mut viewport_root = commands
                .spawn(NodeComponents {
                    ..Default::default()
                })
                .with(BrahmaViewport);

            println!("BrahmaEditor: Create New Viewport");
        }
    }

    pub(crate) fn on_create_new_node(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut editor: ResMut<BrahmaEditor>,
        asset_server: Res<AssetServer>,

        events: Res<Events<OnCreateNewNodeEvent>>,
        mut reader: Local<EventReader<OnCreateNewNodeEvent>>,
    ) {
        for ev in reader.iter(&events) {
            
            let id = editor.create_node(
                &mut commands, 
                &mut *materials, 
                &asset_server, 
                &ev.node_name.to_string(),
                ev.position);

            let graph_node = 0; //TODO* Query from graph module
            editor.set_graph_id_from_id(id, graph_node);

            println!("BrahmaEditor: Create New Node {} Graph Node: {}", id, graph_node);
        }
    }

    pub(crate) fn on_node_selected(
        events: Res<Events<OnNodeSelectedEvent>>,
        mut reader: Local<EventReader<OnNodeSelectedEvent>>
    )
    {
        for ev in reader.iter(&events)
        {
            println!("BrahmaEditor: Selected Node {}", ev.0);
        }
    }

    pub(crate) fn on_node_delete(
        mut commands: Commands,
        mut editor: ResMut<BrahmaEditor>,
        events: Res<Events<OnNodeDeleteEvent>>,
        mut reader: Local<EventReader<OnNodeDeleteEvent>>
    )
    {
        for ev in reader.iter(&events)
        {
            let entity = editor.get_entity_from_id(editor.currently_selected_node).unwrap();
            commands.despawn_recursive(*entity);

            editor.currently_selected_node = 0;

            println!("BrahmaEditor: Deleted Node {}", ev.0);
        }
    }
}