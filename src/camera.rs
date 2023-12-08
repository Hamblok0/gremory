use ggez::glam::*;
use ggez::graphics::{Canvas, DrawParam};
use ggez::Context;

use crate::map::*;
use crate::prelude::*;
use crate::tile::*;
use crate::colors::Colors;

const FWIDTH: i32 = WINDOW_WIDTH as i32 / 2;
const FHEIGHT: i32 = WINDOW_HEIGHT as i32 / 2;

#[derive(Debug)]
pub struct Camera {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    tiles: Tile,
}

impl Camera {
    pub fn new(ctx: &mut Context, player_pos: Vec2) -> Self {
        let tiles = Tile::new(ctx);
        Self {
            left: player_pos.x as i32 - FWIDTH,
            right: player_pos.x as i32 + FWIDTH,
            top: player_pos.y as i32 - FHEIGHT,
            bottom: player_pos.y as i32 + FHEIGHT,
            tiles,
        }
    }

    pub fn on_player_move(&mut self, player_pos: Vec2) {
        self.left = player_pos.x as i32 - FWIDTH;
        self.right = player_pos.x as i32 + FWIDTH;
        self.top = player_pos.y as i32 - FHEIGHT;
        self.bottom = player_pos.y as i32 + FHEIGHT;
    }

    pub fn render(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        map: &Vec<TileType>,
        player: &Vec2,
        colors: &Colors, 
    ) {
        let fplayer = vec2(player.x * 32., player.y * 32.);
        print!("{}, {}, {}, {}", self.top, self.bottom, self.left, self.right); 
        for y in (self.top..self.bottom).step_by(32) {
            for x in (self.left..self.right).step_by(32) {
                let check_idx = idx(x as usize, y as usize);
                let fx = x as f32;
                let fy = y as f32;

                if check_idx.is_some() {
                    let idx = check_idx.unwrap();

                    match map[idx] {
                        TileType::Floor => canvas.draw(
                            &self.tiles.tile_map,
                            DrawParam::default()
                                .dest(vec2(fx, fy))
                                .z(0)
                                .color(colors.dark_grey)
                                .src(*self.tiles.get_tile(TileType::Floor)),
                        ),
                        TileType::Wall => canvas.draw(
                            &self.tiles.tile_map,
                            DrawParam::default()
                                .dest(vec2(fx, fy))
                                .z(0)
                                .color(colors.grey)
                                .src(*self.tiles.get_tile(TileType::Wall)),
                        ),
                        _ => (),
                    }
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
