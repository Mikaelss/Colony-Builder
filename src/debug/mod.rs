mod command;
mod print;
mod spawner;

use crate::core::identity::{EntityMap, IdAllocator};
use crate::core::state::GameState;
use bevy::prelude::*;
use command::DebugCommand;

#[derive(Default)]
pub struct DebugItem;

#[derive(Resource, Default)]
pub struct DebugSettings {
    pub dev_mode: bool,
    pub show_tick: bool,
    pub show_metrics: bool,
    pub show_spawns: bool,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugSettings>();
        app.init_resource::<IdAllocator<DebugItem>>();
        app.init_resource::<EntityMap<DebugItem>>();
        app.add_message::<DebugCommand>();

        app.add_systems(
            Update,
            (
                command::handle_debug_input,
                spawner::handle_spawn_command,
                print::tick_log,
                print::render_metrics,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
