// TODO: implement this as an instanced array

use ggez::graphics::{ Rect, Image };
use ggez::Context;

#[derive(Clone, Debug)]
pub struct Tile {
    pub tiles: Vec<Rect>
}

#[derive(Debug, Clone)]
pub enum TileType {
    Floor,
    Wall,
    Player,
    Enemy
}

impl Tile {
    pub fn new(ctx: &mut Context) -> Self {
        let mut interface_vec: Vec<Rect> = Vec::new();
        create_tile_interface(ctx, &mut interface_vec);

        Self {
            tiles: interface_vec
        }
    }

    pub fn get_tile(&self, tile: TileType) -> &Rect {
        match tile {
            TileType::Floor => &self.tiles[0],
            TileType::Wall => &self.tiles[1],
            TileType::Player => &self.tiles[2],
            TileType::Enemy => &self.tiles[3]
        }
    }
}

fn create_tile_interface(ctx: &mut Context, vec: &mut Vec<Rect>) {
    // Just pull the 4 tiles we need right now for prototyping purposes
    // Eventually we'll have to find a nice way to pull in every tile,
    // but for now just make them from the top to bottom order in TileType
    let tile_map = Image::from_path(ctx, "/tiles.png").unwrap();
    for i in 0..4 {
        let new_interface: Rect;

        match i {
            0 => {
                new_interface = tile_map.uv_rect(8 * 32, 2 * 32, 32, 32);
                vec.push(new_interface);
            },
            1 => {
                new_interface = tile_map.uv_rect(0, 2 * 32, 32, 32);
                vec.push(new_interface);
            },
            2 => {
                new_interface = tile_map.uv_rect(0, 3 * 32, 32, 32);
                vec.push(new_interface);
            },
            3 => {
                new_interface = tile_map.uv_rect(6 * 32, 3 * 32, 32, 32);
                vec.push(new_interface);
            },
            _ => println!("Unexpected value given to tile interface creator")
        }
    }
}