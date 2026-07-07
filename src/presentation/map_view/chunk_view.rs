use crate::presentation::sprites::TILE_SIZE;
use crate::world::terrain::TerrainType;
use crate::world::{GRID_HEIGHT, GRID_WIDTH, SEED, TileGrid, hash2d};
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

const CHUNK_SIZE_TILES: u32 = 32;

pub fn spawn_chunks(
    mut commands: Commands,
    grid: Res<TileGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let chunks_x = GRID_WIDTH.div_ceil(CHUNK_SIZE_TILES);
    let chunks_y = GRID_HEIGHT.div_ceil(CHUNK_SIZE_TILES);

    let white_material = materials.add(ColorMaterial::default());
    let mut spawned = 0;

    for cy in 0..chunks_y {
        for cx in 0..chunks_x {
            let offset_x = cx as f32 * CHUNK_SIZE_TILES as f32 * TILE_SIZE;
            let offset_y = cy as f32 * CHUNK_SIZE_TILES as f32 * TILE_SIZE;

            for terrain in [TerrainType::Water, TerrainType::Dirt, TerrainType::Stone] {
                let mesh = build_terrain_mesh(&grid, cx, cy, terrain);
                let Some(mesh) = mesh else { continue };

                let mesh_handle = meshes.add(mesh);
                commands.spawn((
                    Mesh2d(mesh_handle),
                    MeshMaterial2d(white_material.clone()),
                    Transform::from_xyz(offset_x, offset_y, 0.0),
                    GlobalTransform::default(),
                    Visibility::default(),
                ));
                spawned += 1;
            }
        }
    }

    info!("[ChunkView] Spawned {spawned} chunk meshes ({chunks_x}×{chunks_y} chunks)");
}

fn build_terrain_mesh(grid: &TileGrid, cx: u32, cy: u32, target: TerrainType) -> Option<Mesh> {
    let start_x = cx * CHUNK_SIZE_TILES;
    let start_y = cy * CHUNK_SIZE_TILES;
    let end_x = (start_x + CHUNK_SIZE_TILES).min(grid.width());
    let end_y = (start_y + CHUNK_SIZE_TILES).min(grid.height());

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for ly in 0..(end_y - start_y) {
        for lx in 0..(end_x - start_x) {
            let wx = start_x + lx;
            let wy = start_y + ly;
            let tile = match grid.get(wx, wy) {
                Some(t) => t,
                None => continue,
            };
            if tile.terrain != target {
                continue;
            }

            let vi = positions.len() as u32;
            let x0 = lx as f32 * TILE_SIZE;
            let y0 = ly as f32 * TILE_SIZE;
            let x1 = x0 + TILE_SIZE;
            let y1 = y0 + TILE_SIZE;

            positions.extend_from_slice(&[
                [x0, y0, 0.0],
                [x1, y0, 0.0],
                [x1, y1, 0.0],
                [x0, y1, 0.0],
            ]);

            let color = tile_color(target, wx, wy);
            colors.extend_from_slice(&[color, color, color, color]);

            indices.extend_from_slice(&[vi, vi + 1, vi + 2, vi, vi + 2, vi + 3]);
        }
    }

    if positions.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    Some(mesh)
}

fn tile_color(terrain: TerrainType, x: u32, y: u32) -> [f32; 4] {
    let variant = (hash2d(x, y, SEED) % 4) as usize;
    let base: [f32; 3] = match terrain {
        TerrainType::Dirt => [139.0 / 255.0, 94.0 / 255.0, 60.0 / 255.0],
        TerrainType::Stone => [128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0],
        TerrainType::Water => [30.0 / 255.0, 144.0 / 255.0, 1.0],
    };
    let shift: f32 = match variant {
        0 => 0.0,
        1 => 0.08,
        2 => -0.06,
        3 => 0.04,
        _ => 0.0,
    };
    [
        (base[0] + shift).clamp(0.0, 1.0),
        (base[1] + shift).clamp(0.0, 1.0),
        (base[2] + shift).clamp(0.0, 1.0),
        1.0,
    ]
}
