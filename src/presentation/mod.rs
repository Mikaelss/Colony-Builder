pub mod dummy;

use crate::core::state::GameState;
use bevy::prelude::*;
use dummy::print_tick;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_tick.run_if(in_state(GameState::Playing)));
    }
}
