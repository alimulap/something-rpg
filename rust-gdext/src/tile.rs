use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Tile {
    pub name: String,
    pub source: i32,
    pub altas_coord: (i32, i32),
    pub alternative: i32,
    //pub tile_type: TileType,
    pub sides: TileSides,
    // pub allow_neighbor: NeighborSet,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct TileSides {
    pub right: TileSideType,
    pub left: TileSideType,
    pub top: TileSideType,
    pub bottom: TileSideType,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct TileSide {
    pub side: TileSideType,
    pub neighbor: TileSideType,
}

impl Deref for TileSide {
    type Target = TileSideType;

    fn deref(&self) -> &Self::Target {
        &self.side
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum TileSideType {
    DirtFull,
    DirtMiddle,
    DirtRight,
    DirtLeft,
    DirtTop,
    DirtBottom,
    GrassFull,
    GrassMiddle,
    GrassRight,
    GrassLeft,
    GrassTop,
    GrassBottom,
    Void,
}
