use Colony_Builder::world::terrain::TerrainType;
use Colony_Builder::world::{SEED, hash2d};

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

#[test]
fn alpha_always_one() {
    for terrain in [TerrainType::Dirt, TerrainType::Stone, TerrainType::Water] {
        for i in 0..20 {
            let color = tile_color(terrain, i * 7, i * 13);
            assert_eq!(color[3], 1.0);
        }
    }
}

#[test]
fn colors_clamped_0_to_1() {
    for terrain in [TerrainType::Dirt, TerrainType::Stone, TerrainType::Water] {
        for i in 0..50 {
            let color = tile_color(terrain, i, i * 3);
            for channel in 0..3 {
                assert!(
                    color[channel] >= 0.0 && color[channel] <= 1.0,
                    "channel {} out of range: {}",
                    channel,
                    color[channel]
                );
            }
        }
    }
}

#[test]
fn color_deterministic() {
    assert_eq!(
        tile_color(TerrainType::Dirt, 10, 20),
        tile_color(TerrainType::Dirt, 10, 20)
    );
}

#[test]
fn water_blue_dominant() {
    let color = tile_color(TerrainType::Water, 5, 5);
    assert!(
        color[2] > color[0] && color[2] > color[1],
        "water should be blue-dominant"
    );
}

#[test]
fn stone_grayish() {
    let color = tile_color(TerrainType::Stone, 0, 0);
    let diff = (color[0] - color[1]).abs().max((color[1] - color[2]).abs());
    assert!(diff < 0.05, "stone channels should be close: {:?}", color);
}

#[test]
fn variants_produce_different_colors() {
    let mut first: Option<[f32; 4]> = None;
    for i in 0..20 {
        let c = tile_color(TerrainType::Dirt, i * 37, i * 53);
        match first {
            None => first = Some(c),
            Some(prev) => {
                if c != prev {
                    return;
                }
            }
        }
    }
    panic!("all 20 samples produced the same color");
}
