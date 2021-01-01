pub mod Event {
    use bevy::ecs::Entity;
    // fsm events
    pub struct OnEnter {
        pub target: Entity,
    }
    pub struct OnUpdate {
        pub target: Entity,
    }
    pub struct OnExit {
        pub target: Entity,
    }
}
