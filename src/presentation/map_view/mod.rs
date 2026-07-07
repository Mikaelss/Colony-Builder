pub mod chunk_view;

use crate::presentation::sprites::TILE_SIZE;
use crate::world::{GRID_HEIGHT, GRID_WIDTH};
use bevy::prelude::*;

pub fn setup_map_view(mut commands: Commands) {
    let center_x = GRID_WIDTH as f32 * TILE_SIZE / 2.0;
    let center_y = GRID_HEIGHT as f32 * TILE_SIZE / 2.0;

    commands.spawn((
        Camera2d,
        Transform::from_xyz(center_x, center_y, 0.0),
        GlobalTransform::default(),
    ));

    info!("[MapView] Camera ready.");
}
