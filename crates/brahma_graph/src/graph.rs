use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::*;

#[derive(Default, Serialize, Deserialize)]
pub struct Graph {
    // dependencies
    pub dep_package: HashMap<String, String>,

    // types
    pub type_node: HashMap<u64, NodeType>,
    pub type_data_port: HashMap<u64, String>,

    // fsm nodes
    pub node_events: HashMap<u64, Event>,
    pub node_states: HashMap<u64, State>,
    pub node_transit: HashMap<u64, Transit>,
    pub node_transitions: HashMap<u64, Transition>,

    // bevy nodes
    pub node_bevy_events: HashMap<u64, BevyEvent>,
    pub node_bevy_methods: HashMap<u64, BevyMethod>,
    pub node_bevy_resources: HashMap<u64, BevyResource>,

    // method nodes
    pub node_algos: HashMap<u64, Algo>,
    pub node_methods: HashMap<u64, Method>,

    // data nodes
    pub node_variables: HashMap<u64, Variable>,
    pub node_collection: HashMap<u64, Collection>,

    // flow nodes
    pub node_returns: HashMap<u64, Return>,
    pub node_executes: HashMap<u64, Execute>,
    pub node_branches: HashMap<u64, Branch>,
    pub node_multi_branches: HashMap<u64, MultiBranch>,
    pub node_iterators: HashMap<u64, Iterator>,
    pub node_iterator_skips: HashMap<u64, IteratorSkip>,
    pub node_iterator_breaks: HashMap<u64, IteratorBreak>,

    // node connections
    // maps src to dest
    /// Data ports of algos, methods, events.
    /// Direction does not matter
    pub connection_data_ports: HashMap<u64, u64>,
    /// Flow of state graph. Direction matters.
    pub connection_fsm_control_ports: HashMap<u64, HashSet<u64>>,
    /// Flow of algorithm lane. Direction matters.
    pub connection_lane_control_ports: HashMap<u64, HashSet<u64>>,

    /// Id counter. Used for id generation.
    pub last_used_id: u64,
    /// entry point for the state machine
    pub fsm_start_node: u64,
}
