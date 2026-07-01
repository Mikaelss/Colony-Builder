#![allow(dead_code)]

pub mod terrain;

use bevy::prelude::*;
use terrain::TerrainType;

pub const GRID_WIDTH: u32 = 275;
pub const GRID_HEIGHT: u32 = 275;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub terrain: TerrainType,
}

#[derive(Resource, Debug)]
pub struct TileGrid {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl TileGrid {
    fn index(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Tile> {
        self.index(x, y).map(|i| &self.tiles[i])
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        self.index(x, y).map(|i| &mut self.tiles[i])
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }

    fn generate(width: u32, height: u32) -> Self {
        let mut tiles = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let terrain = if x < 3 || x >= width - 3 || y < 3 || y >= height - 3 {
                    TerrainType::Water
                } else if x < 5 || x >= width - 5 || y < 5 || y >= height - 5 {
                    TerrainType::Dirt
                } else if (x + y * 3) % 7 == 0 {
                    TerrainType::Stone
                } else {
                    TerrainType::Dirt
                };
                tiles.push(Tile { terrain });
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let grid = TileGrid::generate(GRID_WIDTH, GRID_HEIGHT);
        app.insert_resource(grid);
    }
}
