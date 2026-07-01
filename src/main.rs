mod core;
mod presentation;
mod simulation;
mod world;

use bevy::prelude::*;
use core::CorePlugin;
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
        ))
        .run();
}
