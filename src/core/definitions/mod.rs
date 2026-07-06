pub mod item;

use crate::core::state::GameState;
use bevy::prelude::*;
use item::{ItemDef, ItemRegistry};

pub struct DefinitionPlugin;

impl Plugin for DefinitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            load_item_definitions.run_if(in_state(GameState::Loading)),
        );
    }
}

fn load_item_definitions(mut commands: Commands) {
    let content = std::fs::read_to_string("assets/definitions/items.json")
        .expect("Failed to read assets/definitions/items.json");
    let items: Vec<ItemDef> =
        serde_json::from_str(&content).expect("Failed to parse assets/definitions/items.json");

    let mut registry = ItemRegistry::default();
    for item in items {
        if let Err(e) = registry.register(item) {
            panic!("[Definitions] {}", e);
        }
    }

    println!("[Definitions] Loaded {} item definitions", registry.count());
    commands.insert_resource(registry);
}
