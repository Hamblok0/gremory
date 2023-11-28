use crate::tile::TileType;
use ggez::glam::*;

use crate::prelude::*;

const UWIDTH: usize = WINDOW_WIDTH as usize;
const UHEIGHT: usize = WINDOW_HEIGHT as usize;

#[derive(Debug)]
pub struct Map {
    pub map: Vec<TileType>,
    pub player_start: Vec2,
}

pub fn idx(x: usize, y: usize) -> Option<usize> {
    if x >= 0 && x <= UWIDTH && y >= 0 && y <= UHEIGHT {
       return Some((y * NUM_X) + x)
    } 
    None 
}

impl Map {
    pub fn new() -> Self {
        let map = vec![TileType::Floor; NUM_TILES];
        let player_start = vec2((NUM_X / 2) as f32, (NUM_Y / 2) as f32);
        Self { map, player_start }
    }

    pub fn build(&mut self) {
        let iy = NUM_Y - 1;
        let ix = NUM_X - 1;

        for y in 0..NUM_Y {
            let idx_top = idx(0, y).unwrap();
            let idx_bottom = idx(ix, y).unwrap();
            self.map[idx_top] = TileType::Wall;
            self.map[idx_bottom] = TileType::Wall;
        }
        for x in 0..NUM_X {
            let idx_left = idx(x, 0).unwrap();
            let idx_right = idx(x, iy).unwrap();
            self.map[idx_left] = TileType::Wall;
            self.map[idx_right] = TileType::Wall;
        }
    }

    pub fn in_bounds(&mut self, coords: &Vec2) -> bool {
        let ix = coords.x as usize;
        let iy = coords.y as usize;
        self.map[idx(ix, iy).unwrap()] != TileType::Wall
    }
}
