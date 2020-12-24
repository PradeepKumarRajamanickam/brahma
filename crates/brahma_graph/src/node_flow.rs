use std::collections::HashSet;

use crate::*;
use serde::{Deserialize, Serialize};

impl Graph {
    pub fn add_branch_node(&mut self, branch: Branch) -> u64 {
        let id = self.get_new_id();
        self.node_branches.insert(id, branch);

        self.type_node.insert(id, NodeType::Branch);

        return id;
    }
    pub fn remove_branch_node(&mut self, id: u64) {
        self.type_node.remove(&id);

        if let Some(n) = self.node_branches.clone().get(&id) {
            self.remove_execute_node(n.execute_true);
            self.remove_execute_node(n.execute_false);
            self.remove_return_node(n.return_true);
            self.remove_return_node(n.return_false);

            // remove children
            for i in n.algo.clone() {
                self.remove_algo_node(i)
            }
            for i in n.branch.clone() {
                self.remove_branch_node(i)
            }
            for i in n.methods.clone() {
                self.remove_method_node(i)
            }
            for i in n.variables.clone() {
                self.remove_variable_node(i)
            }
            for i in n.iterators.clone() {
                self.remove_iterator_node(i)
            }

            self.disconnect_lane_control_port_for_node(id)
        }

        self.node_branches.remove(&id);
    }

    pub fn add_multi_branch_node(&mut self, multi_branch: MultiBranch) -> u64 {
        let id = self.get_new_id();
        self.node_multi_branches.insert(id, multi_branch);

        self.type_node.insert(id, NodeType::MultiBranch);

        return id;
    }
    pub fn remove_multi_branch_node(&mut self, id: u64) {
        self.type_node.remove(&id);

        if let Some(n) = self.node_multi_branches.clone().get(&id) {
            self.remove_execute_node(n.execute_default);
            for r in n.executes.clone() {
                self.remove_execute_node(r);
            }

            for r in n.returns.clone() {
                self.remove_return_node(r);
            }

            // remove children
            for i in n.algo.clone() {
                self.remove_algo_node(i)
            }
            for i in n.branch.clone() {
                self.remove_branch_node(i)
            }
            for i in n.methods.clone() {
                self.remove_method_node(i)
            }
            for i in n.variables.clone() {
                self.remove_variable_node(i)
            }
            for i in n.iterators.clone() {
                self.remove_iterator_node(i)
            }

            self.disconnect_lane_control_port_for_node(id)
        }

        self.node_branches.remove(&id);
    }

    pub fn add_iterator_node(&mut self, iterator: Iterator) -> u64 {
        let id = self.get_new_id();
        self.node_iterators.insert(id, iterator);

        self.type_node.insert(id, NodeType::Iterator);

        return id;
    }
    pub fn remove_iterator_node(&mut self, id: u64) {
        self.type_node.remove(&id);

        if let Some(n) = self.node_iterators.clone().get(&id) {
            self.remove_execute_node(n.execute);
            self.remove_return_node(n.returns);

            // remove children
            for i in n.algo.clone() {
                self.remove_algo_node(i)
            }
            for i in n.branch.clone() {
                self.remove_branch_node(i)
            }
            for i in n.methods.clone() {
                self.remove_method_node(i)
            }
            for i in n.variables.clone() {
                self.remove_variable_node(i)
            }
            for i in n.iterators.clone() {
                self.remove_iterator_node(i)
            }

            self.disconnect_lane_control_port_for_node(id)
        }

        self.node_iterators.remove(&id);
    }

    pub fn add_iterator_skip_node(&mut self, skip: IteratorSkip) -> u64 {
        let id = self.get_new_id();
        self.node_iterator_skips.insert(id, skip);

        self.type_node.insert(id, NodeType::IteratorSkip);

        return id;
    }
    pub fn remove_iterator_skip_node(&mut self, id: u64) {
        self.type_node.remove(&id);
        self.disconnect_lane_control_port_for_node(id);

        self.node_iterator_skips.remove(&id);
    }

    pub fn add_iterator_break_node(&mut self, _break: IteratorBreak) -> u64 {
        let id = self.get_new_id();
        self.node_iterator_breaks.insert(id, _break);

        self.type_node.insert(id, NodeType::IteratorBreak);

        return id;
    }
    pub fn remove_iterator_break_node(&mut self, id: u64) {
        self.type_node.remove(&id);
        self.disconnect_lane_control_port_for_node(id);

        self.node_iterator_breaks.remove(&id);
    }
}

/// Represents if
#[derive(Serialize, Deserialize, Clone)]
pub struct Branch {
    /// entry
    pub execute_true: u64,
    pub execute_false: u64,
    pub return_true: u64,
    pub return_false: u64,

    // children
    pub algo: HashSet<u64>,
    pub branch: HashSet<u64>,
    pub methods: HashSet<u64>,
    pub variables: HashSet<u64>,
    pub iterators: HashSet<u64>,
}

/// Represents switch
#[derive(Serialize, Deserialize, Clone)]
pub struct MultiBranch {
    /// entry
    pub execute_default: u64,
    pub executes: HashSet<u64>,
    pub returns: HashSet<u64>,

    // children
    pub algo: HashSet<u64>,
    pub branch: HashSet<u64>,
    pub methods: HashSet<u64>,
    pub variables: HashSet<u64>,
    pub iterators: HashSet<u64>,
}

/// Iterates through a collection,
/// Supported Children: nested algo, method,
/// iterator, skip, break, variables, branch
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Iterator {
    // entry
    pub execute: u64,
    pub returns: u64,

    // children
    pub algo: HashSet<u64>,
    pub branch: HashSet<u64>,
    pub methods: HashSet<u64>,
    pub variables: HashSet<u64>,
    pub iterators: HashSet<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IteratorSkip;
#[derive(Serialize, Deserialize, Clone)]
pub struct IteratorBreak;
