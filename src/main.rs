mod core;
mod presentation;
mod simulation;

use bevy::prelude::*;
use core::CorePlugin;
use presentation::PresentationPlugin;
use simulation::SimulationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CorePlugin, SimulationPlugin, PresentationPlugin))
        .run();
}
