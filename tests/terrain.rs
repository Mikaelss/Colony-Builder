use Colony_Builder::world::terrain::TerrainType;

#[test]
fn to_u8_dirt() {
    assert_eq!(TerrainType::Dirt.to_u8(), 0);
}

#[test]
fn to_u8_stone() {
    assert_eq!(TerrainType::Stone.to_u8(), 1);
}

#[test]
fn to_u8_water() {
    assert_eq!(TerrainType::Water.to_u8(), 2);
}

#[test]
fn from_u8_dirt() {
    assert_eq!(TerrainType::from_u8(0), Some(TerrainType::Dirt));
}

#[test]
fn from_u8_stone() {
    assert_eq!(TerrainType::from_u8(1), Some(TerrainType::Stone));
}

#[test]
fn from_u8_water() {
    assert_eq!(TerrainType::from_u8(2), Some(TerrainType::Water));
}

#[test]
fn from_u8_invalid() {
    assert_eq!(TerrainType::from_u8(3), None);
    assert_eq!(TerrainType::from_u8(255), None);
}

#[test]
fn roundtrip_all() {
    for value in 0..=2u8 {
        let terrain = TerrainType::from_u8(value).unwrap();
        assert_eq!(terrain.to_u8(), value);
    }
}

#[test]
fn dirt_not_equal_stone() {
    assert_ne!(TerrainType::Dirt, TerrainType::Stone);
}

#[test]
fn stone_not_equal_water() {
    assert_ne!(TerrainType::Stone, TerrainType::Water);
}
