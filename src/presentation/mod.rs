pub mod map_view;
pub mod sprites;

use crate::core::state::GameState;
use bevy::prelude::*;
use sprites::SpriteRegistry;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteRegistry>();
        app.add_systems(OnEnter(GameState::Playing), map_view::setup_map_view);
    }
}
