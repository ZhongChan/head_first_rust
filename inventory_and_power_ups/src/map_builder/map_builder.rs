use crate::prelude::*;

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Wall => to_cp437('.'),
            TileType::Floor => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Wall => to_cp437(';'),
            TileType::Floor => to_cp437('"'),
            TileType::Exit => to_cp437('>'),
        }
    }
}
