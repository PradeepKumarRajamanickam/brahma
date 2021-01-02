#![allow(non_snake_case)]
pub mod Event {
    use bevy::ecs::Entity;

    pub struct OnSubmit {
        // pub target: Entity,
        pub input: String,
    }
}
