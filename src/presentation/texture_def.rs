use crate::world::SEED;
use crate::world::hash2d;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use serde::Deserialize;

#[derive(Deserialize)]
struct TextureMeta {
    tile_width: u32,
    tile_height: u32,
    variants: Vec<VariantMeta>,
    rotation: Option<bool>,
}

#[derive(Deserialize)]
struct VariantMeta {
    name: String,
    weight: u32,
}

#[derive(Resource)]
pub struct TextureDefs {
    pub dirt: TextureDef,
}

pub struct TextureDef {
    pub atlas: Handle<Image>,
    pub weights: Vec<u32>,
    pub total_weight: u32,
    pub variant_count: u32,
}

impl TextureDef {
    pub fn pick_variant(&self, x: u32, y: u32) -> usize {
        let roll = hash2d(x, y, SEED) % self.total_weight;
        let mut acc = 0;
        for (i, &w) in self.weights.iter().enumerate() {
            acc += w;
            if roll < acc {
                return i;
            }
        }
        self.weights.len() - 1
    }
}

pub fn load_texture_defs(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let content = std::fs::read_to_string("assets/textures/dirt.json")
        .expect("Failed to read assets/textures/dirt.json");
    let meta: TextureMeta =
        serde_json::from_str(&content).expect("Failed to parse assets/textures/dirt.json");

    let weights: Vec<u32> = meta.variants.iter().map(|v| v.weight).collect();
    let total_weight: u32 = weights.iter().sum();

    info!("[TextureDefs] Loaded dirt ({} variants)", weights.len());

    let atlas = generate_dirt_atlas(
        &mut images,
        weights.len() as u32,
        meta.tile_width,
        meta.tile_height,
    );

    commands.insert_resource(TextureDefs {
        dirt: TextureDef {
            atlas,
            weights,
            total_weight,
            variant_count: meta.variants.len() as u32,
        },
    });
}

fn generate_dirt_atlas(
    images: &mut Assets<Image>,
    variant_count: u32,
    tile_w: u32,
    tile_h: u32,
) -> Handle<Image> {
    let atlas_w = variant_count * tile_w;
    let atlas_h = tile_h;
    let pixel_count = (atlas_w * atlas_h) as usize;
    let mut data = vec![0u8; pixel_count * 4];

    for variant in 0..variant_count {
        let base = [139u8, 94, 60];
        let shift: i16 = match variant {
            0 => 0,
            1 => 20,
            2 => -15,
            _ => 10,
        };
        let r = (base[0] as i16 + shift).clamp(0, 255) as u8;
        let g = (base[1] as i16 + shift).clamp(0, 255) as u8;
        let b = (base[2] as i16 + shift).clamp(0, 255) as u8;

        for py in 0..tile_h {
            for px in 0..tile_w {
                let idx = ((py * atlas_w + variant * tile_w + px) * 4) as usize;
                data[idx] = r;
                data[idx + 1] = g;
                data[idx + 2] = b;
                data[idx + 3] = 255;
            }
        }
    }

    images.add(Image::new(
        Extent3d {
            width: atlas_w,
            height: atlas_h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ))
}
