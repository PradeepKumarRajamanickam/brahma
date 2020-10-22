use bevy::prelude::*;

use super::editor::BrahmaEditor;

pub struct BrahmaMaterials {
    pub title_bar_normal: Handle<ColorMaterial>,
    pub title_bar_selected: Handle<ColorMaterial>,

    pub title_text_normal: Handle<ColorMaterial>,
    pub title_text_selected: Handle<ColorMaterial>,
}

impl FromResources for BrahmaMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        BrahmaMaterials {
            title_bar_normal: materials.add( BrahmaEditor::TITLE_BAR_NORMAL_COLOR.into()),
            title_bar_selected: materials.add(BrahmaEditor::TITLE_BAR_SELECTED_COLOR.into()),

            title_text_normal: materials.add( BrahmaEditor::TITLE_TEXT_NORMAL_COLOR.into()),
            title_text_selected: materials.add(BrahmaEditor::TITLE_TEXT_SELECTED_COLOR.into()),
        }
    }
}