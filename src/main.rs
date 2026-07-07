use Colony_Builder::core::CorePlugin;
use Colony_Builder::debug::DebugPlugin;
use Colony_Builder::input::InputPlugin;
use Colony_Builder::presentation::PresentationPlugin;
use Colony_Builder::simulation::SimulationPlugin;
use Colony_Builder::world::WorldPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            CorePlugin,
            SimulationPlugin,
            PresentationPlugin,
            WorldPlugin,
            InputPlugin,
            DebugPlugin,
        ))
        .run();
}
