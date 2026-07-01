mod core;
mod input;
mod presentation;
mod simulation;
mod world;

use bevy::prelude::*;
use core::CorePlugin;
use input::InputPlugin;
use presentation::PresentationPlugin;
use simulation::SimulationPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            CorePlugin,
            SimulationPlugin,
            PresentationPlugin,
            WorldPlugin,
            InputPlugin,
        ))
        .run();
}
