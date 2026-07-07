use crate::world::SEED;
use crate::world::hash2d;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct TextureMeta {
    base_tile_size: u32,
    render_size: u32,
    variations: u32,
    layout_type: String,
}

#[derive(Resource)]
pub struct TextureDefs {
    pub dirt: TextureDef,
}

pub struct TextureDef {
    pub atlas: Handle<Image>,
    pub variant_count: u32,
}

impl TextureDef {
    pub fn pick_variant(&self, x: u32, y: u32) -> usize {
        (hash2d(x, y, SEED) % self.variant_count) as usize
    }
}

pub fn load_texture_defs(mut commands: Commands, asset_server: Res<AssetServer>) {
    let content = std::fs::read_to_string("assets/textures/dirt.json")
        .expect("Failed to read assets/textures/dirt.json");
    let meta: TextureMeta =
        serde_json::from_str(&content).expect("Failed to parse assets/textures/dirt.json");

    info!("[TextureDefs] Loaded dirt ({} variations)", meta.variations);

    let atlas = asset_server.load("textures/dirt.png");

    commands.insert_resource(TextureDefs {
        dirt: TextureDef {
            atlas,
            variant_count: meta.variations,
        },
    });
}
