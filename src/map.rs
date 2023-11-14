use ggez::glam::*;
use ggez::Context;
use ggez::graphics::{ Image, DrawParam, Rect, Canvas };

const WINDOW_HEIGHT: i32 = 1080;
const WINDOW_WIDTH: i32 = 1920;
const NUM_X: usize = (WINDOW_WIDTH as usize) / 32;
const NUM_Y: usize = (WINDOW_HEIGHT as usize) / 32;
const NUM_TILES: usize = NUM_Y * NUM_X;

pub struct Map {
    tile_map: Image,
    map: Vec<TileType>
}

#[derive(Debug, Clone, PartialEq)]
enum TileType {
    Floor,
    Wall
}

impl Map {
    pub fn new(ctx: &mut Context) -> Self {
        let tile_map = Image::from_path(ctx, "/tiles.png").unwrap();
        let map = vec![TileType::Floor; NUM_TILES];
        Self {
            tile_map,
            map
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        (y * NUM_X) + x
    }

    pub fn build(&mut self) {
        let iy = NUM_Y - 1;
        let ix = NUM_X - 1;
        for y in 0..NUM_Y {
            let idx_top = self.idx(0, y);
            let idx_bottom = self.idx(ix, y);
            self.map[idx_top] = TileType::Wall;
            self.map[idx_bottom] = TileType::Wall;
        }
        for x in 0..NUM_X {
            let idx_left = self.idx(x, 0);
            let idx_right = self.idx(x, iy);
            self.map[idx_left] = TileType::Wall;
            self.map[idx_right] = TileType::Wall;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for y in 0..NUM_Y {
            for x in 0..NUM_X {
                let idx = self.idx(x, y);
                let fx = (x * 32) as f32;
                let yx = (y * 32) as f32;
                match self.map[idx] {
                    TileType::Floor => {
                        let floor_tile = &self.tile_map.uv_rect(8 * 32, 2 * 32, 32, 32);
                        canvas.draw(&self.tile_map, DrawParam::default()
                        .dest(vec2(fx, yx))
                        .src(*floor_tile))
                    },
                    TileType::Wall => {
                        let wall_tile = &self.tile_map.uv_rect(0, 2 * 32, 32, 32);
                        canvas.draw(&self.tile_map, DrawParam::default()
                        .dest(vec2(fx, yx))
                        .src(*wall_tile))
                    }
                }
            }
        }
    }
}
