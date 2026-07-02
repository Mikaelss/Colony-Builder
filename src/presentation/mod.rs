pub mod dummy;
pub mod map_view;
pub mod metrics;
pub mod sprites;

use crate::core::state::GameState;
use bevy::prelude::*;
use dummy::print_tick;
use metrics::log_render_metrics;
use sprites::SpriteRegistry;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteRegistry>();
        app.add_systems(Update, print_tick.run_if(in_state(GameState::Playing)));
        app.add_systems(
            Update,
            log_render_metrics.run_if(in_state(GameState::Playing)),
        );
        app.add_systems(OnEnter(GameState::Playing), map_view::setup_map_view);
    }
}
