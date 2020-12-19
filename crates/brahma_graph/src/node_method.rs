use std::collections::{HashMap, HashSet};

use crate::*;
use serde::{Deserialize, Serialize};

impl Graph {
    // algorithms
    pub fn add_algo_node(&mut self, algo: Algo) -> u64
    {
        let id = self.get_new_id();
        self.node_algos.insert(id, algo);

        self.type_node.insert(id, NodeType::Algo);

        return id;
    }
    pub fn remove_algo_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_algos.clone().get(&id)
        {
            self.remove_execute_node(n.execute);
            self.remove_return_node(n.returns);

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
            
            self.disconnect_lane_control_port_for_node(id)
        }

        self.node_algos.remove(&id);
    }

    pub fn add_method_node(&mut self, method: Method) -> u64
    {
        let id = self.get_new_id();
        self.node_methods.insert(id, method);

        self.type_node.insert(id, NodeType::Method);

        return id;
    }
    pub fn remove_method_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_methods.clone().get(&id)
        {
            // remove param connections
            for d in n.params.clone()
            {
                self.disconnect_data_port(d);
            }

            if let Some(r) = n.returns
            {
                self.disconnect_data_port(r);
            }

            self.disconnect_lane_control_port_for_node(id)
        }

        self.node_methods.remove(&id);
    }
    
    pub fn add_execute_node(&mut self, execute: Execute) -> u64
    {
        let id = self.get_new_id();
        self.node_executes.insert(id, execute);

        self.type_node.insert(id, NodeType::Execute);

        return id;
    }
    pub fn remove_execute_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_executes.get(&id)
        {
            // remove alias connections
            for d in n.param_alias.clone()
            {
                self.disconnect_data_port(d)
            }
        }

        self.disconnect_lane_control_port_for_node(id);
        self.node_executes.remove(&id);
    }

    pub fn add_return_node(&mut self, mut _return: Return) -> u64
    {
        let id = self.get_new_id();
        self.node_returns.insert(id,_return);

        self.type_node.insert(id, NodeType::Execute);

        return id;
    }

    pub fn remove_return_node(&mut self, id: u64)
    {
        self.type_node.remove(&id);

        if let Some(n) = self.node_returns.get(&id)
        {
            // remove alias connections
            for d in n.value_alias.clone()
            {
                self.disconnect_data_port(d)
            }
        }

        self.disconnect_lane_control_port_for_node(id);
        self.node_returns.remove(&id);
    }
}

// Algorithms
/// Entry for algorithm nodes
#[derive(Serialize, Deserialize, Clone)]
pub struct Execute {
    // data port
    /// internal representation 
    /// of external data port
    pub param_alias: HashSet<u64>,
    /// maps params to original
    pub alias_lookup: HashMap<u64, u64>,
}
/// Exit for algorithm node
#[derive(Serialize, Deserialize, Clone)]
pub struct Return {
    // data ports
    pub value_alias: HashSet<u64>,
    /// internal representation 
    /// of the data port, maps
    /// value to original
    pub alias_lookup: HashMap<u64, u64>,
}

/// Algorithm node. 
/// Supported Children: nested algo, method, data, iterator, branch
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Algo {
    // node display info
    pub display_name: String,
    pub description: String, 

    // internal control nodes
    pub execute: u64,
    pub returns: u64,

    // children
    pub algo: HashSet<u64>,
    pub branch: HashSet<u64>,
    pub methods: HashSet<u64>,
    pub variables: HashSet<u64>,
    pub iterators: HashSet<u64>
}


// Leaf node represents a rust method call
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Method {
    // data ports
    pub params: HashSet<u64>, // input parameters
    pub returns: Option<u64>,  // return value

    // dep info
    pub imports: Option<String>,
    
    // method info
    pub function_name: String, // function call
    pub is_op_overload: bool,  //+,-,*...etc
}