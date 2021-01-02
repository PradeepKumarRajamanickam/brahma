#![allow(non_snake_case)]
pub mod Event {
    use bevy::ecs::Entity;
    use brahma_yantra::*;

    // local events
    pub struct OnSubmit {
        pub(crate) owner: Entity,
        pub input: String,
    }
    impl Trait::Owner for OnSubmit {
        fn get_owner(&self) -> Entity {
            self.owner
        }
    }
}
