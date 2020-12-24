use bevy::{prelude::*, math::*};
use super::{editor::BrahmaEditor, components::*, materials::*};

pub struct OnCreateNewViewportEvent;

pub struct OnCreateNewNodeEvent
{
    pub node_name: String, 
    pub position: Vec2
}
pub struct OnNodeSelectedEvent(pub u64);
pub struct OnNodeDragEvent(pub u64, pub Vec2);
pub struct OnNodeDeleteEvent(pub u64);

pub struct OnNoodleSelectedEvent(pub u64);
pub struct OnNoodleDragEvent(pub u64, pub Vec2);
pub struct OnNoodleDropEvent(pub u64, pub u64); // param 0 source, param 1 target if 0 none


impl BrahmaEditor
{
    // user events
    pub(crate) fn on_create_new_viewport(
        mut commands: Commands,

        events: Res<Events<OnCreateNewViewportEvent>>,
        mut reader: Local<EventReader<OnCreateNewViewportEvent>>,
    ) {
        for _ev in reader.latest(&events) {
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
        materials: Res<BrahmaMaterials>,
        events: Res<Events<OnNodeSelectedEvent>>,
        mut reader: Local<EventReader<OnNodeSelectedEvent>>,

        mut title_bar_query: Query<(
            &BrahmaOwnerElementId,
            &BrahmaTitleBarTagComponent,
            &mut Handle<ColorMaterial>
        )>,

        mut title_text_query: Query<(
            &BrahmaOwnerElementId,
            &BrahmaTitleTextTagComponent,
            &mut Handle<ColorMaterial>
        )>,
    )
    {
        for ev in reader.latest(&events)
        {
            let _owner_id: u64 = ev.0;
            println!("BrahmaEditor: Selected Node {}", _owner_id);

            // title bar
            for (owner_id, _tag, mut mat) in &mut title_bar_query.iter()
            {
                if owner_id.0 == _owner_id
                {
                    *mat = materials.title_bar_selected.clone();
                }else
                {
                    *mat = materials.title_bar_normal.clone();
                }
            }
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
            let _owner_id: u64 = ev.0;
            let entity = editor.get_entity_from_id(_owner_id).unwrap();
            commands.despawn_recursive(*entity);

            editor.currently_selected_node = 0;

            println!("BrahmaEditor: Deleted Node {}", ev.0);
        }
    }

    pub(crate) fn on_node_drag(
        mut commands: Commands,
        mut trans: ResMut<Translation>,
        mut editor: ResMut<BrahmaEditor>,
        events: Res<Events<OnNodeDragEvent>>,
        mut reader: Local<EventReader<OnNodeDragEvent>>
    )
    {
        for ev in reader.latest(&events)
        {
            let id = ev.0;
            let delta = ev.1;
            let entity = editor.get_entity_from_id(id).unwrap();
            println!("BrahmaEditor: Drag Node {} {}", id, delta);
        }
    }
}