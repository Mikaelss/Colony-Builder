use crate::presentation::sprites::{SpriteRegistry, TILE_SIZE};
use crate::world::{GRID_HEIGHT, GRID_WIDTH, TileGrid};
use bevy::prelude::*;

pub fn setup_map_view(mut commands: Commands, grid: Res<TileGrid>, registry: Res<SpriteRegistry>) {
    let center_x = GRID_WIDTH as f32 * TILE_SIZE / 2.0;
    let center_y = GRID_HEIGHT as f32 * TILE_SIZE / 2.0;

    commands.spawn((
        Camera2d,
        Transform::from_xyz(center_x, center_y, 0.0),
        GlobalTransform::default(),
    ));

    println!(
        "[MapView] Spawning {} tiles...",
        GRID_WIDTH as u64 * GRID_HEIGHT as u64
    );

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let tile = grid.get(x, y).unwrap();
            let texture = registry.get(tile.terrain, x, y);
            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_xyz(
                    x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                    y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                    0.0,
                ),
                GlobalTransform::default(),
                Visibility::default(),
            ));
        }
    }
}
