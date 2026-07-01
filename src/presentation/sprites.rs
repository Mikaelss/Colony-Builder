use crate::world::terrain::TerrainType;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub const TILE_SIZE: f32 = 64.0;
pub const SPRITE_PX: u32 = 64;
const VARIANTS: usize = 4;

#[derive(Resource)]
pub struct SpriteRegistry {
    dirt_sprites: Vec<Handle<Image>>,
    stone_sprites: Vec<Handle<Image>>,
    water_sprites: Vec<Handle<Image>>,
}

impl SpriteRegistry {
    pub fn generate(images: &mut Assets<Image>) -> Self {
        Self {
            dirt_sprites: generate_variants(images, TerrainType::Dirt),
            stone_sprites: generate_variants(images, TerrainType::Stone),
            water_sprites: generate_variants(images, TerrainType::Water),
        }
    }

    pub fn get(&self, terrain: TerrainType, x: u32, y: u32) -> Handle<Image> {
        let i = pick_variant(x, y);
        self.sprites_for(terrain)[i].clone()
    }

    fn sprites_for(&self, terrain: TerrainType) -> &[Handle<Image>] {
        match terrain {
            TerrainType::Dirt => &self.dirt_sprites,
            TerrainType::Stone => &self.stone_sprites,
            TerrainType::Water => &self.water_sprites,
        }
    }
}

impl FromWorld for SpriteRegistry {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        Self::generate(&mut images)
    }
}

fn generate_variants(images: &mut Assets<Image>, terrain: TerrainType) -> Vec<Handle<Image>> {
    let base = base_color(terrain);
    (0..VARIANTS)
        .map(|v| {
            let color = variant_color(base, v);
            let image = make_tile_texture(color);
            images.add(image)
        })
        .collect()
}

fn pick_variant(x: u32, y: u32) -> usize {
    let hash = x.wrapping_mul(7).wrapping_add(y.wrapping_mul(31));
    hash as usize % VARIANTS
}

fn base_color(terrain: TerrainType) -> [u8; 3] {
    match terrain {
        TerrainType::Dirt => [139, 94, 60],
        TerrainType::Stone => [128, 128, 128],
        TerrainType::Water => [30, 144, 255],
    }
}

fn variant_color(base: [u8; 3], variant: usize) -> [u8; 4] {
    let shift: i16 = match variant {
        0 => 0,
        1 => 20,
        2 => -15,
        3 => 10,
        _ => 0,
    };
    [
        (base[0] as i16 + shift).clamp(0, 255) as u8,
        (base[1] as i16 + shift).clamp(0, 255) as u8,
        (base[2] as i16 + shift).clamp(0, 255) as u8,
        255,
    ]
}

fn make_tile_texture(color: [u8; 4]) -> Image {
    let pixel_count = (SPRITE_PX * SPRITE_PX) as usize;
    let mut data = vec![0u8; pixel_count * 4];
    for chunk in data.chunks_exact_mut(4) {
        chunk.copy_from_slice(&color);
    }
    Image::new(
        Extent3d {
            width: SPRITE_PX,
            height: SPRITE_PX,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
}
