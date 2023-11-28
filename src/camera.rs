use ggez::glam::*;
use ggez::graphics::{Canvas, Color, DrawParam};
use ggez::Context;

use crate::map::*;
use crate::prelude::*;
use crate::tile::*;
use crate::colors::Colors;

const FWIDTH: f32 = WINDOW_WIDTH / 2.;
const FHEIGHT: f32 = WINDOW_HEIGHT / 2.;

pub struct Camera {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    tiles: Tile,
}

impl Camera {
    pub fn new(ctx: &mut Context, player_pos: Vec2) -> Self {
        let tiles = Tile::new(ctx);
        Self {
            left: player_pos.x - FWIDTH,
            right: player_pos.x + FWIDTH,
            top: player_pos.y - FHEIGHT,
            bottom: player_pos.y + FHEIGHT,
            tiles,
        }
    }

    pub fn on_player_move(&mut self, player_pos: Vec2) {
        self.left = player_pos.x - FWIDTH;
        self.right = player_pos.x + FWIDTH;
        self.top = player_pos.y - FHEIGHT;
        self.bottom = player_pos.y + FHEIGHT;
    }

    pub fn render(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        map: &Vec<TileType>,
        player: Vec2,
        colors: &Colors, 
    ) {
        let fplayer = vec2(player.x * 32., player.y * 32.);
        
        for y in 0..NUM_Y {
            for x in 0..NUM_X {
                let idx = idx(x, y);
                let fx = (x * 32) as f32;
                let yx = (y * 32) as f32;
                match map[idx] {
                    TileType::Floor => canvas.draw(
                        &self.tiles.tile_map,
                        DrawParam::default()
                            .dest(vec2(fx, yx))
                            .z(0)
                            .color(colors.dark_grey)
                            .src(*self.tiles.get_tile(TileType::Floor)),
                    ),
                    TileType::Wall => canvas.draw(
                        &self.tiles.tile_map,
                        DrawParam::default()
                            .dest(vec2(fx, yx))
                            .z(0)
                            .color(colors.grey)
                            .src(*self.tiles.get_tile(TileType::Wall)),
                    ),
                    _ => (),
                }
            }
        }

        canvas.draw(
            &self.tiles.tile_map,
            DrawParam::default()
                .z(1)
                .dest(fplayer)
                .color(colors.player_color)
                .src(*self.tiles.get_tile(TileType::Player)),
        )
    }
}
