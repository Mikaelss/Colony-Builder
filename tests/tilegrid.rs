use Colony_Builder::world::terrain::TerrainType;
use Colony_Builder::world::{GRID_HEIGHT, GRID_WIDTH, TileGrid};

fn make_grid(width: u32, height: u32) -> TileGrid {
    let tiles: Vec<_> = (0..width * height)
        .map(|i| {
            let x = i % width;
            let y = i / width;
            let terrain = if x < 3 || x >= width - 3 || y < 3 || y >= height - 3 {
                TerrainType::Water
            } else if x < 5 || x >= width - 5 || y < 5 || y >= height - 5 {
                TerrainType::Dirt
            } else {
                TerrainType::Dirt
            };
            Colony_Builder::world::Tile { terrain }
        })
        .collect();
    TileGrid::from_tiles(tiles, width, height)
}

#[test]
fn default_size_constants() {
    assert_eq!(GRID_WIDTH, 275);
    assert_eq!(GRID_HEIGHT, 275);
}

#[test]
fn get_valid_coordinates() {
    let grid = make_grid(10, 10);
    assert!(grid.get(0, 0).is_some());
    assert!(grid.get(9, 9).is_some());
}

#[test]
fn get_out_of_bounds() {
    let grid = make_grid(10, 10);
    assert!(grid.get(10, 0).is_none());
    assert!(grid.get(0, 10).is_none());
}

#[test]
fn get_water_border() {
    let grid = make_grid(10, 10);
    let tile = grid.get(0, 0).unwrap();
    assert_eq!(tile.terrain, TerrainType::Water);
}

#[test]
fn get_interior_not_water() {
    let grid = make_grid(10, 10);
    let tile = grid.get(5, 5).unwrap();
    assert_ne!(tile.terrain, TerrainType::Water);
}

#[test]
fn to_tile_data_length() {
    let grid = make_grid(10, 10);
    assert_eq!(grid.to_tile_data().len(), 100);
}

#[test]
fn from_save_restores_exact_grid() {
    let original = make_grid(10, 10);
    let data = original.to_tile_data();
    let restored = TileGrid::from_save(10, 10, &data);
    assert_eq!(original.width(), restored.width());
    assert_eq!(original.height(), restored.height());
    for y in 0..10 {
        for x in 0..10 {
            assert_eq!(
                original.get(x, y).unwrap().terrain,
                restored.get(x, y).unwrap().terrain,
                "mismatch at ({}, {})",
                x,
                y
            );
        }
    }
}

#[test]
#[should_panic(expected = "Invalid tile data")]
fn from_save_invalid_data_panics() {
    TileGrid::from_save(1, 1, &[99u8]);
}
