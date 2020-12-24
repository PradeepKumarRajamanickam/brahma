use brahma_graph::Graph;
use std::collections::{HashMap, HashSet};

pub struct Scribe {
    pub code_fsm: HashMap<u64, System>,
    pub code_lanes: HashMap<u64, System>,
}
pub struct System {
    /*
    fn state_start_on_enter()
    fn state_start_on_update()
    */
    pub method_name: String,
    /*
    {

    }
    */
    pub code_block: String,
    pub code_children: HashSet<u64>,
}
