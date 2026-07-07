use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const SAVE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub tick: TickSave,
    pub world: WorldSave,
    pub allocators: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize)]
pub struct TickSave {
    pub tick_count: u64,
    pub paused: bool,
}

#[derive(Serialize, Deserialize)]
pub struct WorldSave {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<u8>,
}
