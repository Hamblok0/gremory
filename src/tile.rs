// TODO: implement this as an instanced array

use ggez::graphics::{Image, Rect};
use ggez::Context;

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_map: Image,
    pub tiles: Vec<Rect>,
}

#[derive(Debug, Clone)]
pub enum TileType {
    Floor,
    Wall,
    Player,
    Enemy,
}

impl Tile {
    pub fn new(ctx: &mut Context) -> Self {
        let mut interface: Vec<Rect> = Vec::new();
        let tile_map = Image::from_path(ctx, "/tiles.png").unwrap();
        // Just pull the 4 tiles we need right now for prototyping purposes
        // Eventually we'll have to find a nice way to pull in every tile,
        // but for now just make them from the top to bottom order in TileType

        for i in 0..4 {
            let tile: Rect;

            match i {
                0 => {
                    tile = tile_map.uv_rect(8 * 32, 2 * 32, 32, 32);
                    interface.push(tile);
                }
                1 => {
                    tile = tile_map.uv_rect(0, 2 * 32, 32, 32);
                    interface.push(tile);
                }
                2 => {
                    tile = tile_map.uv_rect(0, 3 * 32, 32, 32);
                    interface.push(tile);
                }
                3 => {
                    tile = tile_map.uv_rect(6 * 32, 3 * 32, 32, 32);
                    interface.push(tile);
                }
                _ => println!("Unexpected value given to tile interface creator"),
            }
        }
        Self {
            tile_map,
            tiles: interface,
        }
    }

    pub fn get_tile(&self, tile: TileType) -> &Rect {
        match tile {
            TileType::Floor => &self.tiles[0],
            TileType::Wall => &self.tiles[1],
            TileType::Player => &self.tiles[2],
            TileType::Enemy => &self.tiles[3],
        }
    }
}
