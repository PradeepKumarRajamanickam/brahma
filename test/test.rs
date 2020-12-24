mod brahma_logics;

use bevy::{app::startup_stage, prelude::*};

fn main() {
    println!("Brahma Sample Test");
    App::build()
        .add_plugins(DefaultPlugins)
        // setup
        .add_plugin(brahma_logics::BrahmaBootStrap::default())
        .add_startup_system_to_stage(startup_stage::STARTUP, setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dComponents::default())
        // paddle
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(
                -200.0, -215.0, 0.0,
            )),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .with(brahma_logics::sample_machine::Machine)
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1., 0.5, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(
                -200.0, 215.0, 0.0,
            )),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .with(brahma_logics::sample_machine::Machine)
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 1.5, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(
                100.0, -215.0, 0.0,
            )),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .with(brahma_logics::sample_machine::Machine);
    println!("Test: Setup Done");
}
