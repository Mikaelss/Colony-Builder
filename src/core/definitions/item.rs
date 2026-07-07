use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemCategory {
    Raw,
    Refined,
    Tool,
    Food,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub category: ItemCategory,
    pub stack_size: u32,
}

#[derive(Resource, Debug, Default)]
pub struct ItemRegistry {
    items: HashMap<String, ItemDef>,
}

impl ItemRegistry {
    pub fn register(&mut self, def: ItemDef) -> Result<(), String> {
        if self.items.contains_key(&def.id) {
            return Err(format!("Duplicate item id: '{}'", def.id));
        }
        self.items.insert(def.id.clone(), def);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&ItemDef> {
        self.items.get(id)
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn all(&self) -> impl Iterator<Item = &ItemDef> {
        self.items.values()
    }
}
