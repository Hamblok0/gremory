// TODO, put map's render function here, have camera follow character

use ggez::glam::*;
use ggez::Context;

use crate::tile::TileType;

const WINDOW_HEIGHT: i32 = 1080;
const WINDOW_WIDTH: i32 = 1920;
const NUM_X: usize = (WINDOW_WIDTH as usize) / 32;
const NUM_Y: usize = (WINDOW_HEIGHT as usize) / 32;
const NUM_TILES: usize = NUM_Y * NUM_X;

pub struct Camera {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32
}

impl Camera {
    pub fn new(ctx: &mut Context, player_position: Vec2) -> Self {
        Self {  
            left: player_position.x - WINDOW_WIDTH / 2,
            right: player_position.x + WINDOW_WIDTH / 2,
            top: player_position.y - WINDOW_HEIGHT / 2,
            bottom: player_position.y + WINDOW_HEIGHT / 2,
        }
    }

    pub fn on_player_move(&self) {
        self.left = player_position.x - WINDOW_WIDTH / 2;
        self.right = player_position.x + WINDOW_WIDTH / 2;
        self.top = player_position.y - WINDOW_HEIGHT / 2;
        self.bottom = player_position.y + WINDOW_HEIGHT / 2;
    }
}