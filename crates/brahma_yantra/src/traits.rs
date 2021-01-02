pub mod Trait {
    use bevy::ecs::Entity;
    // traits
    pub trait Owner {
        fn get_owner(&self) -> Entity;
    }
}
