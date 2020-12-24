use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum NodeType {
    None,

    // fsm nodes
    Event,
    State,
    Transit,
    Transition,

    // bevy nodes
    BevyEvent,
    BevyMethod,
    BevyResource,

    // method nodes
    Algo,
    Method,
    Return,
    Execute,

    // data node
    Variable,
    Collection,

    // flow nodes
    Branch,
    MultiBranch,
    Iterator,
    IteratorSkip,
    IteratorBreak,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::None
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum BrahmaEventType {
    None,

    OnEnter,
    OnUpdate,
    OnExit,
}

impl Default for BrahmaEventType {
    fn default() -> Self {
        BrahmaEventType::None
    }
}
