pub mod data;

use crate::core::save::data::{SaveData, TickSave, WorldSave};
use crate::core::time::TickResource;
use crate::world::TileGrid;
use bevy::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Message, Debug)]
pub enum SaveCommand {
    Save(String),
    Load(String),
}

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveCommand>();
        app.add_systems(Update, (save_game, load_game));
    }
}

fn slot_path(slot: &str) -> PathBuf {
    let path = PathBuf::from("saves").join(format!("{}.json", slot));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    path
}

fn save_game(mut reader: MessageReader<SaveCommand>, tick: Res<TickResource>, grid: Res<TileGrid>) {
    for cmd in reader.read() {
        let SaveCommand::Save(slot) = cmd else {
            continue;
        };

        let data = SaveData {
            version: data::SAVE_VERSION,
            tick: TickSave {
                tick_count: tick.tick_count,
                paused: tick.paused,
            },
            world: WorldSave {
                width: grid.width(),
                height: grid.height(),
                tiles: grid.to_tile_data(),
            },
            allocators: HashMap::new(),
        };

        let path = slot_path(slot);
        let json = serde_json::to_string_pretty(&data).expect("Failed to serialize save");
        std::fs::write(&path, json).expect("Failed to write save file");
        println!("[Save] Game saved to {}", path.display());
    }
}

fn load_game(
    mut reader: MessageReader<SaveCommand>,
    mut tick: ResMut<TickResource>,
    mut grid: ResMut<TileGrid>,
) {
    for cmd in reader.read() {
        let SaveCommand::Load(slot) = cmd else {
            continue;
        };

        let path = slot_path(slot);
        let json = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                println!("[Save] Failed to read save: {}", e);
                continue;
            }
        };

        let data: SaveData = match serde_json::from_str(&json) {
            Ok(d) => d,
            Err(e) => {
                println!("[Save] Failed to parse save: {}", e);
                continue;
            }
        };

        tick.tick_count = data.tick.tick_count;
        tick.accumulator = data.tick.tick_count as f64 * TickResource::TICK_DURATION;
        tick.paused = data.tick.paused;

        *grid = TileGrid::from_save(data.world.width, data.world.height, &data.world.tiles);

        println!("[Save] Game loaded from {}", path.display());
    }
}
