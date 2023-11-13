use super::camps::Camp;


#[derive(PartialEq, Clone)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
    pub caves: Vec<CaveNode>,
    pub camps: Vec<Camp>,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Tile {
    pub tile: Tiles,
    pub smooth_tile: Option<SmoothTiles>,
    pub durability: i32,
}

#[derive(PartialEq, Clone)]
pub struct CaveNode {
    x: i32,
    y: i32,
    next: Vec<CaveNode>,
    radius: i32,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Tiles {
    /// ------------------- World building -------------------
    Air,
    Grass,
    Dirt,
    Stone,
    HardStone, /// HardStone is a stone that is harder to break than normal stone and forms the deepest layer of the world.

    /// ------------------- Ores -------------------
    /// Ores are scattered throughout the stone and hardstone layers.
    /// They are forming small clusters of 2-5 ores.
    IronOre,
    GoldOre,
    RuneOre,

    /// ------------------- Gems -------------------
    /// Gems are rare and can be found in the hardstone layer inside caves.
    /// They are forming bigger clusters of 5-7 gems.
    Ruby,
    Emerald,
    Sapphire,
    Diamond,

    /// ------------------- Environmental -------------------
    /// Environmental tiles are used to create structures.
    Log,
    Leaves,
}

/// Smooth tiles can be placed on top of other tiles.
#[derive(PartialEq, Copy, Clone)]
pub enum SmoothTiles {
    Pebbles,
    Sand,
    Vine,
    Torch,
    /// ------------------- Cave -------------------
    SpiderWeb,
    SpiderEgg,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = Vec::new();
        let mut caves = Vec::new();
        let mut camps = Vec::new();
        
        for _ in 0..width {
            let mut row = Vec::new();
            for _ in 0..height {
                row.push(Tile {
                    tile: Tiles::Air,
                    smooth_tile: None,
                    durability: 0,
                });
            }
            tiles.push(row);
        }

        Self {
            width,
            height,
            tiles,
            caves,
            camps,
        }
    }
}