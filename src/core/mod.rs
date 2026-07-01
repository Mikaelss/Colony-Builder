pub mod schedule;
pub mod time;

use bevy::prelude::*;
use schedule::SimulationPhase;
use time::TickResource;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickResource>();
        app.configure_sets(
            FixedUpdate,
            (
                SimulationPhase::PreTick,
                SimulationPhase::Tick,
                SimulationPhase::PostTick,
            )
                .chain(),
        );
    }
}
