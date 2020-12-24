pub use self::{
    stages::*,
    yantra::*,
    components::*,
};


use bevy::{app::prelude::*, ecs::{Commands, Res}};

mod stages;
mod yantra;
mod components;

#[derive(Default)]
pub struct YantraPlugin;

impl Plugin for YantraPlugin {
    fn build(&self, app: &mut AppBuilder) {

        app
        .add_startup_system(setup)
        .add_resource(Yantra::default());
    }
}

fn setup(
    mut commands: Commands,
    mut yantra: Res<Yantra>
)
{
    let comp = YantraEntityHolder::default();
    let holder_entity = commands
    .spawn((comp))
    .current_entity()
    .unwrap();
    println!("YantraPlugin: Initialised");

}