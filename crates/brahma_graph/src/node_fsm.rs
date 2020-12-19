use std::collections::HashSet;

use crate::*;
use serde::{Deserialize, Serialize};

// FSM Nodes
impl Graph {

    // fsm
    pub fn add_state_node(&mut self, state: State) -> u64
    {
        let id = self.get_new_id();
        self.node_states.insert(id, state);
        
        self.type_node.insert(id, NodeType::State);

        return id;
    }
    pub fn remove_state_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_states.clone().get(&id)
        {
            // remove children
            for i in n.algo.clone()
            {
                self.remove_algo_node(i)
            }
            for i in n.branch.clone()
            {
                self.remove_branch_node(i)
            }
            for i in n.methods.clone()
            {
                self.remove_method_node(i)
            }
            for i in n.variables.clone()
            {
                self.remove_variable_node(i)
            }
            for i in n.iterators.clone()
            {
                self.remove_iterator_node(i)
            }

            
            self.disconnect_fsm_control_port_for_node(id)
        }


        self.node_states.remove(&id);
    }
    
    pub fn add_transition_node(&mut self, transition: Transition) -> u64
    {
        let id = self.get_new_id();
        self.node_transitions.insert(id, transition);

        self.type_node.insert(id, NodeType::Transition);

        return id;
    }
    pub fn remove_transition_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        self.node_transitions.remove(&id);
        self.connection_fsm_control_ports.remove(&id);
    }

    pub fn add_event_node(&mut self, event: Event) -> u64
    {
        let id = self.get_new_id();
        self.node_events.insert(id, event);

        self.type_node.insert(id, NodeType::Event);

        return id;
    }
    pub fn remove_event_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_events.get(&id)
        {
            // remove argument connections
            for d in n.arguments.clone()
            {
                self.disconnect_data_port(d);
            }
            
            self.disconnect_lane_control_port_for_node(id)
        }

        self.node_events.remove(&id);
    }

    pub fn add_transit_node(&mut self, transit: Transit) -> u64
    {
        let id = self.get_new_id();
        self.node_transit.insert(id, transit);

        self.type_node.insert(id, NodeType::Transit);

        return id;
    }
    pub fn remove_transit_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        self.node_transit.remove(&id);
        self.disconnect_lane_control_port_for_node(id);
    }
}

/// A State in the machine. Note* on_enter should be added as entry point
/// for state. Can only connect to a Transition.
/// Supported Children: event, algo, method, data, iterator, branch
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct State {
    // node display info
    pub display_name: String,
    pub description: String, 

    // children
    pub algo: HashSet<u64>,
    pub branch: HashSet<u64>,
    pub events: HashSet<u64>,
    pub methods: HashSet<u64>,
    pub variables: HashSet<u64>,
    pub iterators: HashSet<u64>,
}
/// When control flows to exit fsm transitions to 
/// the connected State. Can only connect to a State.
/// Supported Children: event, algo, method, data, branch
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Transition {
    // node display info
    pub display_name: String,
    pub description: String, 

    // children
    pub algo: HashSet<u64>,
    pub branch: HashSet<u64>,
    pub events: HashSet<u64>, // on_enter, tick, on_exit, custom message...etc
    pub methods: HashSet<u64>,
    pub variables: HashSet<u64>,

    pub transits: HashSet<u64>
}

/// Leaf node represents brahma state/transition events
/// on_enter, on_update and on_exit. These are the entry
/// nodes for state & transition lanes.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Event {
    // event info
    pub event_type: BrahmaEventType,

    // data ports
    pub arguments: HashSet<u64>,
}


/// Leaf node represents brahma transition lane exit
/// node
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Transit
{
    pub owner: u64 // owner transition it belongs to
}