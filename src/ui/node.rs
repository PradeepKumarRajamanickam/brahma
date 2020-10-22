use bevy::prelude::*;

use super::{components::*, BrahmaEditor};

impl BrahmaEditor {
    
    pub fn generate_nodes(&mut self)
    {
        // stub
    }

    pub fn create_node(
        &mut self,
        mut commands: &mut Commands,
        materials: &mut Assets<ColorMaterial>,
        asset_server: &Res<AssetServer>,
        title: &str,
        pos: Vec2,
    ) -> u64 {

        let id = self.get_new_element_id();

        let  container = commands
            // outline
            .spawn(BrahmaNodeComponents {
                element_id: BrahmaElementId(id),

                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(175.0)),
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::ColumnReverse,

                    position: Rect {
                        top: Val::Px(0.0),
                        right: Val::Px(0.0),
                        left: Val::Px(pos.x()),
                        bottom: Val::Px(pos.y()),
                    },
                    ..Default::default()
                },
                material: materials.add(Color::BLACK.into()),
                ..Default::default()
            });
        
        let entity = container.current_entity().unwrap();
        self.set_entity_for_id(id, entity);

        container
            .with(BrahmaNodeRoot)
            .with_children(|parent| {
                BrahmaEditor::add_title_bar(parent, materials, &asset_server, id, title);
                BrahmaEditor::add_body(parent, materials, id);
            });
        
        id // node id
    }

    pub(crate) fn add_title_bar(
        parent: &mut ChildBuilder,
        materials: &mut Assets<ColorMaterial>,
        asset_server: &Res<AssetServer>,
        owner_id: u64,
        title: &str,
    ) {
        // handle
        parent
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(35.0)),
                    margin: Rect {
                        top: Val::Px(2.),
                        right: Val::Px(2.),
                        left: Val::Px(2.),
                        bottom: Val::Px(0.),
                    },
                    ..Default::default()
                },
                material: materials.add(BrahmaEditor::TITLE_BAR_NORMAL_COLOR.into()),
                ..Default::default()
            })
            // .with(BrahmaTitleBarTagComponent)

            // title
            .with_children(|parent| {
                parent
                    .spawn(ButtonComponents {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            justify_content: JustifyContent::Center,

                            margin: Rect {
                                top: Val::Px(1.),
                                bottom: Val::Px(1.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        material: materials.add(Color::NONE.into()),
                        draw: Draw {
                            is_transparent: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(BrahmaOwnerElementId(owner_id))
                    .with(BrahmaSelectableUiComponent)
                    .with_children(|parent| {
                        parent.spawn(TextComponents {
                            text: Text {
                                value: title.to_string(),
                                font: asset_server.load(BrahmaEditor::FONT.to_string()).unwrap(),
                                style: TextStyle {
                                    font_size: 25.0,
                                    color: BrahmaEditor::TITLE_TEXT_NORMAL_COLOR.into(),
                                },
                            },
                            ..Default::default()
                        });
                    });
            });
    }

    pub(crate) fn add_body(
        parent: &mut ChildBuilder, 
        materials: &mut Assets<ColorMaterial>, 
        owner_id: u64,
        ) {
        parent.spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: Rect {
                    top: Val::Px(1.),
                    right: Val::Px(2.),
                    left: Val::Px(2.),
                    bottom: Val::Px(2.),
                },
                ..Default::default()
            },
            material: materials.add(BrahmaEditor::NODE_BODY_NORMAL_COLOR.into()),
            ..Default::default()
        })
        .with(BrahmaOwnerElementId(owner_id))
        .with(BrahmaSelectableUiComponent);
        // .with(BrahmaBodyTagComponent);
    }
}
