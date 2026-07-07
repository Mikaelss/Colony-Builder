#![allow(dead_code)]

pub mod terrain;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use terrain::TerrainType;

pub const GRID_WIDTH: u32 = 275;
pub const GRID_HEIGHT: u32 = 275;
pub const SEED: u32 = 0;

pub fn hash2d(x: u32, y: u32, seed: u32) -> u32 {
    let h = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(seed);
    h.wrapping_mul(h.wrapping_add(0x9e3779b9)) ^ h >> 13
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub terrain: TerrainType,
}

#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct TileGrid {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl TileGrid {
    pub fn from_tiles(tiles: Vec<Tile>, width: u32, height: u32) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

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

    pub fn from_save(width: u32, height: u32, tile_data: &[u8]) -> Self {
        let tiles = tile_data
            .iter()
            .map(|&b| Tile {
                terrain: TerrainType::from_u8(b).expect("Invalid tile data"),
            })
            .collect();
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn to_tile_data(&self) -> Vec<u8> {
        self.tiles.iter().map(|t| t.terrain.to_u8()).collect()
    }

    fn is_water_border(x: u32, y: u32, w: u32, h: u32) -> bool {
        x < 3 || x >= w - 3 || y < 3 || y >= h - 3
    }

    fn is_dirt_border(x: u32, y: u32, w: u32, h: u32) -> bool {
        x < 5 || x >= w - 5 || y < 5 || y >= h - 5
    }

    fn count_stone_neighbors(tiles: &[Tile], x: u32, y: u32, w: u32) -> u32 {
        let mut count = 0;
        for dy in 0..=2 {
            for dx in 0..=2 {
                if dx == 1 && dy == 1 {
                    continue;
                }
                let nx = x as i64 + dx as i64 - 1;
                let ny = y as i64 + dy as i64 - 1;
                if nx < 0 || ny < 0 || nx >= w as i64 {
                    continue;
                }
                let ny = ny as u32;
                let nx = nx as u32;
                if ny * w + nx < tiles.len() as u32
                    && tiles[(ny * w + nx) as usize].terrain == TerrainType::Stone
                {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn generate(width: u32, height: u32) -> Self {
        let mut tiles = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let terrain = if Self::is_water_border(x, y, width, height) {
                    TerrainType::Water
                } else if Self::is_dirt_border(x, y, width, height) {
                    TerrainType::Dirt
                } else if hash2d(x, y, SEED) % 1000 < 40 {
                    TerrainType::Stone
                } else {
                    TerrainType::Dirt
                };
                tiles.push(Tile { terrain });
            }
        }

        let mut expanded = tiles.clone();
        for y in 0..height {
            for x in 0..width {
                if Self::is_water_border(x, y, width, height)
                    || Self::is_dirt_border(x, y, width, height)
                {
                    continue;
                }
                let idx = (y * width + x) as usize;
                if tiles[idx].terrain == TerrainType::Dirt
                    && Self::count_stone_neighbors(&tiles, x, y, width) >= 3
                {
                    expanded[idx].terrain = TerrainType::Stone;
                }
            }
        }

        Self {
            tiles: expanded,
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
