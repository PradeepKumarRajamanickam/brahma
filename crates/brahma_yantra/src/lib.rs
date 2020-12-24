pub use self::{components::*, stages::*, yantra::*};

use bevy::{
    app::prelude::*,
    ecs::{Commands, Res},
};

mod components;
mod stages;
mod yantra;

#[derive(Default)]
pub struct YantraPlugin;

impl Plugin for YantraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup)
            .add_resource(Yantra::default());
    }
}

fn setup(mut commands: Commands, mut yantra: Res<Yantra>) {
    let comp = YantraEntityHolder::default();
    let holder_entity = commands.spawn((comp)).current_entity().unwrap();
    println!("YantraPlugin: Initialised");
}
