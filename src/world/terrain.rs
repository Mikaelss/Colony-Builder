use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Dirt,
    Stone,
    Water,
}

impl TerrainType {
    pub fn to_u8(self) -> u8 {
        match self {
            TerrainType::Dirt => 0,
            TerrainType::Stone => 1,
            TerrainType::Water => 2,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TerrainType::Dirt),
            1 => Some(TerrainType::Stone),
            2 => Some(TerrainType::Water),
            _ => None,
        }
    }
}
