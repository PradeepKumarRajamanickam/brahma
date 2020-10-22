use std::collections::HashMap;
use bevy::{prelude::*, math::vec2};

use super::components::*;

#[derive(Default)]
pub struct BrahmaEditor {
    
    // crates
    pub(crate) currently_selected_node: u64,
    pub(crate) currently_held_node: u64, // if 0 none being dragged


    // privates
    ui_element_id_counter: u64,
    id_to_entity: HashMap<u64, Entity>,
    id_to_graph_node: HashMap<u64, u64>, // TODO* not fully functional
}

impl BrahmaEditor {

    // crates
    pub(crate) fn get_graph_id_from_id(&self, id: u64) -> Option<&u64>
    {
        self.id_to_graph_node.get(&id)
    }

    pub(crate) fn set_graph_id_from_id(&mut self, id: u64, graph_node: u64)
    {
        self.id_to_graph_node.entry(id).or_insert(graph_node);
    }

    pub(crate) fn get_entity_from_id(&self, id: u64) -> Option<&Entity>
    {
        self.id_to_entity.get(&id)
    }

    pub(crate) fn set_entity_for_id(&mut self, id: u64, entity: Entity)
    {
        self.id_to_entity.entry(id).or_insert(entity);
    }

    pub(crate) fn get_new_element_id(&mut self) -> u64
    {
        self.ui_element_id_counter = self.ui_element_id_counter + 1;
        self.ui_element_id_counter
    }

    pub(crate) fn reset(&mut self)
    {
        self.id_to_entity.clear();
        self.ui_element_id_counter = 0;

        self.currently_held_node = 0;
        self.currently_selected_node = 0;
    }
}