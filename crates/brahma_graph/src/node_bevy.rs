use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use crate::*;

impl Graph {

    pub fn add_bevy_event_node(&mut self, bevy_event: BevyEvent) -> u64
    {
        let id = self.get_new_id();
        self.node_bevy_events.insert(id, bevy_event);

        self.type_node.insert(id, NodeType::BevyEvent);

        return id;
    }
    pub fn remove_bevy_event_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_bevy_events.clone().get(&id)
        {
            // remove argument connections
            for d in n.arguments.clone()
            {
                self.disconnect_data_port(d.clone());
            }
            
            self.disconnect_lane_control_port_for_node(id)
        }

        self.node_bevy_events.remove(&id);
    }

    pub fn add_bevy_resource_node(&mut self, bevy_resource: BevyResource) -> u64
    {
        let id = self.get_new_id();
        self.node_bevy_resources.insert(id, bevy_resource);

        self.type_node.insert(id, NodeType::BevyResource);

        return id;
    }
    pub fn remove_bevy_resource_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);
        
        if let Some(n) = self.node_bevy_resources.clone().get(&id)
        {
            self.disconnect_data_port(n.getter.clone());
            self.disconnect_data_port(n.setter.clone());
            self.disconnect_lane_control_port_for_node(id);
        }
        self.node_bevy_resources.remove(&id);
    }

    pub fn add_bevy_method_node(&mut self, bevy_method: BevyMethod) -> u64
    {
        let id = self.get_new_id();
        self.node_bevy_methods.insert(id, bevy_method);

        self.type_node.insert(id, NodeType::BevyMethod);

        return id;
    }
    pub fn remove_bevy_method_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_bevy_methods.clone().get(&id)
        {
            // remove param connections
            for d in n.params.clone()
            {
                self.disconnect_data_port(d.clone());
            }

            if let Some(r) = n.returns
            {
                self.disconnect_data_port(r.clone());
            }

            self.disconnect_lane_control_port_for_node(id)
        }

        self.node_bevy_methods.remove(&id);
    }
}

/// Leaf node represents a bevy event
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct BevyEvent {
    // dep info
    pub imports: Option<String>,

    // event info
    pub event_name: String,

    // data ports
    pub arguments: HashSet<u64>,
}

// Leaf node represents variables
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct BevyResource {
    // dep info
    pub imports: Option<String>,

    // data ports
    pub setter: u64,
    pub getter: u64,

    // var info
    /// Variable name
    pub var_name: Option<String>,
}

// Leaf node represents a resource method call
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct BevyMethod {
    // data ports
    pub params: HashSet<u64>, // input parameters
    pub returns: Option<u64>,  // return value

    // dep info
    pub imports: Option<String>,
    
    // method info
    pub function_name: String, // function call
}