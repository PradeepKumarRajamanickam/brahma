use bevy::{prelude::*, render::pipeline::*, sprite::QUAD_HANDLE, ui::*};

#[derive(Bundle, Clone)]
pub struct BrahmaNodeComponents {
    
    pub node: Node,
    pub element_id: BrahmaElementId,

    // copied from node components
    pub style: Style,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for BrahmaNodeComponents {
    fn default() -> Self {
        BrahmaNodeComponents {
            element_id: BrahmaElementId(0),

            // copied from node components
            mesh: QUAD_HANDLE,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                UI_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 1,
                            binding: 0,
                        },
                        // Node_size
                        DynamicBinding {
                            bind_group: 1,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),

            node: Default::default(),
            style: Default::default(),
            material: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

/// Current Node id
/// Note* if zero id not assigned
#[derive(Default, Clone)]
pub struct BrahmaElementId(pub u64);

/// Owner id
/// Note* if zero id not assigned
#[derive(Default, Clone)]
pub struct BrahmaOwnerElementId(pub u64);

/// Interacting with component allows you to drag the node
#[derive(Default, Clone)]
pub struct BrahmaDragHandleUiComponent;

/// Interacting with component causes the node to be selected 
#[derive(Default, Clone)]
pub struct BrahmaSelectableUiComponent;

// TAGS
/// Viewport root under which nodes are spawned
#[derive(Default, Clone)]
pub struct BrahmaViewport;

/// Root of the node ui i.e. if this is deleted the node is removed
#[derive(Default, Clone)]
pub struct BrahmaNodeRoot;

// tag to identify title bar
#[derive(Default, Clone)]
pub struct BrahmaTitleBarTagComponent;

// tag to identify title text
#[derive(Default, Clone)]
pub struct BrahmaTitleTextTagComponent;

// tag to identify node body
#[derive(Default, Clone)]
pub struct BrahmaBodyTagComponent;