// TODO, put map's render function here, have camera follow character

use ggez::glam::*;
use ggez::Context;
use ggez::graphics::{ DrawParam, Canvas};

use crate::tile::*;
use crate::map::*;

const WINDOW_HEIGHT: i32 = 1080;
const WINDOW_WIDTH: i32 = 1920;
const NUM_X: usize = (WINDOW_WIDTH as usize) / 32;
const NUM_Y: usize = (WINDOW_HEIGHT as usize) / 32;
const NUM_TILES: usize = NUM_Y * NUM_X;
const FWIDTH: f32 = (WINDOW_WIDTH / 2) as f32;
const FHEIGHT: f32 = (WINDOW_HEIGHT / 2) as f32;

pub struct Camera {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub player_position: Vec2
}

impl Camera {
    pub fn new(ctx: &mut Context, player_position: Vec2) -> Self {
        Self {  
            left: player_position.x - FWIDTH,
            right: player_position.x + FWIDTH,
            top: player_position.y - FHEIGHT,
            bottom: player_position.y + FHEIGHT,
            player_position
        }
    }

    pub fn on_player_move(&mut self) {
        self.left = self.player_position.x - FWIDTH;
        self.right = self.player_position.x + FWIDTH;
        self.top = self.player_position.y - FHEIGHT;
        self.bottom = self.player_position.y + FHEIGHT;
    }
    
    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, map: &Vec<TileType>) {
        let tiles = Tile::new(ctx); 

        for y in 0..NUM_Y {
            for x in 0..NUM_X {
                let idx = idx(x, y);
                let fx = (x * 32) as f32;
                let yx = (y * 32) as f32;
                match map[idx] {
                    TileType::Floor => {
                        canvas.draw(&tiles.tile_map, DrawParam::default()
                        .dest(vec2(fx, yx))
                        .src(*tiles.get_tile(TileType::Floor)))
                    },
                    TileType::Wall => {
                        canvas.draw(&tiles.tile_map, DrawParam::default()
                        .dest(vec2(fx, yx))
                        .src(*tiles.get_tile(TileType::Wall)))
                    },
                    _ => ()
                }
            }
        }
    }
}