pub mod definitions;
pub mod event;
pub mod identity;
pub mod schedule;
pub mod state;
pub mod time;

use bevy::prelude::*;
use definitions::DefinitionPlugin;
use event::CoreEventPlugin;
use schedule::SimulationPhase;
use state::GameState;
use time::TickResource;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(
            TickResource::TICKS_PER_DAY as f64 / TickResource::DAY_DURATION_SECS,
        ));
        app.init_resource::<TickResource>();
        app.init_state::<GameState>();
        app.add_plugins((CoreEventPlugin, DefinitionPlugin));
        app.configure_sets(
            FixedUpdate,
            (
                SimulationPhase::PreTick,
                SimulationPhase::Tick,
                SimulationPhase::PostTick,
            )
                .chain(),
        );
        app.add_systems(Update, setup_loading.run_if(in_state(GameState::Loading)));
    }
}

fn setup_loading(mut next_state: ResMut<NextState<GameState>>) {
    println!("[GameState] Loading complete. Starting game...");
    next_state.set(GameState::Playing);
}
