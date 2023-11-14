use ggez::glam::vec2;
use ggez::Context;
use ggez::graphics::{ Image, DrawParam, Rect, Canvas };

const WINDOW_HEIGHT: i32 = 1080;
const WINDOW_WIDTH: i32 = 1920;
const NUM_X: usize = (WINDOW_WIDTH as usize) / 32;
const NUM_Y: usize = (WINDOW_HEIGHT as usize) / 32;
const NUM_TILES: usize = NUM_X * NUM_Y;

pub struct Map {
    tile_map: Image,
    tiles: Vec<TileType>
}

#[derive(Clone, PartialEq)]
enum TileType {
    Floor,
    Wall
}

impl Map {
    pub fn new(ctx: &mut Context) -> Self {
        let tile_map = Image::from_path(ctx, "/tiles.png").unwrap();
        let tiles = vec![TileType::Floor; NUM_TILES];
        Self {
            tile_map,
            tiles
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * (NUM_X) + x
    }

    pub fn build(&mut self) {
        for y in NUM_Y.. {
            let idx_top = self.idx(0, y);
            let idx_bottom = self.idx(NUM_X - 1, y);
            self.tiles[idx_top] = TileType::Wall;
            self.tiles[idx_bottom] = TileType::Wall;
        }
        for x in NUM_X.. {
            let idx_left = self.idx(x, 0);
            let idx_right = self.idx(x, NUM_Y - 1);
            self.tiles[idx_left] = TileType::Wall;
            self.tiles[idx_right] = TileType::Wall;
        }

    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
        for y in (WINDOW_HEIGHT..).step_by(32) {
            for x in (WINDOW_WIDTH..).step_by(32) {
                let idx = self.idx((y / 32) as usize, (x / 32) as usize);
                match self.tiles[idx] {
                    TileType::Floor => {
                        canvas.draw(&self.tile_map, DrawParam::default()
                        .dest(vec2(x as f32, y as f32))
                        .src(Rect {
                            x: 8. * 32.,
                            y: 2. * 32.,
                            w: 32.,
                            h: 32.
                        }))
                    },
                    TileType::Wall => {
                        canvas.draw(&self.tile_map, DrawParam::default()
                        .dest(vec2(x as f32, y as f32))
                        .src(Rect {
                            x: 0.,
                            y: 2. * 32.,
                            w: 32.,
                            h: 32.
                        }))
                    }
                }
            }
        }
       
        
    }
}
