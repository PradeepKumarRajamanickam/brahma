use std::collections::HashSet;

use brahma_graph::*;

fn main() {
    let mut graph: Graph = Graph {
        ..Default::default()
    };

    let state_id_start = get_start_state(&mut graph);
    let state_id_idle = get_idle_state(&mut graph);

    let transition_id_start_to_idle =
        get_start_to_idle_transition(&mut graph, state_id_start, state_id_idle);

    // fsm
    graph
        .connect_fsm_control_ports(state_id_start, transition_id_start_to_idle)
        .unwrap();

    graph
        .connect_fsm_control_ports(transition_id_start_to_idle, state_id_idle)
        .unwrap();

    graph.fsm_start_node = state_id_start;

    let pretty = ron::ser::PrettyConfig::new()
        .with_enumerate_arrays(true)
        .with_new_line("\n".to_string());
    let serialized = ron::ser::to_string_pretty(&graph, pretty);

    println!("Data: \n\n {} ", serialized.unwrap());
}

fn get_start_state(graph: &mut Graph) -> u64 {
    // Start state lane
    let id_on_enter_event = graph.add_event_node(Event {
        event_type: BrahmaEventType::OnEnter,

        ..Default::default()
    });

    let id_variable_string = graph.add_variable_node(Variable {
        var_name: Some("local_var_msg".to_string()),
        type_name: "String".to_string(),
        serialised_data: "\"hello I am in start\"".to_string(),

        ..Default::default()
    });
    let id_print_method = graph.add_method_node(Method {
        function_name: "println!".to_string(),
        ..Default::default()
    });

    graph
        .connect_lane_control_ports(id_on_enter_event, id_print_method)
        .unwrap();

    return graph.add_state_node(State {
        display_name: "start".to_string(),

        events: [id_on_enter_event].iter().cloned().collect(),
        methods: [id_print_method].iter().cloned().collect(),
        variables: [id_variable_string].iter().cloned().collect(),
        ..Default::default()
    });
}

fn get_idle_state(graph: &mut Graph) -> u64 {
    // Idle state lane
    let id_on_enter_event = graph.add_event_node(Event {
        event_type: BrahmaEventType::OnEnter,

        ..Default::default()
    });

    let id_variable_string = graph.add_variable_node(Variable {
        var_name: Some("local_var_msg".to_string()),
        type_name: "String".to_string(),
        serialised_data: "\"Now I am in idle\"".to_string(),

        ..Default::default()
    });
    let id_print_method = graph.add_method_node(Method {
        function_name: "println!".to_string(),
        ..Default::default()
    });

    graph
        .connect_lane_control_ports(id_on_enter_event, id_print_method)
        .unwrap();

    return graph.add_state_node(State {
        display_name: "idle".to_string(),

        events: [id_on_enter_event].iter().cloned().collect(),
        methods: [id_print_method].iter().cloned().collect(),
        variables: [id_variable_string].iter().cloned().collect(),

        ..Default::default()
    });
}

fn get_start_to_idle_transition(
    graph: &mut Graph,
    start: u64,
    end: u64,
) -> u64 {
    // node
    let transition = graph.add_transition_node(Transition {
        display_name: "transition_on_enter".to_string(),
        ..Default::default()
    });

    // transistion lane
    let id_on_enter_event = graph.add_event_node(Event {
        event_type: BrahmaEventType::OnEnter,
        ..Default::default()
    });

    let id_transit = graph.add_transit_node(Transit { owner: transition });

    let mut t = graph.node_transitions.get_mut(&transition).unwrap();
    t.events = [id_on_enter_event].iter().cloned().collect();
    t.transits = [id_transit].iter().cloned().collect();

    return transition;
}
