pub mod tick;

use crate::core::schedule::SimulationPhase;
use crate::core::state::GameState;
use bevy::prelude::*;
use tick::advance_tick;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            advance_tick
                .in_set(SimulationPhase::Tick)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
