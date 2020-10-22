use bevy::prelude::Color;

use super::BrahmaEditor;

impl BrahmaEditor {
    // assets
    pub const FONT: &'static str = "assets/FiraSans-Bold.ttf";

     // colors
    pub const TITLE_BAR_NORMAL_COLOR: Color = Color::rgb(0.05, 0.05, 0.05);
    pub const TITLE_BAR_SELECTED_COLOR: Color = Color::rgb(0.15, 0.75, 0.15);

    pub const TITLE_TEXT_NORMAL_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const TITLE_TEXT_SELECTED_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);

    pub const NODE_BODY_NORMAL_COLOR: Color = Color::rgb(0.05, 0.05, 0.05);
    pub const NODE_BODY_SELECTED_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
}