pub mod map_view;
pub mod sprites;
pub mod texture_def;

use crate::core::state::GameState;
use bevy::prelude::*;
use texture_def::load_texture_defs;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_texture_defs);
        app.add_systems(
            OnEnter(GameState::Playing),
            (map_view::setup_map_view, map_view::chunk_view::spawn_chunks),
        );
    }
}
