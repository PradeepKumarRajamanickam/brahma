use std::collections::HashSet;

use crate::*;
use serde::{Deserialize, Serialize};

impl Graph {
    pub fn add_variable_node(&mut self, variable: Variable) -> u64
    {
        let id = self.get_new_id();
        self.node_variables.insert(id, variable);

        self.type_node.insert(id, NodeType::Variable);

        return id;
    }
    pub fn remove_variable_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_variables.clone().get(&id)
        {
            self.disconnect_data_port(n.getter);
            self.disconnect_data_port(n.setter);
            self.disconnect_lane_control_port_for_node(id);
        }

        self.node_variables.remove(&id);
    }

    pub fn add_collection_node(&mut self, collection: Collection) -> u64
    {
        let id = self.get_new_id();
        self.node_collection.insert(id, collection);

        self.type_node.insert(id, NodeType::Collection);

        return id;
    }
    pub fn remove_collection_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_collection.clone().get(&id)
        {
            // remove param connections
            for p in n.setter.clone()
            {
                self.disconnect_data_port(p);
            }

            // remove return connections
            self.disconnect_data_port(n.getter);
            self.disconnect_lane_control_port_for_node(id)
        }
        self.node_collection.remove(&id);
    }
}

// Leaf node represents lane local variables
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Variable {
    // node display info
    pub description: String,

    // dep info
    pub imports: Option<String>,

    // data ports
    pub setter: u64,
    pub getter: u64,

    // var info
    /// Variable name
    pub var_name: Option<String>,
    // Type name e.g. i32, custom_type...etc
    pub type_name: String,
    // RON Data
    pub serialised_data: String,
}

/// Leaf node represents a collection.
/// Takes variables of same type
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Collection {
    // node display info
    pub display_name: String,
    pub description: String,

    // data ports
    pub setter: HashSet<u64>,
    pub getter: u64,

    // Type name e.g. i32, custom_type...etc
    pub type_name: String,
}