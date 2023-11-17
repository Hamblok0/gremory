use ggez::glam::*;
use crate::tile::TileType;

const WINDOW_HEIGHT: i32 = 1080;
const WINDOW_WIDTH: i32 = 1920;
const NUM_X: usize = (WINDOW_WIDTH as usize) / 32;
const NUM_Y: usize = (WINDOW_HEIGHT as usize) / 32;
const NUM_TILES: usize = NUM_Y * NUM_X;

pub struct Map {
    map: Vec<TileType>,
    player_start: Vec2
}

pub fn idx(x: usize, y: usize) -> usize {
    (y * NUM_X) + x
}

impl Map {
    pub fn new() -> Self {
        let map = vec![TileType::Floor; NUM_TILES];
        let player_start = vec2((NUM_X / 2) as f32, (NUM_Y / 2) as f32);
        Self {
            map,
            player_start
        }
    }

    pub fn build(&mut self) {
        let iy = NUM_Y - 1;
        let ix = NUM_X - 1;
        for y in 0..NUM_Y {
            let idx_top = idx(0, y);
            let idx_bottom = idx(ix, y);
            self.map[idx_top] = TileType::Wall;
            self.map[idx_bottom] = TileType::Wall;
        }
        for x in 0..NUM_X {
            let idx_left = idx(x, 0);
            let idx_right = idx(x, iy);
            self.map[idx_left] = TileType::Wall;
            self.map[idx_right] = TileType::Wall;
        }
    }

    // pub fn draw(&self, canvas: &mut Canvas) {
    //     let floor_tile = &self.tile_map.uv_rect(8 * 32, 2 * 32, 32, 32);
    //     let wall_tile = &self.tile_map.uv_rect(0, 2 * 32, 32, 32);
    //     for y in 0..NUM_Y {
    //         for x in 0..NUM_X {
    //             let idx = self.idx(x, y);
    //             let fx = (x * 32) as f32;
    //             let yx = (y * 32) as f32;
    //             match self.map[idx] {
    //                 TileType::Floor => {
    //                     canvas.draw(&self.tile_map, DrawParam::default()
    //                     .dest(vec2(fx, yx))
    //                     .src(*floor_tile))
    //                 },
    //                 TileType::Wall => {
    //                     canvas.draw(&self.tile_map, DrawParam::default()
    //                     .dest(vec2(fx, yx))
    //                     .src(*wall_tile))
    //                 }
    //             }
    //         }
    //     }
    // }
}
