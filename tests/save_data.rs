use Colony_Builder::core::save::data::{SAVE_VERSION, SaveData, TickSave, WorldSave};
use std::collections::HashMap;

#[test]
fn save_version_is_1() {
    assert_eq!(SAVE_VERSION, 1);
}

#[test]
fn tick_save_serde_roundtrip() {
    let original = TickSave {
        tick_count: 150000,
        paused: false,
    };
    let json = serde_json::to_string(&original).unwrap();
    let restored: TickSave = serde_json::from_str(&json).unwrap();
    assert_eq!(original.tick_count, restored.tick_count);
    assert_eq!(original.paused, restored.paused);
}

#[test]
fn world_save_serde_roundtrip() {
    let original = WorldSave {
        width: 275,
        height: 275,
        tiles: vec![0u8, 1, 2, 0, 1, 2],
    };
    let json = serde_json::to_string(&original).unwrap();
    let restored: WorldSave = serde_json::from_str(&json).unwrap();
    assert_eq!(original.width, restored.width);
    assert_eq!(original.height, restored.height);
    assert_eq!(original.tiles, restored.tiles);
}

#[test]
fn save_data_serde_roundtrip() {
    let original = SaveData {
        version: SAVE_VERSION,
        tick: TickSave {
            tick_count: 12345,
            paused: true,
        },
        world: WorldSave {
            width: 100,
            height: 200,
            tiles: vec![0, 1, 2],
        },
        allocators: HashMap::from([("test".into(), 42u64)]),
    };
    let json = serde_json::to_string_pretty(&original).unwrap();
    let restored: SaveData = serde_json::from_str(&json).unwrap();
    assert_eq!(original.version, restored.version);
    assert_eq!(original.tick.tick_count, restored.tick.tick_count);
    assert_eq!(original.tick.paused, restored.tick.paused);
    assert_eq!(original.world.width, restored.world.width);
    assert_eq!(original.world.height, restored.world.height);
    assert_eq!(original.world.tiles, restored.world.tiles);
    assert_eq!(original.allocators, restored.allocators);
}

#[test]
fn save_data_empty_allocators() {
    let data = SaveData {
        version: SAVE_VERSION,
        tick: TickSave {
            tick_count: 0,
            paused: false,
        },
        world: WorldSave {
            width: 10,
            height: 10,
            tiles: vec![],
        },
        allocators: HashMap::new(),
    };
    let json = serde_json::to_string(&data).unwrap();
    let restored: SaveData = serde_json::from_str(&json).unwrap();
    assert!(restored.allocators.is_empty());
}
