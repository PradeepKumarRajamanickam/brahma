use std::collections::HashSet;

use crate::Graph;

impl Graph {
    // control ports
    // fsm node control port
    pub fn connect_fsm_control_ports(
        &mut self,
        src: u64,
        dest: u64,
    ) -> Result<(), &str> {
        if src == dest {
            return Err("Cannot connect to self");
        }

        if let Some(_set) = self.connection_fsm_control_ports.get_mut(&src) {
            _set.insert(dest);
        } else {
            let mut new_set = HashSet::new();
            new_set.insert(dest);
            self.connection_fsm_control_ports.insert(src, new_set);
        }

        return Ok(());
    }
    pub fn disconnect_fsm_control_ports(&mut self, src: u64, dest: u64) {
        if let Some(set) = self.connection_fsm_control_ports.get_mut(&src) {
            set.remove(&dest);
        }
    }
    pub fn disconnect_fsm_control_port_for_node(&mut self, node: u64) {
        self.connection_fsm_control_ports.remove(&node);
    }

    // lane node control port
    pub fn connect_lane_control_ports(
        &mut self,
        src: u64,
        dest: u64,
    ) -> Result<(), &str> {
        if src == dest {
            return Err("Cannot connect to self");
        }

        if let Some(_set) = self.connection_lane_control_ports.get_mut(&src) {
            _set.insert(dest);
        } else {
            let mut new_set = HashSet::new();
            new_set.insert(dest);
            self.connection_lane_control_ports.insert(src, new_set);
        }

        return Ok(());
    }
    pub fn disconnect_lane_control_port(&mut self, src: u64, dest: u64) {
        if let Some(set) = self.connection_lane_control_ports.get_mut(&src) {
            set.remove(&dest);
        }
    }
    pub fn disconnect_lane_control_port_for_node(&mut self, node: u64) {
        self.connection_lane_control_ports.remove(&node);
    }

    // data port
    pub fn connect_data_port(
        &mut self,
        src: u64,
        dest: u64,
    ) -> Result<(), &str> {
        if src == dest {
            return Err("Cannot connect to self");
        }

        self.connection_data_ports.insert(src, dest);
        self.connection_data_ports.insert(dest, src);
        return Ok(());
    }
    pub fn disconnect_data_port(&mut self, src: u64) {
        let ports = self.connection_data_ports.clone();
        if let Some(dest) = ports.get(&src) {
            self.connection_data_ports.remove(&src);
            self.connection_data_ports.remove(&dest);
        }
    }
}
